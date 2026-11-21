//! Writes SYNTHETIC history into a database, for looking at the chart in all six ranges before real
//! history exists: `cargo run --example seed_demo -- demo.db 400`. Never use the file in production; run the
//! service on it with `STATUS_RECORD=off` so nothing real is mixed in.

use spacecorps_status::sampler::unix_now;
use spacecorps_status::store::{Sample, ServerSample, Store};

fn server(id: &str, name: &str, region: &str, status: &str, online: i64) -> ServerSample {
    ServerSample {
        id: id.into(),
        name: name.into(),
        region: region.into(),
        kind: if id == "home" { "home" } else { "regional" }.into(),
        status: status.into(),
        up: status == "up",
        online,
        flying: online / 2,
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: seed_demo <db> [days]");
    let days: i64 = args.next().and_then(|d| d.parse().ok()).unwrap_or(400);
    let mut store = Store::open(&path).expect("open");
    let now = unix_now() - unix_now() % 60;
    let start = now - days * 86_400;
    let mut seed: u64 = 7;
    let mut noise = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((seed >> 33) as f64 / (1u64 << 31) as f64) - 0.5
    };
    // every 5th minute, the last two days every minute
    let mut t = start;
    while t <= now {
        let hour = ((t % 86_400) / 3600) as f64;
        let weekday = ((t / 86_400 + 4) % 7) as f64; // 0 = Monday
        let growth = 0.4 + 0.9 * ((t - start) as f64 / (now - start) as f64);
        let evening = (-((hour - 19.0) / 3.5).powi(2)).exp();
        let weekend = if weekday >= 5.0 { 1.35 } else { 1.0 };
        let home = ((4.0 + 30.0 * evening * weekend * growth + 6.0 * noise()).max(0.0)) as i64;
        let sg = ((0.5 + 5.0 * (-((hour - 12.0) / 3.0).powi(2)).exp() * growth + 1.5 * noise())
            .max(0.0)) as i64;
        let us = ((0.5 + 7.0 * (-((hour - 2.0) / 3.0).powi(2)).exp() * growth + 1.5 * noise())
            .max(0.0)) as i64;
        let day = (t - start) / 86_400;
        let updating = day % 9 == 4 && (t % 86_400) / 60 < 6; // six minutes of deploy every 9th day
        let sg_down = day == days / 2 && (t % 86_400) / 60 < 40; // one longer outage
        let home_state = if updating { "updating" } else { "up" };
        let sample = Sample {
            ts: t,
            servers: vec![
                server(
                    "home",
                    "Helsinki",
                    "eu",
                    home_state,
                    if updating { 0 } else { home },
                ),
                server(
                    "singapore-1",
                    "Singapore",
                    "ap",
                    if sg_down { "down" } else { "up" },
                    if sg_down { 0 } else { sg },
                ),
                server("us-east-1", "US East", "us", "up", us),
            ],
        };
        store.record(&sample).expect("record");
        t += if t > now - 2 * 86_400 { 60 } else { 300 };
    }
    println!("seeded {days} days of SYNTHETIC history into {path}");
}
