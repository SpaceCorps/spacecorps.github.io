//! The history store: one SQLite file, three resolutions.
//!
//! Every minute the sampler records one row per server (and one for the sum of all servers) at
//! minute resolution (tier 0). The same sample is folded into a 5-minute bucket (tier 1) and an
//! hourly bucket (tier 2) with an upsert, so every tier is always current and no separate roll-up job
//! exists. Old rows of the fine tiers are dropped; the hourly tier is kept for ever (about 35,000 rows
//! a year for four series), so the year and the "since the beginning" views stay cheap.
//!
//! | tier | bucket  | kept       | used for                              |
//! |------|---------|------------|---------------------------------------|
//! | 0    | 1 min   | 2 days     | the first hours of a new install      |
//! | 1    | 5 min   | 95 days    | today, week, month                    |
//! | 2    | 1 hour  | for ever   | 3 months, year, since the beginning   |

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

/// The id of the sum over all servers.
pub const TOTAL: &str = "total";

/// Bucket length in seconds of each tier.
const BUCKET: [i64; 3] = [60, 300, 3600];
/// How long each tier is kept, in seconds (`None`: for ever).
const KEEP: [Option<i64>; 3] = [Some(2 * 86_400), Some(95 * 86_400), None];
/// A chart gets at most about this many points per series.
pub const MAX_POINTS: i64 = 400;
/// The step of a point in seconds and the tier it is read from, finest first.
const LADDER: [(i64, usize); 7] = [
    (60, 0),
    (300, 1),
    (1_800, 1),
    (7_200, 1),
    (21_600, 2),
    (86_400, 2),
    (604_800, 2),
];

/// One server as the sampler saw it at one moment.
#[derive(Clone, Debug, PartialEq)]
pub struct ServerSample {
    pub id: String,
    pub name: String,
    pub region: String,
    pub kind: String,
    /// The server's own word: up, full, updating, maintenance, down.
    pub status: String,
    pub up: bool,
    pub online: i64,
    pub flying: i64,
}

/// What one look at the servers gave. `ts` is Unix seconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Sample {
    pub ts: i64,
    pub servers: Vec<ServerSample>,
}

impl Sample {
    /// The servers of the sample plus the row that sums them (`total`).
    fn rows(&self) -> Vec<ServerSample> {
        let mut rows = self.servers.clone();
        rows.push(ServerSample {
            id: TOTAL.into(),
            name: "All servers".into(),
            region: String::new(),
            kind: TOTAL.into(),
            status: if self.servers.iter().any(|s| s.up) {
                "up"
            } else {
                "down"
            }
            .into(),
            up: self.servers.iter().any(|s| s.up),
            online: self.servers.iter().map(|s| s.online).sum(),
            flying: self.servers.iter().map(|s| s.flying).sum(),
        });
        rows
    }
}

/// The time ranges of the chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Range {
    Day,
    Week,
    Month,
    Quarter,
    Year,
    All,
}

impl Range {
    pub fn parse(s: &str) -> Option<Range> {
        Some(match s {
            "day" => Range::Day,
            "week" => Range::Week,
            "month" => Range::Month,
            "quarter" => Range::Quarter,
            "year" => Range::Year,
            "all" => Range::All,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Range::Day => "day",
            Range::Week => "week",
            Range::Month => "month",
            Range::Quarter => "quarter",
            Range::Year => "year",
            Range::All => "all",
        }
    }

    /// Seconds the window reaches back (`None`: to the first sample).
    fn window(self) -> Option<i64> {
        Some(match self {
            Range::Day => 86_400,
            Range::Week => 7 * 86_400,
            Range::Month => 30 * 86_400,
            Range::Quarter => 91 * 86_400,
            Range::Year => 365 * 86_400,
            Range::All => return None,
        })
    }
}

/// Where a query reads and how coarse its points are.
#[derive(Debug, PartialEq, Eq)]
pub struct Plan {
    pub from: i64,
    pub tier: usize,
    pub step: i64,
}

/// The window of a range, cut at the first sample (nothing exists before it), and the finest step
/// that keeps the series under [`MAX_POINTS`]. A young install therefore gets fine points even for
/// "year" and "all", and a full one gets 5 minutes for a day, 30 minutes for a week, 2 hours for a
/// month, 6 hours for 3 months and a day for a year.
pub fn plan(range: Range, now: i64, first: Option<i64>) -> Plan {
    let from = match (range.window(), first) {
        (Some(w), Some(f)) => (now - w).max(f),
        (Some(w), None) => now - w,
        (None, Some(f)) => f,
        (None, None) => now,
    };
    let span = (now - from).max(0);
    let (step, tier) = LADDER
        .iter()
        .copied()
        .find(|(step, _)| span / step <= MAX_POINTS)
        .unwrap_or(LADDER[LADDER.len() - 1]);
    Plan {
        from: from - from.rem_euclid(step),
        tier,
        step,
    }
}

/// One point of a series: time, mean players online in the bucket, the most at one sample, and
/// the share of samples in which the server was up (0 to 1).
pub type Point = (i64, f64, i64, f64);

/// What the chart shows about one server now.
#[derive(Debug, Serialize, PartialEq)]
pub struct ServerNow {
    pub id: String,
    pub name: String,
    pub region: String,
    pub status: String,
    pub up: bool,
    pub online: i64,
    pub flying: i64,
    /// When this was seen (Unix seconds).
    pub ts: i64,
}

/// The answer of the read API.
#[derive(Debug, Serialize, PartialEq)]
pub struct Series {
    pub range: &'static str,
    pub from: i64,
    pub to: i64,
    pub step: i64,
    /// The first sample ever recorded: nothing earlier exists.
    pub since: Option<i64>,
    pub generated: i64,
    pub servers: Vec<ServerNow>,
    pub series: BTreeMap<String, Vec<Point>>,
}

pub struct Store {
    conn: Connection,
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS buckets (
    tier INTEGER NOT NULL,
    bucket INTEGER NOT NULL,
    server TEXT NOT NULL,
    n INTEGER NOT NULL,
    sum_online INTEGER NOT NULL,
    max_online INTEGER NOT NULL,
    sum_flying INTEGER NOT NULL,
    max_flying INTEGER NOT NULL,
    up_n INTEGER NOT NULL,
    status TEXT,
    PRIMARY KEY (tier, bucket, server)
) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS servers (
    id TEXT PRIMARY KEY,
    ord INTEGER NOT NULL,
    name TEXT NOT NULL,
    region TEXT NOT NULL,
    kind TEXT NOT NULL,
    ts INTEGER NOT NULL,
    status TEXT NOT NULL,
    up INTEGER NOT NULL,
    online INTEGER NOT NULL,
    flying INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value INTEGER NOT NULL);
";

const UPSERT: &str = "
INSERT INTO buckets (tier, bucket, server, n, sum_online, max_online, sum_flying, max_flying, up_n, status)
VALUES (?1, ?2, ?3, 1, ?4, ?4, ?5, ?5, ?6, NULL)
ON CONFLICT (tier, bucket, server) DO UPDATE SET
    n = n + 1,
    sum_online = sum_online + excluded.sum_online,
    max_online = MAX(max_online, excluded.max_online),
    sum_flying = sum_flying + excluded.sum_flying,
    max_flying = MAX(max_flying, excluded.max_flying),
    up_n = up_n + excluded.up_n";

impl Store {
    /// Opens (and makes) the database for writing.
    pub fn open(path: impl AsRef<Path>) -> rusqlite::Result<Store> {
        let conn = Connection::open(path)?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        // WAL lets the read API run while the sampler writes. In memory it answers "memory": fine.
        conn.query_row("PRAGMA journal_mode = WAL", [], |r| r.get::<_, String>(0))?;
        conn.execute_batch("PRAGMA synchronous = NORMAL;")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Store { conn })
    }

    /// A second connection for reading; the file must exist.
    pub fn open_read(path: impl AsRef<Path>) -> rusqlite::Result<Store> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        Ok(Store { conn })
    }

    pub fn open_memory() -> rusqlite::Result<Store> {
        Store::open(":memory:")
    }

    /// Records a sample. Returns `false` when this minute was already recorded (a restart inside
    /// the minute), in which case nothing changes.
    pub fn record(&mut self, sample: &Sample) -> rusqlite::Result<bool> {
        let ts = sample.ts - sample.ts.rem_euclid(BUCKET[0]);
        let tx = self.conn.transaction()?;
        let mut any = false;
        for (ord, row) in sample.rows().iter().enumerate() {
            let fresh = tx.execute(
                "INSERT OR IGNORE INTO buckets (tier, bucket, server, n, sum_online, max_online, sum_flying, max_flying, up_n, status)
                 VALUES (0, ?1, ?2, 1, ?3, ?3, ?4, ?4, ?5, ?6)",
                params![ts, row.id, row.online, row.flying, row.up as i64, row.status],
            )?;
            if fresh == 0 {
                continue;
            }
            any = true;
            for (tier, width) in BUCKET.iter().enumerate().skip(1) {
                tx.execute(
                    UPSERT,
                    params![
                        tier as i64,
                        ts - ts.rem_euclid(*width),
                        row.id,
                        row.online,
                        row.flying,
                        row.up as i64
                    ],
                )?;
            }
            // The row of "now" per server: names and order come from the newest sample. An empty name
            // (Home could not be read) keeps the one we know.
            let ord = if row.id == TOTAL { -1 } else { ord as i64 };
            tx.execute(
                "INSERT INTO servers (id, ord, name, region, kind, ts, status, up, online, flying)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                 ON CONFLICT (id) DO UPDATE SET
                    ord = excluded.ord,
                    name = CASE WHEN excluded.name = '' THEN servers.name ELSE excluded.name END,
                    region = CASE WHEN excluded.region = '' THEN servers.region ELSE excluded.region END,
                    kind = CASE WHEN excluded.kind = '' THEN servers.kind ELSE excluded.kind END,
                    ts = excluded.ts, status = excluded.status, up = excluded.up,
                    online = excluded.online, flying = excluded.flying
                 WHERE excluded.ts >= servers.ts",
                params![row.id, ord, row.name, row.region, row.kind, ts, row.status, row.up as i64, row.online, row.flying],
            )?;
        }
        if any {
            tx.execute(
                "INSERT OR IGNORE INTO meta (key, value) VALUES ('first_ts', ?1)",
                params![ts],
            )?;
        }
        tx.commit()?;
        Ok(any)
    }

    /// Drops what each tier no longer keeps. Returns the rows removed.
    pub fn prune(&mut self, now: i64) -> rusqlite::Result<usize> {
        let mut removed = 0;
        for (tier, keep) in KEEP.iter().enumerate() {
            if let Some(keep) = keep {
                removed += self.conn.execute(
                    "DELETE FROM buckets WHERE tier = ?1 AND bucket < ?2",
                    params![tier as i64, now - keep],
                )?;
            }
        }
        Ok(removed)
    }

    /// The time of the first sample ever recorded.
    pub fn first_ts(&self) -> rusqlite::Result<Option<i64>> {
        self.conn
            .query_row("SELECT value FROM meta WHERE key = 'first_ts'", [], |r| {
                r.get(0)
            })
            .optional()
    }

    /// The series of every server for a range.
    pub fn series(&self, range: Range, now: i64) -> rusqlite::Result<Series> {
        let first = self.first_ts()?;
        let plan = plan(range, now, first);
        let mut servers = self.conn.prepare(
            "SELECT id, name, region, status, up, online, flying, ts FROM servers ORDER BY ord, id",
        )?;
        let servers = servers
            .query_map([], |r| {
                let id: String = r.get(0)?;
                let name: String = r.get(1)?;
                Ok(ServerNow {
                    name: if name.is_empty() { id.clone() } else { name },
                    id,
                    region: r.get(2)?,
                    status: r.get(3)?,
                    up: r.get::<_, i64>(4)? != 0,
                    online: r.get(5)?,
                    flying: r.get(6)?,
                    ts: r.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        let mut series: BTreeMap<String, Vec<Point>> =
            servers.iter().map(|s| (s.id.clone(), Vec::new())).collect();
        let mut stmt = self.conn.prepare(
            "SELECT server, (bucket / ?1) * ?1 AS t, SUM(sum_online), SUM(n), MAX(max_online), SUM(up_n)
             FROM buckets WHERE tier = ?2 AND bucket >= ?3 AND bucket <= ?4
             GROUP BY server, t ORDER BY server, t",
        )?;
        let rows = stmt.query_map(params![plan.step, plan.tier as i64, plan.from, now], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, i64>(5)?,
            ))
        })?;
        for row in rows {
            let (server, t, sum, n, max, up) = row?;
            if n > 0 {
                // The share up is exactly 1 only when every sample was up: one down minute in a week must still show.
                let up_share = if up >= n {
                    1.0
                } else {
                    ((up as f64 / n as f64 * 1000.0).round() / 1000.0).min(0.999)
                };
                series.entry(server).or_default().push((
                    t,
                    (sum as f64 / n as f64 * 10.0).round() / 10.0,
                    max,
                    up_share,
                ));
            }
        }
        Ok(Series {
            range: range.name(),
            from: plan.from,
            to: now,
            step: plan.step,
            since: first,
            generated: now,
            servers,
            series,
        })
    }
}
