import { expect, test } from "vite-plus/test";
import {
  fmtNum,
  nearest,
  pathOf,
  runs,
  stepLabel,
  tickLabel,
  timeTicks,
  yScale,
} from "./status-chart.js";

test("yScale starts at 0 and gives a round top at most five intervals high", () => {
  expect(yScale(0)).toEqual({ step: 1, top: 1, ticks: [0, 1] });
  expect(yScale(35).ticks).toEqual([0, 10, 20, 30, 40]);
  expect(yScale(44).top).toBe(50);
  expect(yScale(120).ticks).toEqual([0, 50, 100, 150]);
  expect(yScale(3000).ticks).toEqual([0, 1000, 2000, 3000]);
  for (const m of [1, 2, 7, 13, 99, 101, 640, 9999]) {
    const s = yScale(m);
    expect(s.top).toBeGreaterThanOrEqual(m);
    expect(s.ticks.length).toBeLessThanOrEqual(6);
  }
});

test("timeTicks aligns to the clock and keeps the label count down", () => {
  const from = new Date(2026, 10, 20, 14, 7).getTime();
  const day = timeTicks(from, from + 24 * 36e5);
  expect(day.unit).toBe("h");
  expect(day.ticks.length).toBeLessThanOrEqual(6);
  for (const t of day.ticks) expect(new Date(t).getMinutes()).toBe(0);
  const week = timeTicks(from, from + 7 * 864e5);
  expect(week.unit).toBe("d");
  for (const t of week.ticks) expect(new Date(t).getHours()).toBe(0);
  const year = timeTicks(from, from + 365 * 864e5);
  expect(year.unit).toBe("m");
  expect(year.ticks.length).toBeLessThanOrEqual(6);
  for (const t of year.ticks) expect(new Date(t).getDate()).toBe(1);
  // a young install has a window of minutes: it gets minute ticks, on the clock
  const young = timeTicks(from, from + 13 * 60e3);
  expect(young.unit).toBe("n");
  expect(young.ticks.length).toBeGreaterThan(1);
  for (const t of young.ticks) expect(new Date(t).getMinutes() % young.n).toBe(0);
  expect(() => timeTicks(from, from + 30e3)).not.toThrow();
});

test("tickLabel is the clock, the date or the month", () => {
  const t = new Date(2027, 0, 5, 9, 30).getTime();
  expect(tickLabel(t, "h", "en-GB")).toBe("09:30");
  expect(tickLabel(t, "d", "en-GB")).toBe("5 Jan");
  expect(tickLabel(new Date(2027, 0, 1).getTime(), "m", "en-GB")).toBe("Jan 2027");
  expect(tickLabel(new Date(2027, 3, 1).getTime(), "m", "en-GB")).toBe("Apr");
});

test("stepLabel names the width of a point", () => {
  expect([60, 300, 1800, 3600, 7200, 21600, 86400, 604800].map(stepLabel)).toEqual([
    "minute",
    "5 minutes",
    "30 minutes",
    "hour",
    "2 hours",
    "6 hours",
    "day",
    "week",
  ]);
});

test("runs break at holes and at downtime", () => {
  const id = (v) => v;
  const pts = [
    [0, 1, 1, 1],
    [300, 2, 2, 1],
    [600, 3, 3, 1],
    [3000, 4, 4, 1],
    [3300, 5, 5, 1],
    [3600, 0, 0, 0],
    [3900, 6, 6, 1],
  ];
  const r = runs(pts, 300, id, id);
  expect(r).toEqual([
    [
      [0, 1],
      [300, 2],
      [600, 3],
    ],
    [
      [3000, 4],
      [3300, 5],
    ],
    [[3900, 6]],
  ]);
  expect(runs([], 300, id, id)).toEqual([]);
  expect(runs([[0, 1, 1, 0]], 300, id, id)).toEqual([]);
});

test("a partly down bucket is still drawn", () => {
  const id = (v) => v;
  expect(runs([[0, 5, 9, 0.4]], 300, id, id)).toEqual([[[0, 5]]]);
});

test("pathOf writes SVG path data", () => {
  expect(
    pathOf([
      [0, 10],
      [5.55, 20.04],
    ]),
  ).toBe("M0.0 10.0L5.5 20.0");
  expect(pathOf([])).toBe("");
});

test("nearest finds the closest time", () => {
  const ts = [0, 100, 200, 400];
  expect([
    nearest(ts, -5),
    nearest(ts, 40),
    nearest(ts, 60),
    nearest(ts, 290),
    nearest(ts, 310),
    nearest(ts, 9999),
  ]).toEqual([0, 0, 1, 2, 3, 3]);
  expect(nearest([7], 3)).toBe(0);
});

test("fmtNum keeps small means readable", () => {
  expect([fmtNum(0), fmtNum(3), fmtNum(3.14), fmtNum(12.6), fmtNum(1234.4)]).toEqual([
    "0",
    "3",
    "3.1",
    "13",
    "1234",
  ]);
});
