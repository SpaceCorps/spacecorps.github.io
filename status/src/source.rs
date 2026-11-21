//! The source of the numbers: the game's public server list, `GET /api/servers` on Home.
//!
//! It is the list the game client shows before login: unauthenticated, CORS `*`, built from memory on
//! the game side (no database), cached for a second there. One request gives every server: its
//! status, the pilots connected (`online`, menus included) and the pilots flying. A pilot flying on
//! Singapore or US East is connected to that node only (Home's station hub is closed while he
//! flies), so the sum over the three servers counts nobody twice.

use crate::store::{Sample, ServerSample};
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
