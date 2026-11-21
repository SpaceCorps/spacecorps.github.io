# status: players online over time

The recorder and read API behind the chart on <https://spacecorps.tech/status.html>. One small Rust service, one SQLite file, one container.

## What it does

Once a minute, on the minute, it reads the game's public server list, `GET https://play.spacecorps.xyz/api/servers` (the list the game client shows before login: no login, CORS `*`, built in memory on the game side), and records for every server:

| field | meaning |
| --- | --- |
| `online` | pilots connected to that server, menus included (the game's own `online` number) |
| `flying` | pilots flying in the worlds (stored, not charted yet) |
| `up` | the server's status is `up` or `full`; `updating`, `maintenance` and `down` count as down |

and a `total` row that sums all servers. A pilot flying on Singapore or US East is connected to that node only (Home's station hub is closed while he flies), so the sum counts nobody twice. When Home itself cannot be read, a sample is still written: Home down, 0 online, and no rows for the nodes (they are only known through Home). The history shows the hole instead of skipping it.

## Storage and roll-ups

Every sample goes into three resolutions at once (an upsert per tier, so each is always current and there is no roll-up job):

| tier | bucket | kept | read for |
| --- | --- | --- | --- |
| 0 | 1 minute | 2 days | the first hours of a new install |
| 1 | 5 minutes | 95 days | today, week, month |
| 2 | 1 hour | for ever (about 35,000 rows a year for four series) | 3 months, year, since the beginning |

A bucket holds the sample count, the sum and the peak of `online`, and how many samples were up. The API turns that into the mean, the peak and the share of samples that were up.

**The true beginning is the first sample the recorder ever wrote.** Nothing earlier can be invented, and the API says when that was (`since`). A range that reaches back further than the data starts there, and the page says so.

## Read API

`GET /api/status/series?range=day|week|month|quarter|year|all` (default `day`), public, read-only, cached 30 s in memory and by `Cache-Control`. `/healthz` answers `ok`.

```json
{
  "range": "week", "from": 1795000000, "to": 1795604800, "step": 1800, "since": 1794700000, "generated": 1795604800,
  "servers": [{ "id": "total", "name": "All servers", "region": "", "status": "up", "up": true, "online": 35, "flying": 15, "ts": 1795604760 }, ...],
  "series": { "total": [[t, mean, peak, up_share], ...], "home": [...], "singapore-1": [...], "us-east-1": [...] }
}
```

`t` is the start of the bucket (Unix seconds), `up_share` is 1 only when every sample in the bucket was up. A bucket with no samples is simply missing.

The step is the finest of 1, 5, 30 and 120 minutes, 6 hours, 1 day and 1 week that keeps a series under 400 points over the part of the range that has data. A full install gets 5 minutes for a day, 30 minutes for a week, 2 hours for a month, 6 hours for 3 months and a day for a year or all time (a week beyond about 400 days).

## Run

```bash
cargo test                      # 13 tests, no network (a local HTTP server stands in for the game)
cargo clippy --all-targets -- -D warnings
cargo run                       # records into /data/status.db unless STATUS_DB says otherwise
docker build -t spacecorps-status . && docker run --rm -p 8090:8090 -v status-data:/data spacecorps-status
```

| variable | default | |
| --- | --- | --- |
| `STATUS_DB` | `/data/status.db` | the SQLite file (WAL); put a volume under `/data` |
| `STATUS_ADDR` | `0.0.0.0:8090` | where the read API listens |
| `STATUS_SOURCE_URL` | `https://play.spacecorps.xyz/api/servers` | the server list to sample |
| `STATUS_RECORD` | `on` | `off`: serve a database and sample nothing |

To look at the chart before there is history, `cargo run --example seed_demo -- demo.db 400` writes 400 days of **synthetic** samples; serve them with `STATUS_RECORD=off STATUS_DB=demo.db` and never mix that file with a real one.

## Deploy (Sliplane)

A second service next to the site, from this repository: Dockerfile `status/Dockerfile`, build context `status/`, a volume at `/data`, port 8090, health check `/healthz`. The site's Caddy proxies `/api/status/*` to it, so the page stays same-origin (the site promises no third-party requests): set `STATUS_UPSTREAM` on the **site** service to the status service's address, for example `https://spacecorps-status.sliplane.app`. Without it that path answers 502 and the rest of the site is untouched; the page then says the history is not available.

Back up `/data/status.db` with the volume: it is the only copy of the history.
