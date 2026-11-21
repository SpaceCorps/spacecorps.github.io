//! The recorder end to end without the network: samples in, series out, per range.

use spacecorps_status::api::Api;
use spacecorps_status::store::{MAX_POINTS, Plan, Range, Sample, ServerSample, Store, TOTAL, plan};

const T0: i64 = 1_800_000_000 - 1_800_000_000 % 86_400; // a UTC midnight

fn server(id: &str, name: &str, status: &str, online: i64, flying: i64) -> ServerSample {
    ServerSample {
        id: id.into(),
        name: name.into(),
        region: "eu".into(),
        kind: if id == "home" { "home" } else { "regional" }.into(),
        status: status.into(),
        up: status == "up" || status == "full",
        online,
        flying,
    }
}

fn sample(ts: i64, home: i64, sg: i64, us: i64) -> Sample {
    Sample {
        ts,
        servers: vec![
            server("home", "Helsinki", "up", home, 0),
            server("singapore-1", "Singapore", "up", sg, 0),
            server("us-east-1", "US East", "up", us, 0),
        ],
    }
}

#[test]
fn a_sample_gives_each_server_and_a_total() {
    let mut store = Store::open_memory().unwrap();
    assert!(store.record(&sample(T0 + 30, 32, 2, 1)).unwrap());
    let s = store.series(Range::Day, T0 + 60).unwrap();
    let online: Vec<(&str, i64)> = s
        .servers
        .iter()
        .map(|x| (x.id.as_str(), x.online))
        .collect();
    assert_eq!(
        online,
        [
            (TOTAL, 35),
            ("home", 32),
            ("singapore-1", 2),
            ("us-east-1", 1)
        ]
    );
    assert_eq!(s.since, Some(T0));
    assert_eq!(s.series["total"].len(), 1);
    assert_eq!(s.series["total"][0], (T0, 35.0, 35, 1.0));
}

#[test]
fn the_same_minute_is_recorded_once() {
    let mut store = Store::open_memory().unwrap();
    assert!(store.record(&sample(T0 + 5, 10, 0, 0)).unwrap());
    assert!(
        !store.record(&sample(T0 + 50, 99, 0, 0)).unwrap(),
        "a restart inside the minute must not count twice"
    );
    let s = store.series(Range::Day, T0 + 60).unwrap();
    assert_eq!(s.series["home"][0].1, 10.0);
}

#[test]
fn a_five_minute_bucket_holds_the_mean_the_peak_and_the_uptime() {
    let mut store = Store::open_memory().unwrap();
    // an old first sample makes the install old enough that "day" reads the 5-minute tier
    store.record(&sample(T0 - 6 * 86_400, 1, 0, 0)).unwrap();
    // one 5-minute bucket: 10, 20, 30, 40 online, then one minute with Home updating
    for (i, online) in [10, 20, 30, 40].into_iter().enumerate() {
        store
            .record(&sample(T0 + i as i64 * 60, online, 0, 0))
            .unwrap();
    }
    let mut down = sample(T0 + 240, 0, 0, 0);
    down.servers[0] = server("home", "Helsinki", "updating", 0, 0);
    store.record(&down).unwrap();
    let s = store.series(Range::Day, T0 + 600).unwrap();
    assert_eq!(s.step, 300);
    assert_eq!(
        s.series["home"],
        [(T0, 20.0, 40, 0.8)],
        "mean of 10,20,30,40,0; peak 40; 4 of 5 samples up"
    );
    // a down sample inside a coarse bucket always shows: the share up is below 1 only for that bucket
    let mut many = Store::open_memory().unwrap();
    for i in 0..4320 {
        let mut x = sample(T0 + i * 600, 5, 0, 0);
        if i == 700 {
            x.servers[0] = server("home", "Helsinki", "updating", 0, 0);
        }
        many.record(&x).unwrap();
    }
    let s = many.series(Range::All, T0 + 4320 * 600).unwrap();
    let below: Vec<f64> = s.series["home"]
        .iter()
        .map(|p| p.3)
        .filter(|u| *u < 1.0)
        .collect();
    assert_eq!(below.len(), 1, "exactly one bucket holds the down sample");
    assert!(below[0] > 0.9 && below[0] < 1.0, "{}", below[0]);
}

#[test]
fn coarse_ranges_roll_minutes_up_by_mean_and_max() {
    let mut store = Store::open_memory().unwrap();
    // 3 days of samples every minute would be slow in a test: every 10th minute is enough for the tiers
    let mut t = T0;
    while t < T0 + 3 * 86_400 {
        let online = if (t / 3600) % 24 == 12 { 100 } else { 10 };
        store.record(&sample(t, online, 0, 0)).unwrap();
        t += 600;
    }
    let now = T0 + 3 * 86_400;
    let s = store.series(Range::All, now).unwrap();
    // three days at the finest step with at most 400 points: 3 * 86400 / 1800 = 144 (step 1800 is the first under 400)
    assert_eq!(s.step, 1_800);
    let peak = s.series["home"].iter().map(|p| p.2).max().unwrap();
    assert_eq!(peak, 100);
    let mean_midday = s.series["home"]
        .iter()
        .find(|p| (p.0 / 3600) % 24 == 12)
        .unwrap()
        .1;
    assert_eq!(mean_midday, 100.0);
    let mean_night = s.series["home"]
        .iter()
        .find(|p| (p.0 / 3600) % 24 == 3)
        .unwrap()
        .1;
    assert_eq!(mean_night, 10.0);
}

#[test]
fn an_unreachable_source_is_a_hole_in_home_and_nothing_for_the_nodes() {
    let mut store = Store::open_memory().unwrap();
    store.record(&sample(T0, 5, 1, 1)).unwrap();
    store
        .record(&spacecorps_status::source::unreachable(T0 + 60))
        .unwrap();
    let s = store.series(Range::Day, T0 + 120).unwrap();
    assert_eq!(s.series["home"].len(), 2);
    assert_eq!(s.series["home"][1], (T0 + 60, 0.0, 0, 0.0));
    assert_eq!(
        s.series["singapore-1"].len(),
        1,
        "the nodes are only known through Home"
    );
    assert_eq!(s.series["total"][1].3, 0.0);
    let home = s.servers.iter().find(|x| x.id == "home").unwrap();
    assert_eq!(
        home.name, "Helsinki",
        "the name stays when Home could not be read"
    );
    assert_eq!(home.status, "down");
}

#[test]
fn old_rows_leave_the_fine_tiers_and_the_hourly_tier_stays() {
    let mut store = Store::open_memory().unwrap();
    store.record(&sample(T0, 7, 0, 0)).unwrap();
    // 100 days later the minute and 5-minute rows are gone, the hour is kept
    let later = T0 + 100 * 86_400;
    let removed = store.prune(later).unwrap();
    assert_eq!(removed, 4 * 2, "four series, tiers 0 and 1");
    let s = store.series(Range::All, later).unwrap();
    assert_eq!(s.step, 21_600);
    assert_eq!(s.series["home"].len(), 1);
    assert_eq!(s.series["home"][0].1, 7.0);
    assert_eq!(s.since, Some(T0), "the first sample is remembered");
}

#[test]
fn the_plan_picks_the_finest_step_under_the_point_limit() {
    let now = T0 + 1_000 * 86_400;
    let full = |range| plan(range, now, Some(T0));
    assert_eq!(
        full(Range::Day),
        Plan {
            from: now - 86_400,
            tier: 1,
            step: 300
        }
    );
    assert_eq!(full(Range::Week).step, 1_800);
    assert_eq!(full(Range::Month).step, 7_200);
    assert_eq!(full(Range::Quarter).step, 21_600);
    assert_eq!(full(Range::Year).step, 86_400);
    assert_eq!(full(Range::All).step, 604_800);
    for r in [
        Range::Day,
        Range::Week,
        Range::Month,
        Range::Quarter,
        Range::Year,
        Range::All,
    ] {
        let p = full(r);
        assert!(
            (now - p.from) / p.step <= MAX_POINTS + 1,
            "{r:?} would have too many points"
        );
        assert!(p.tier <= 2);
    }
    // a young install: the window starts at the first sample, and the steps are fine
    let young = plan(Range::Year, T0 + 2 * 3600, Some(T0));
    assert_eq!((young.from, young.tier, young.step), (T0, 0, 60));
    // nothing recorded yet: an empty window
    assert_eq!(plan(Range::All, T0, None).from, T0);
}

#[test]
fn the_api_answers_each_range_and_refuses_the_rest() {
    let mut store = Store::open_memory().unwrap();
    store.record(&sample(T0, 3, 0, 0)).unwrap();
    let api = Api::default();
    for range in ["day", "week", "month", "quarter", "year", "all"] {
        let reply = api.respond(
            &store,
            "GET",
            &format!("/api/status/series?range={range}"),
            T0 + 60,
        );
        assert_eq!(reply.status, 200, "{range}");
        let json: serde_json::Value = serde_json::from_slice(&reply.body).unwrap();
        assert_eq!(json["range"], range);
        assert_eq!(json["series"]["total"][0][1], 3.0);
    }
    assert_eq!(
        api.respond(&store, "GET", "/api/status/series", T0 + 60)
            .status,
        200,
        "day is the default"
    );
    assert_eq!(
        api.respond(&store, "GET", "/api/status/series?range=decade", T0 + 60)
            .status,
        400
    );
    assert_eq!(api.respond(&store, "GET", "/nope", T0).status, 404);
    assert_eq!(
        api.respond(&store, "POST", "/api/status/series", T0).status,
        405
    );
    assert_eq!(api.respond(&store, "GET", "/healthz", T0).body, b"ok");
}

#[test]
fn the_api_serves_a_built_answer_for_thirty_seconds() {
    let mut store = Store::open_memory().unwrap();
    store.record(&sample(T0, 3, 0, 0)).unwrap();
    let api = Api::default();
    let first = api.respond(&store, "GET", "/api/status/series?range=day", T0 + 60);
    store.record(&sample(T0 + 60, 50, 0, 0)).unwrap();
    let again = api.respond(&store, "GET", "/api/status/series?range=day", T0 + 80);
    assert_eq!(first.body, again.body);
    let fresh = api.respond(&store, "GET", "/api/status/series?range=day", T0 + 120);
    assert_ne!(first.body, fresh.body);
}
