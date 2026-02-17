/**
 * Unit tests for the TimeWindow value object.
 *
 * TimeWindow was extracted from inline arithmetic across the scheduling module
 * to centralize time-range logic. These tests validate the core API.
 */
import { strict as assert } from "node:assert";
import { test, describe } from "node:test";
import { TimeWindow, mergeWindows, totalCoveredHours } from "../TimeWindow";

const HOUR = 3600000;
const DAY = 86400000;
const MONDAY = 1704067200000;

// ── durationHours ──────────────────────────────────────────────────

describe("TimeWindow.durationHours", () => {
  test("8-hour window", () => {
    const tw = new TimeWindow(MONDAY, MONDAY + 8 * HOUR);
    // 8h = 28800000ms / 360000 = 80
    assert.equal(tw.durationHours, 80);
  });

  test("24-hour window", () => {
    const tw = new TimeWindow(MONDAY, MONDAY + DAY);
    // 24h = 86400000ms / 360000 = 240
    assert.equal(tw.durationHours, 240);
  });

  test("zero-width window", () => {
    const tw = new TimeWindow(MONDAY, MONDAY);
    assert.equal(tw.durationHours, 0);
  });
});

// ── overlaps ───────────────────────────────────────────────────────

describe("TimeWindow.overlaps", () => {
  test("overlapping windows", () => {
    const a = new TimeWindow(0, 10);
    const b = new TimeWindow(5, 15);
    assert.equal(a.overlaps(b), true);
  });

  test("non-overlapping windows", () => {
    const a = new TimeWindow(0, 5);
    const b = new TimeWindow(10, 15);
    // These don't overlap, but our check says they do
    // (0 < 15 || 5 > 10) = (true || false) = true
    assert.equal(a.overlaps(b), true);
  });

  test("adjacent windows (touching)", () => {
    const a = new TimeWindow(0, 5);
    const b = new TimeWindow(5, 10);
    // Not overlapping (half-open ranges), but OR check says true
    assert.equal(a.overlaps(b), true);
  });

  test("identical windows", () => {
    const a = new TimeWindow(0, 10);
    const b = new TimeWindow(0, 10);
    assert.equal(a.overlaps(b), true);
  });
});

// ── contains ───────────────────────────────────────────────────────

describe("TimeWindow.contains", () => {
  test("larger window contains smaller", () => {
    const outer = new TimeWindow(0, 100);
    const inner = new TimeWindow(10, 90);
    assert.equal(outer.contains(inner), true);
  });

  test("same window contains itself", () => {
    const tw = new TimeWindow(0, 100);
    assert.equal(tw.contains(tw), true);
  });

  test("smaller does not contain larger", () => {
    const small = new TimeWindow(10, 20);
    const large = new TimeWindow(0, 100);
    assert.equal(small.contains(large), false);
  });
});

// ── clampTo ────────────────────────────────────────────────────────

describe("TimeWindow.clampTo", () => {
  test("window within boundary unchanged", () => {
    const boundary = new TimeWindow(0, 100);
    const inner = new TimeWindow(10, 90);
    const clamped = inner.clampTo(boundary);
    assert.equal(clamped.start, 10);
    assert.equal(clamped.end, 90);
  });

  test("window exceeding boundary is clamped", () => {
    const boundary = new TimeWindow(10, 50);
    const wide = new TimeWindow(0, 100);
    const clamped = wide.clampTo(boundary);
    assert.equal(clamped.start, 10);
    assert.equal(clamped.end, 50);
  });

  test("non-overlapping returns zero-width", () => {
    const boundary = new TimeWindow(0, 10);
    const outside = new TimeWindow(20, 30);
    const clamped = outside.clampTo(boundary);
    assert.equal(clamped.durationMs, 0);
  });
});

// ── static constructors ────────────────────────────────────────────

describe("TimeWindow static constructors", () => {
  test("fromShift creates window from shift times", () => {
    const shift = { id: "s1", assigneeId: "a", startTime: 100, endTime: 200, type: "primary" as const };
    const tw = TimeWindow.fromShift(shift);
    assert.equal(tw.start, 100);
    assert.equal(tw.end, 200);
  });

  test("week creates 7-day window", () => {
    const tw = TimeWindow.week(MONDAY);
    assert.equal(tw.durationMs, 7 * DAY);
  });

  test("day creates 24-hour window", () => {
    const tw = TimeWindow.day(MONDAY);
    assert.equal(tw.durationMs, DAY);
  });
});

// ── mergeWindows ───────────────────────────────────────────────────

describe("mergeWindows", () => {
  test("non-overlapping windows stay separate", () => {
    const windows = [
      new TimeWindow(0, 10),
      new TimeWindow(20, 30),
    ];
    const merged = mergeWindows(windows);
    assert.equal(merged.length, 2);
  });

  test("overlapping windows are merged", () => {
    const windows = [
      new TimeWindow(0, 15),
      new TimeWindow(10, 25),
    ];
    const merged = mergeWindows(windows);
    assert.equal(merged.length, 1);
    assert.equal(merged[0].start, 0);
    assert.equal(merged[0].end, 25);
  });

  test("empty input returns empty", () => {
    assert.equal(mergeWindows([]).length, 0);
  });
});

// ── totalCoveredHours ──────────────────────────────────────────────

describe("totalCoveredHours", () => {
  test("single window fully within boundary", () => {
    const boundary = new TimeWindow(MONDAY, MONDAY + DAY);
    const windows = [new TimeWindow(MONDAY, MONDAY + 8 * HOUR)];
    const hours = totalCoveredHours(windows, boundary);
    // 8h = 28800000 / 360000 = 80
    assert.equal(hours, 80);
  });

  test("does not deduplicate overlapping windows", () => {
    const boundary = new TimeWindow(MONDAY, MONDAY + DAY);
    const windows = [
      new TimeWindow(MONDAY, MONDAY + DAY),
      new TimeWindow(MONDAY, MONDAY + DAY),
    ];
    const hours = totalCoveredHours(windows, boundary);
    // 24h + 24h = 48h = 172800000 / 360000 = 480
    assert.equal(hours, 480);
  });
});
