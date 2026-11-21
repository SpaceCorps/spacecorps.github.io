//! The read API: `GET /api/status/series?range=day|week|month|quarter|year|all`.
//!
//! Public and read-only: player counts that the game already shows on its login screen. Answers are
//! kept for 30 seconds in memory, so a crowd on the page costs one query per range per half minute.

use crate::store::{Range, Store};
use std::collections::HashMap;
use std::sync::Mutex;

/// How long a built answer is served again.
const KEEP_SECS: i64 = 30;

pub struct Reply {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
    /// `Cache-Control: public, max-age=` (0: no-store).
    pub max_age: i64,
}

impl Reply {
    fn json(status: u16, body: String, max_age: i64) -> Reply {
        Reply {
            status,
            content_type: "application/json",
            body: body.into_bytes(),
            max_age,
        }
    }
    fn error(status: u16, message: &str) -> Reply {
        Reply::json(
            status,
            serde_json::json!({ "error": message }).to_string(),
            0,
        )
    }
}

#[derive(Default)]
pub struct Api {
    cache: Mutex<HashMap<Range, (i64, Vec<u8>)>>,
}

/// The value of `name` in a query string.
fn query_value<'a>(query: &'a str, name: &str) -> Option<&'a str> {
    query
        .split('&')
        .filter_map(|p| p.split_once('='))
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v)
}

impl Api {
    /// Answers one request. `url` is the path with its query.
    pub fn respond(&self, store: &Store, method: &str, url: &str, now: i64) -> Reply {
        let (path, query) = url.split_once('?').unwrap_or((url, ""));
        match (method, path) {
            ("GET" | "HEAD", "/healthz") => Reply {
                status: 200,
                content_type: "text/plain",
                body: b"ok".to_vec(),
                max_age: 0,
            },
            ("GET" | "HEAD", "/api/status/series") => {
                let Some(range) = Range::parse(query_value(query, "range").unwrap_or("day")) else {
                    return Reply::error(
                        400,
                        "range must be day, week, month, quarter, year or all",
                    );
                };
                if let Some((at, body)) = self
                    .cache
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .get(&range)
                    && now - at < KEEP_SECS
                {
                    return Reply {
                        status: 200,
                        content_type: "application/json",
                        body: body.clone(),
                        max_age: KEEP_SECS,
                    };
                }
                match store.series(range, now) {
                    Ok(series) => {
                        let body = serde_json::to_vec(&series).unwrap_or_default();
                        self.cache
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .insert(range, (now, body.clone()));
                        Reply {
                            status: 200,
                            content_type: "application/json",
                            body,
                            max_age: KEEP_SECS,
                        }
                    }
                    Err(e) => {
                        eprintln!("series {range:?}: {e}");
                        Reply::error(500, "the history could not be read")
                    }
                }
            }
            (_, "/healthz" | "/api/status/series") => Reply::error(405, "GET only"),
            _ => Reply::error(404, "not found"),
        }
    }
}
