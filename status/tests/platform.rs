//! The split of the pilots by client (steam, desktop, ios, android, web, unknown): the parser, the
//! table, the view by platform, and the one thing that must never happen: a missing split read as zero.

use spacecorps_status::api::Api;
use spacecorps_status::source;
use spacecorps_status::store::{PLATFORMS, Platforms, Range, Sample, ServerSample, Store};

const T0: i64 = 1_800_000_000 - 1_800_000_000 % 86_400; // a UTC midnight

/// The shape the game announced for 0.5.11: `onlineByPlatform` with all six keys on every row, summing
/// to `online`; US East is still on an older version and has no such key.
const GAME: &str = r#"{"protocol":1,"servers":[
{"id":"home","name":"Helsinki","region":"eu","kind":"home","status":"up","online":32,"flying":15,"onlineByPlatform":{"steam":20,"desktop":9,"ios":0,"android":1,"web":0,"unknown":2}},
{"id":"singapore-1","name":"Singapore","region":"ap","kind":"regional","status":"up","online":3,"flying":1,"onlineByPlatform":{"steam":1,"desktop":2,"ios":0,"android":0,"web":0,"unknown":0}},
{"id":"us-east-1","name":"US East","region":"us","kind":"regional","status":"up","online":1,"flying":0}]}"#;

fn server(id: &str, online: i64, platforms: Option<Platforms>) -> ServerSample {
    ServerSample {
        id: id.into(),
        name: id.into(),
        region: "eu".into(),
        kind: if id == "home" { "home" } else { "regional" }.into(),
        status: "up".into(),
        up: true,
        online,
        flying: 0,
        platforms,
    }
}

/// Home alone, with or without the split.
fn home(ts: i64, online: i64, platforms: Option<Platforms>) -> Sample {
    Sample {
        ts,
        servers: vec![server("home", online, platforms)],
    }
}

fn steam_of(s: &spacecorps_status::store::PlatformSeries, server: &str) -> Vec<(i64, f64, i64)> {
    s.series[server]["steam"].clone()
}

#[test]
fn the_list_of_the_game_parses_and_a_server_without_the_key_has_no_split() {
    let s = source::parse(GAME, T0).unwrap();
    let split: Vec<_> = s.servers.iter().map(|x| x.platforms).collect();
    assert_eq!(
        split,
        [Some([20, 9, 0, 1, 0, 2]), Some([1, 2, 0, 0, 0, 0]), None]
    );
    assert_eq!(
        PLATFORMS,
        ["steam", "desktop", "ios", "android", "web", "unknown"]
    );
    // online and flying are read as before
    assert_eq!(s.servers[0].online, 32);
    assert_eq!(s.servers[0].flying, 15);
}

#[test]
fn odd_shapes_never_invent_a_split() {
    let row = |extra: &str| {
        let body = format!(r#"{{"servers":[{{"id":"home","status":"up","online":4{extra}}}]}}"#);
        source::parse(&body, T0).unwrap().servers[0].platforms
    };
    assert_eq!(row(""), None);
    assert_eq!(row(r#","onlineByPlatform":null"#), None);
    assert_eq!(row(r#","onlineByPlatform":{}"#), None);
    assert_eq!(row(r#","onlineByPlatform":[1,2,3]"#), None);
    assert_eq!(row(r#","onlineByPlatform":"steam""#), None);
    assert_eq!(
        row(r#","onlineByPlatform":{"cloud":4}"#),
        None,
        "only keys we do not know"
    );
    // a known client that is missing, negative or not a number counts 0; an unknown key is ignored
    assert_eq!(
        row(r#","onlineByPlatform":{"steam":3,"desktop":"x","ios":-2,"switch":9}"#),
        Some([3, 0, 0, 0, 0, 0])
    );
}

#[test]
fn a_sample_with_the_split_is_stored_per_client_and_the_total_adds_up() {
    let mut store = Store::open_memory().unwrap();
    store
        .record(&source::parse(GAME, T0 + 30).unwrap())
        .unwrap();
    let s = store.series_by_platform(Range::Day, T0 + 120).unwrap();
    assert_eq!(s.by, "platform");
    assert_eq!(s.since, Some(T0));
    // Home has 2 pilots on unknown builds, so unknown is charted
    assert_eq!(
        s.platforms,
        ["steam", "desktop", "ios", "android", "unknown"]
    );
    // 32 + 3 + 1 = 36 online; US East has no split, so its pilot is "unknown" in the total only
    let total = &s.series["total"];
    assert_eq!(
        total["unknown"],
        [(T0, 3.0, 3)],
        "2 of Home and the pilot of US East"
    );
    assert_eq!(total["steam"], [(T0, 21.0, 21)]);
    assert_eq!(total["desktop"], [(T0, 11.0, 11)]);
    assert_eq!(total["android"], [(T0, 1.0, 1)]);
    assert_eq!(total["ios"], [(T0, 0.0, 0)]);
    assert_eq!(s.series["home"]["steam"], [(T0, 20.0, 20)]);
    assert!(
        !s.series.contains_key("us-east-1"),
        "a server that did not report has no series, not a zero one"
    );
    let now: Vec<_> = s
        .servers
        .iter()
        .map(|x| {
            (
                x.server.id.as_str(),
                x.platforms.as_ref().map(|p| p["steam"]),
            )
        })
        .collect();
    assert_eq!(
        now,
        [
            ("total", Some(21)),
            ("home", Some(20)),
            ("singapore-1", Some(1)),
            ("us-east-1", None)
        ]
    );
}

#[test]
fn unknown_and_web_are_charted_only_when_somebody_was_on_them() {
    let mut store = Store::open_memory().unwrap();
    store
        .record(&home(T0, 5, Some([3, 2, 0, 0, 0, 0])))
        .unwrap();
    let s = store.series_by_platform(Range::Day, T0 + 60).unwrap();
    assert_eq!(s.platforms, ["steam", "desktop", "ios", "android"]);
    assert!(!s.series["home"].contains_key("web"));

    store
        .record(&home(T0 + 60, 7, Some([3, 2, 0, 0, 1, 1])))
        .unwrap();
    let s = store.series_by_platform(Range::Day, T0 + 120).unwrap();
    assert_eq!(
        s.platforms,
        ["steam", "desktop", "ios", "android", "web", "unknown"]
    );
    // every drawn client has a value at every minute, so the lines are complete
    assert_eq!(s.series["home"]["web"], [(T0, 0.0, 0), (T0 + 60, 1.0, 1)]);
}

#[test]
fn history_before_the_split_is_untouched_and_the_view_starts_where_the_split_starts() {
    let mut store = Store::open_memory().unwrap();
    for i in 0..60 {
        store.record(&home(T0 + i * 60, 10, None)).unwrap();
    }
    let split_start = T0 + 3600;
    for i in 0..30 {
        store
            .record(&home(split_start + i * 60, 10, Some([6, 4, 0, 0, 0, 0])))
            .unwrap();
    }
    let now = split_start + 30 * 60;
    let plain = store.series(Range::Day, now).unwrap();
    assert_eq!(
        plain.series["home"].len(),
        90,
        "the old view keeps its 90 minutes"
    );
    assert_eq!(plain.platform_since, Some(split_start));
    assert_eq!(plain.since, Some(T0));

    let s = store.series_by_platform(Range::All, now).unwrap();
    assert_eq!(s.since, Some(split_start));
    assert_eq!(s.from, split_start);
    let steam = steam_of(&s, "home");
    assert_eq!(steam.len(), 30);
    assert_eq!(steam[0], (split_start, 6.0, 6));
    assert!(
        steam.iter().all(|p| p.0 >= split_start),
        "nothing before the split, and no zeros for the time before it"
    );
}

#[test]
fn a_bucket_that_straddles_the_start_averages_the_samples_that_had_the_split() {
    let mut store = Store::open_memory().unwrap();
    // an old sample with the split makes the window old enough that "day" reads 5-minute buckets
    store
        .record(&home(T0 - 6 * 86_400, 1, Some([1, 0, 0, 0, 0, 0])))
        .unwrap();
    // one 5-minute bucket: three samples without the split, then steam 10 and 20
    for i in 0..3 {
        store.record(&home(T0 + i * 60, 30, None)).unwrap();
    }
    store
        .record(&home(T0 + 180, 10, Some([10, 0, 0, 0, 0, 0])))
        .unwrap();
    store
        .record(&home(T0 + 240, 20, Some([20, 0, 0, 0, 0, 0])))
        .unwrap();
    let s = store.series_by_platform(Range::Day, T0 + 600).unwrap();
    assert_eq!(s.step, 300);
    assert_eq!(
        steam_of(&s, "home")[0],
        (T0, 15.0, 20),
        "mean of the two samples that reported (not of five with three zeros)"
    );
}

#[test]
fn a_missing_split_in_the_total_is_unknown_only_while_another_server_reports() {
    let mut store = Store::open_memory().unwrap();
    // nobody reports: the total has no split either
    let none = Sample {
        ts: T0,
        servers: vec![server("home", 5, None), server("singapore-1", 2, None)],
    };
    store.record(&none).unwrap();
    assert_eq!(store.platform_first_ts().unwrap(), None);
    // Home reports, Singapore (older game) does not: its 2 pilots are unknown in the total
    let some = Sample {
        ts: T0 + 60,
        servers: vec![
            server("home", 5, Some([5, 0, 0, 0, 0, 0])),
            server("singapore-1", 2, None),
        ],
    };
    store.record(&some).unwrap();
    assert_eq!(store.platform_first_ts().unwrap(), Some(T0 + 60));
    let s = store.series_by_platform(Range::Day, T0 + 120).unwrap();
    assert_eq!(s.series["total"]["steam"], [(T0 + 60, 5.0, 5)]);
    assert_eq!(s.platforms.last(), Some(&"unknown"));
    assert_eq!(s.series["total"]["unknown"], [(T0 + 60, 2.0, 2)]);
    assert!(!s.series.contains_key("singapore-1"));
}

#[test]
fn an_unreachable_game_leaves_no_split() {
    let mut store = Store::open_memory().unwrap();
    store
        .record(&home(T0, 4, Some([4, 0, 0, 0, 0, 0])))
        .unwrap();
    store.record(&source::unreachable(T0 + 60)).unwrap();
    let s = store.series_by_platform(Range::Day, T0 + 120).unwrap();
    assert_eq!(
        s.series["home"]["steam"].len(),
        1,
        "the hole is in the default view (Home down), not a zero here"
    );
}

#[test]
fn downtime_is_listed_so_a_line_at_zero_is_not_mistaken_for_nobody() {
    let mut store = Store::open_memory().unwrap();
    store
        .record(&home(T0, 5, Some([5, 0, 0, 0, 0, 0])))
        .unwrap();
    // one minute of an update: Home answers with zero pilots and is not up
    let mut update = home(T0 + 60, 0, Some([0; 6]));
    update.servers[0].status = "updating".into();
    update.servers[0].up = false;
    store.record(&update).unwrap();
    store
        .record(&home(T0 + 120, 5, Some([5, 0, 0, 0, 0, 0])))
        .unwrap();
    let s = store.series_by_platform(Range::Day, T0 + 180).unwrap();
    assert_eq!(s.step, 60);
    assert_eq!(s.series["home"]["steam"][1], (T0 + 60, 0.0, 0));
    assert_eq!(s.down["home"], [(T0 + 60, 0.0)]);
    assert_eq!(
        s.down["total"],
        [(T0 + 60, 0.0)],
        "the only server is down, so the total is"
    );
    let all_up = Store::open_memory().unwrap();
    assert!(
        all_up
            .series_by_platform(Range::Day, T0)
            .unwrap()
            .down
            .is_empty()
    );
}

#[test]
fn old_rows_leave_the_fine_tiers_and_the_hourly_split_stays() {
    let mut store = Store::open_memory().unwrap();
    let sample = Sample {
        ts: T0,
        servers: vec![server("home", 7, Some([7, 0, 0, 0, 0, 0]))],
    };
    store.record(&sample).unwrap();
    let later = T0 + 100 * 86_400;
    // two series (home, total), tiers 0 and 1, in both tables
    assert_eq!(store.prune(later).unwrap(), 2 * 2 * 2);
    let s = store.series_by_platform(Range::All, later).unwrap();
    assert_eq!(s.series["home"]["steam"], [(T0, 7.0, 7)]);
    assert_eq!(s.since, Some(T0), "the first split sample is remembered");
}

/// The recorder of 0.1 knew only `buckets`, `servers` and `meta`: a file written by it must open, keep
/// its history, and take the split from then on.
#[test]
fn a_database_from_before_the_split_is_upgraded_without_losing_history() {
    let path = std::env::temp_dir().join(format!("sc-status-migration-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE buckets (tier INTEGER NOT NULL, bucket INTEGER NOT NULL, server TEXT NOT NULL,
                n INTEGER NOT NULL, sum_online INTEGER NOT NULL, max_online INTEGER NOT NULL,
                sum_flying INTEGER NOT NULL, max_flying INTEGER NOT NULL, up_n INTEGER NOT NULL, status TEXT,
                PRIMARY KEY (tier, bucket, server)) WITHOUT ROWID;
             CREATE TABLE servers (id TEXT PRIMARY KEY, ord INTEGER NOT NULL, name TEXT NOT NULL,
                region TEXT NOT NULL, kind TEXT NOT NULL, ts INTEGER NOT NULL, status TEXT NOT NULL,
                up INTEGER NOT NULL, online INTEGER NOT NULL, flying INTEGER NOT NULL);
             CREATE TABLE meta (key TEXT PRIMARY KEY, value INTEGER NOT NULL);
             INSERT INTO buckets VALUES (0, 1799971200, 'home', 1, 7, 7, 3, 3, 1, 'up');
             INSERT INTO buckets VALUES (0, 1799971200, 'total', 1, 7, 7, 3, 3, 1, 'up');
             INSERT INTO servers VALUES ('home', 0, 'Helsinki', 'eu', 'home', 1799971200, 'up', 1, 7, 3);
             INSERT INTO servers VALUES ('total', -1, 'All servers', '', 'total', 1799971200, 'up', 1, 7, 3);
             INSERT INTO meta VALUES ('first_ts', 1799971200);",
        )
        .unwrap();
    }
    let t = 1_799_971_200;
    let mut store = Store::open(&path).unwrap();
    let old = store.series(Range::Day, t + 60).unwrap();
    assert_eq!(old.series["home"], [(t, 7.0, 7, 1.0)]);
    assert_eq!(old.since, Some(t));
    assert_eq!(old.platform_since, None, "the old rows have no split");
    let nothing = store.series_by_platform(Range::All, t + 60).unwrap();
    assert_eq!(nothing.since, None);
    assert!(
        nothing.series.is_empty(),
        "no split yet: no series, not zeros"
    );
    assert_eq!(nothing.platforms, ["steam", "desktop", "ios", "android"]);
    // from the first new sample on, the split is kept next to the old history
    store
        .record(&home(t + 3600, 9, Some([5, 4, 0, 0, 0, 0])))
        .unwrap();
    let s = store.series_by_platform(Range::All, t + 3660).unwrap();
    assert_eq!(s.since, Some(t + 3600));
    assert_eq!(steam_of(&s, "home"), [(t + 3600, 5.0, 5)]);
    assert_eq!(
        store.series(Range::Day, t + 3660).unwrap().series["home"].len(),
        2
    );
    drop(store);
    for ext in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{ext}", path.display()));
    }
}

#[test]
fn the_api_serves_the_split_next_to_the_plain_series() {
    let mut store = Store::open_memory().unwrap();
    store.record(&source::parse(GAME, T0).unwrap()).unwrap();
    let api = Api::default();
    let get = |url: &str| api.respond(&store, "GET", url, T0 + 60);

    let plain = get("/api/status/series?range=day");
    let by = get("/api/status/series?range=day&by=platform");
    assert_eq!((plain.status, by.status), (200, 200));
    assert_ne!(plain.body, by.body, "the two views are cached apart");
    let plain: serde_json::Value = serde_json::from_slice(&plain.body).unwrap();
    assert_eq!(
        plain["platform_since"], T0,
        "the page learns from the plain view that a split exists"
    );
    assert!(plain["series"]["home"][0].is_array());

    let by: serde_json::Value = serde_json::from_slice(&by.body).unwrap();
    assert_eq!(by["by"], "platform");
    assert_eq!(by["since"], T0);
    assert_eq!(
        by["platforms"],
        serde_json::json!(["steam", "desktop", "ios", "android", "unknown"])
    );
    assert_eq!(by["servers"][0]["id"], "total");
    assert_eq!(by["servers"][0]["platforms"]["steam"], 21);
    assert_eq!(by["servers"][3]["id"], "us-east-1");
    assert!(by["servers"][3]["platforms"].is_null());
    assert_eq!(
        by["series"]["total"]["steam"],
        serde_json::json!([[T0, 21.0, 21]])
    );

    assert_eq!(get("/api/status/series?range=day&by=client").status, 400);
    assert_eq!(get("/api/status/series?range=day&by=").status, 200);
    assert_eq!(get("/api/status/series?range=nope&by=platform").status, 400);
}
