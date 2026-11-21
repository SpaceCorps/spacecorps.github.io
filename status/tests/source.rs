//! The source: the real list shape, a node that is updating, garbage, and a fetch from a local server.

use spacecorps_status::sampler::sample_at;
use spacecorps_status::source::{self, is_up};
use spacecorps_status::store::{Range, Store};

/// `GET https://play.spacecorps.xyz/api/servers`, read 2026-11-21 (0.5.10), shortened to the fields we use plus a few we ignore.
const LIVE: &str = r#"{"protocol":1,"season":"2026-11-06T00:05:00.510121478+00:00","servers":[
{"id":"home","name":"Helsinki","region":"eu","kind":"home","url":"https://play.spacecorps.xyz","status":"up","joinable":true,"reason":null,"online":32,"flying":15,"capacity":3000,"version":"0.5.10","protocol":1,"countdown":null},
{"id":"singapore-1","name":"Singapore","region":"ap","kind":"regional","url":"https://spacecorps-sg.sliplane.app","altUrl":"https://play-sg.spacecorps.xyz","status":"up","joinable":true,"reason":null,"online":0,"flying":0,"capacity":60,"version":"0.5.10","protocol":1,"countdown":null},
{"id":"us-east-1","name":"US East","region":"us","kind":"regional","url":"https://spacecorps-us.sliplane.app","status":"updating","joinable":false,"reason":"NODE_UPDATING","online":0,"flying":0,"capacity":60,"version":"0.5.9","protocol":1,"countdown":null}],"recall":true}"#;

#[test]
fn the_live_list_parses() {
    let s = source::parse(LIVE, 1_800_000_000).unwrap();
    let rows: Vec<_> = s
        .servers
        .iter()
        .map(|x| (x.id.as_str(), x.name.as_str(), x.up, x.online, x.flying))
        .collect();
    assert_eq!(
        rows,
        [
            ("home", "Helsinki", true, 32, 15),
            ("singapore-1", "Singapore", true, 0, 0),
            ("us-east-1", "US East", false, 0, 0)
        ]
    );
}

#[test]
fn up_and_full_serve_the_rest_do_not() {
    assert!(is_up("up") && is_up("full"));
    for s in ["updating", "maintenance", "down", "staff", ""] {
        assert!(!is_up(s), "{s}");
    }
}

#[test]
fn garbage_and_empty_lists_are_refused() {
    assert!(source::parse("<html>502 Bad Gateway</html>", 0).is_err());
    assert!(source::parse(r#"{"servers":[]}"#, 0).is_err());
    assert!(
        source::parse(r#"{"servers":[{"id":"home"}]}"#, 0).is_err(),
        "a server without a status is not a list entry"
    );
}

#[test]
fn a_look_over_http_is_recorded_and_a_dead_source_is_a_hole() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let url = format!(
        "http://{}/api/servers",
        server.server_addr().to_ip().unwrap()
    );
    let handle = std::thread::spawn(move || {
        // answer the first request with the list, the second with a 503
        let r = server.recv().unwrap();
        r.respond(tiny_http::Response::from_string(LIVE)).unwrap();
        let r = server.recv().unwrap();
        r.respond(tiny_http::Response::from_string("down").with_status_code(503))
            .unwrap();
    });
    let agent = source::agent();
    let mut store = Store::open_memory().unwrap();
    let t = 1_800_000_000;
    let line = sample_at(&mut store, &agent, &url, t);
    assert!(
        line.contains("home=32") && line.contains("us-east-1=0 DOWN"),
        "{line}"
    );
    let line = sample_at(&mut store, &agent, &url, t + 60);
    assert!(line.contains("source unreadable"), "{line}");
    handle.join().unwrap();
    let s = store.series(Range::Day, t + 120).unwrap();
    assert_eq!(s.series["home"].len(), 2);
    assert_eq!(s.series["home"][0].1, 32.0);
    assert_eq!(
        s.series["home"][1].3, 0.0,
        "Home counts as down while it cannot be read"
    );
}
