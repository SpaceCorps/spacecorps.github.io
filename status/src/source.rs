//! The source of the numbers: the game's public server list, `GET /api/servers` on Home.
//!
//! It is the list the game client shows before login: unauthenticated, CORS `*`, built from memory on
//! the game side (no database), cached for a second there. One request gives every server: its
//! status, the pilots connected (`online`, menus included) and the pilots flying. A pilot flying on
//! Singapore or US East is connected to that node only (Home's station hub is closed while he
//! flies), so the sum over the three servers counts nobody twice.
//!
//! From game 0.5.11 every row also has `onlineByPlatform`: the pilots by client, six numbers that add
//! up to `online`. A server on an older version has no such key; that is recorded as "not reported",
//! never as zero and never as `unknown`.

use crate::store::{PLATFORMS, Platforms, Sample, ServerSample};
use serde::Deserialize;
use std::time::Duration;

pub const DEFAULT_URL: &str = "https://play.spacecorps.xyz/api/servers";
const AGENT: &str = "spacecorps-status/0.1 (+https://spacecorps.tech/status.html)";

#[derive(Deserialize)]
struct List {
    servers: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    region: String,
    #[serde(default)]
    kind: String,
    status: String,
    #[serde(default)]
    online: i64,
    #[serde(default)]
    flying: i64,
    #[serde(default, rename = "onlineByPlatform")]
    by_platform: Option<serde_json::Value>,
}

/// The split by client of a row. `None` when the key is missing or is not an object with at least one
/// of the known clients; a known client that is missing or not a number counts 0 (the game sends all
/// six), an unknown key is ignored.
fn platforms(value: Option<&serde_json::Value>) -> Option<Platforms> {
    let map = value?.as_object()?;
    if !PLATFORMS.iter().any(|p| map.contains_key(*p)) {
        return None;
    }
    let mut out = [0; PLATFORMS.len()];
    for (slot, name) in out.iter_mut().zip(PLATFORMS) {
        *slot = map
            .get(name)
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0)
            .max(0);
    }
    Some(out)
}

/// A server is up while it is serving, also when it is full; updating, maintenance and down count as down.
pub fn is_up(status: &str) -> bool {
    matches!(status, "up" | "full")
}

/// Reads the list; `ts` is the time of the look.
pub fn parse(body: &str, ts: i64) -> Result<Sample, String> {
    let list: List = serde_json::from_str(body).map_err(|e| format!("not a server list: {e}"))?;
    if list.servers.is_empty() {
        return Err("the server list is empty".into());
    }
    let servers = list
        .servers
        .into_iter()
        .map(|e| ServerSample {
            up: is_up(&e.status),
            online: e.online.max(0),
            flying: e.flying.max(0),
            platforms: platforms(e.by_platform.as_ref()),
            id: e.id,
            name: e.name,
            region: e.region,
            kind: e.kind,
            status: e.status,
        })
        .collect();
    Ok(Sample { ts, servers })
}

/// What a look records when Home could not be read: Home is down, and the nodes are unknown (they
/// are only known through Home), so they get no row.
pub fn unreachable(ts: i64) -> Sample {
    Sample {
        ts,
        servers: vec![ServerSample {
            id: "home".into(),
            name: String::new(),
            region: String::new(),
            kind: "home".into(),
            status: "down".into(),
            up: false,
            online: 0,
            flying: 0,
            platforms: None,
        }],
    }
}

pub fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(10)))
        .user_agent(AGENT)
        .build()
        .into()
}

/// One look at the server list.
pub fn fetch(agent: &ureq::Agent, url: &str, ts: i64) -> Result<Sample, String> {
    let mut res = agent.get(url).call().map_err(|e| format!("{url}: {e}"))?;
    let body = res
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("{url}: {e}"))?;
    parse(&body, ts)
}
