//! The sampler: once a minute, on the minute, one look at the server list, recorded.

use crate::source;
use crate::store::{Sample, Store};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// One look: fetch, record, and the line for the log. A source that cannot be read is recorded as
/// Home down (see [`source::unreachable`]): the history must show the hole, not skip it.
pub fn sample_once(store: &mut Store, agent: &ureq::Agent, url: &str) -> String {
    sample_at(store, agent, url, unix_now())
}

/// [`sample_once`] with the time of the look given.
pub fn sample_at(store: &mut Store, agent: &ureq::Agent, url: &str, ts: i64) -> String {
    let (sample, problem) = match source::fetch(agent, url, ts) {
        Ok(sample) => (sample, None),
        Err(why) => (source::unreachable(ts), Some(why)),
    };
    let line = describe(&sample);
    match store.record(&sample) {
        Ok(true) => {}
        Ok(false) => return format!("{ts} already recorded this minute"),
        Err(e) => return format!("{ts} could not record: {e}"),
    }
    match problem {
        Some(why) => format!("{line} (source unreadable: {why})"),
        None => line,
    }
}

fn describe(sample: &Sample) -> String {
    let servers: Vec<String> = sample
        .servers
        .iter()
        .map(|s| format!("{}={}{}", s.id, s.online, if s.up { "" } else { " DOWN" }))
        .collect();
    format!("{} {}", sample.ts, servers.join(" "))
}

/// Seconds to the next full minute.
pub fn until_next_minute(now: i64) -> Duration {
    Duration::from_secs((60 - now.rem_euclid(60)) as u64)
}

/// Runs for ever: sample on the minute, prune about every hour.
pub fn run(mut store: Store, url: String) -> ! {
    let agent = source::agent();
    let mut looks: u64 = 0;
    loop {
        println!("{}", sample_once(&mut store, &agent, &url));
        looks += 1;
        if looks % 60 == 1 {
            match store.prune(unix_now()) {
                Ok(0) => {}
                Ok(n) => println!("{} pruned {n} old rows", unix_now()),
                Err(e) => println!("{} prune failed: {e}", unix_now()),
            }
        }
        std::thread::sleep(until_next_minute(unix_now()));
    }
}
