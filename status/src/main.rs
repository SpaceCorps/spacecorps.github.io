//! The status service: samples the game's server list every minute into SQLite and serves the series.
//!
//! Environment: `STATUS_DB` (default `/data/status.db`), `STATUS_ADDR` (default `0.0.0.0:8090`),
//! `STATUS_SOURCE_URL` (default the game's public `/api/servers`), `STATUS_RECORD=off` (serve a database, sample nothing).

use spacecorps_status::api::Api;
use spacecorps_status::sampler::{self, unix_now};
use spacecorps_status::source;
use spacecorps_status::store::Store;
use std::sync::Arc;
use tiny_http::{Header, Method, Response, Server};

fn env(name: &str, default: &str) -> String {
    std::env::var(name)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("a valid header")
}

/// How many threads answer requests.
const WORKERS: usize = 4;

fn main() {
    let db = env("STATUS_DB", "/data/status.db");
    let addr = env("STATUS_ADDR", "0.0.0.0:8090");
    let url = env("STATUS_SOURCE_URL", source::DEFAULT_URL);

    // The container is PID 1: without a handler SIGTERM would be ignored and a deploy would wait for the kill.
    ctrlc::set_handler(|| {
        println!("{} stopping", unix_now());
        std::process::exit(0);
    })
    .expect("signal handler");

    let writer = Store::open(&db).unwrap_or_else(|e| panic!("cannot open {db}: {e}"));
    println!(
        "{} recording {url} into {db}, serving http://{addr}",
        unix_now()
    );
    if env("STATUS_RECORD", "on") == "off" {
        println!(
            "{} STATUS_RECORD=off: serving only, nothing is sampled",
            unix_now()
        );
    } else {
        std::thread::spawn({
            let url = url.clone();
            move || sampler::run(writer, url)
        });
    }

    let server =
        Arc::new(Server::http(&addr).unwrap_or_else(|e| panic!("cannot listen on {addr}: {e}")));
    let api = Arc::new(Api::default());
    let workers: Vec<_> = (0..WORKERS)
        .map(|_| {
            let (server, api, db) = (server.clone(), api.clone(), db.clone());
            std::thread::spawn(move || {
                let store =
                    Store::open_read(&db).unwrap_or_else(|e| panic!("cannot read {db}: {e}"));
                for request in server.incoming_requests() {
                    let method = match request.method() {
                        Method::Get => "GET",
                        Method::Head => "HEAD",
                        _ => "OTHER",
                    };
                    let reply = api.respond(&store, method, request.url(), unix_now());
                    let cache = if reply.max_age > 0 {
                        format!("public, max-age={}", reply.max_age)
                    } else {
                        "no-store".to_string()
                    };
                    let mut response = Response::from_data(if method == "HEAD" {
                        Vec::new()
                    } else {
                        reply.body
                    })
                    .with_status_code(reply.status)
                    .with_header(header("Content-Type", reply.content_type))
                    .with_header(header("Cache-Control", &cache))
                    .with_header(header("Access-Control-Allow-Origin", "*"))
                    .with_header(header("X-Content-Type-Options", "nosniff"));
                    if method == "HEAD" {
                        response = response.with_header(header("Content-Length", "0"));
                    }
                    let _ = request.respond(response);
                }
            })
        })
        .collect();
    for w in workers {
        let _ = w.join();
    }
}
