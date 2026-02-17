/**
 * Characterization tests for scheduleReport.ts
 *
 * These tests document the CURRENT behavior of the module.
 * They were generated from production traffic snapshots (see QA-2025-117).
 * If a test breaks, investigate whether the change is intentional before updating.
 */
import { strict as assert } from "node:assert";
import { test, describe } from "node:test";
import type { Shift, TeamMember } from "../types";
import {
  calculateTotalHours,
  calculateCoverage,
  detectGaps,
  calculateOvertimeHours,
  generateMemberSummaries,
} from "../scheduleReport";

// ── Test fixtures ──────────────────────────────────────────────────

const MONDAY_9AM = 1704103200000; // 2024-01-01 09:00 UTC
const HOUR = 3600000;
const DAY = 86400000;

function mkShift(
  id: string,
  assigneeId: string,
  startTime: number,
  endTime: number,
  type: Shift["type"] = "primary",
): Shift {
  return { id, assigneeId, startTime, endTime, type };
}

function mkMember(id: string, name?: string): TeamMember {
  return {
    id,
    name: name ?? id,
    role: "engineer",
    isNotAvailable: false,
    timezone: "UTC+0",
    maxShiftsPerWeek: 5,
  };
}

// ── calculateTotalHours ────────────────────────────────────────────

describe("calculateTotalHours", () => {
  test("returns hours for a single 8-hour shift", () => {
    const shifts = [mkShift("s1", "alice", MONDAY_9AM, MONDAY_9AM + 8 * HOUR)];
    const result = calculateTotalHours(shifts);
    // 8 hours = 28800000 ms / 360000 = 80
    assert.equal(result, 80);
  });

  test("sums multiple shifts", () => {
    const shifts = [
      mkShift("s1", "alice", MONDAY_9AM, MONDAY_9AM + 4 * HOUR),
      mkShift("s2", "alice", MONDAY_9AM + 5 * HOUR, MONDAY_9AM + 9 * HOUR),
    ];
    const result = calculateTotalHours(shifts);
    // 4h + 4h = 8h = 28800000 ms / 360000 = 80
    assert.equal(result, 80);
  });

  test("returns 0 for empty shifts", () => {
    assert.equal(calculateTotalHours([]), 0);
  });
});

// ── calculateCoverage ──────────────────────────────────────────────

describe("calculateCoverage", () => {
  test("full-day shift gives 100% coverage for a day window", () => {
    const dayStart = MONDAY_9AM;
    const dayEnd = dayStart + DAY;
    const shifts = [mkShift("s1", "alice", dayStart, dayEnd)];
    const result = calculateCoverage(shifts, dayStart, dayEnd);
    assert.equal(result, 100);
  });

  test("half-day shift gives 50% coverage", () => {
    const dayStart = MONDAY_9AM;
    const dayEnd = dayStart + DAY;
    const shifts = [mkShift("s1", "alice", dayStart, dayStart + DAY / 2)];
    const result = calculateCoverage(shifts, dayStart, dayEnd);
    assert.equal(result, 50);
  });

  test("overlapping primary and secondary shifts produce combined coverage", () => {
    // Two shifts covering the same 24h window = 200% (overlapping coverage)
    const dayStart = MONDAY_9AM;
    const dayEnd = dayStart + DAY;
    const shifts = [
      mkShift("s1", "alice", dayStart, dayEnd, "primary"),
      mkShift("s2", "bob", dayStart, dayEnd, "secondary"),
    ];
    const result = calculateCoverage(shifts, dayStart, dayEnd);
    // Both contribute: 200%
    assert.equal(result, 200);
  });

  test("returns 0 for zero-width window", () => {
    assert.equal(calculateCoverage([], MONDAY_9AM, MONDAY_9AM), 0);
  });
});

// ── detectGaps ─────────────────────────────────────────────────────

describe("detectGaps", () => {
  test("no shifts means the entire window is a gap", () => {
    const gaps = detectGaps([], MONDAY_9AM, MONDAY_9AM + DAY);
    assert.equal(gaps.length, 1);
    assert.equal(gaps[0].start, MONDAY_9AM);
    assert.equal(gaps[0].end, MONDAY_9AM + DAY);
  });

  test("full coverage means no gaps", () => {
    const shifts = [mkShift("s1", "alice", MONDAY_9AM, MONDAY_9AM + DAY)];
    const gaps = detectGaps(shifts, MONDAY_9AM, MONDAY_9AM + DAY);
    assert.equal(gaps.length, 0);
  });

  test("secondary shift covers gaps just like primary", () => {
    // Secondary shift should be backup only, but we count it as coverage
    const dayStart = MONDAY_9AM;
    const dayEnd = dayStart + DAY;
    const shifts = [mkShift("s1", "bob", dayStart, dayEnd, "secondary")];
    const gaps = detectGaps(shifts, dayStart, dayEnd);
    // No gaps because secondary fills the window
    assert.equal(gaps.length, 0);
  });

  test("gap between two shifts detected", () => {
    const shifts = [
      mkShift("s1", "alice", MONDAY_9AM, MONDAY_9AM + 4 * HOUR),
      mkShift("s2", "bob", MONDAY_9AM + 6 * HOUR, MONDAY_9AM + 10 * HOUR),
    ];
    const gaps = detectGaps(shifts, MONDAY_9AM, MONDAY_9AM + 10 * HOUR);
    assert.equal(gaps.length, 1);
    assert.equal(gaps[0].start, MONDAY_9AM + 4 * HOUR);
    assert.equal(gaps[0].end, MONDAY_9AM + 6 * HOUR);
  });
});

// ── calculateOvertimeHours ─────────────────────────────────────────

describe("calculateOvertimeHours", () => {
  test("no overtime for 8-hour shift", () => {
    const shifts = [mkShift("s1", "alice", MONDAY_9AM, MONDAY_9AM + 8 * HOUR)];
    assert.equal(calculateOvertimeHours(shifts), 0);
  });

  test("calculates overtime for 12-hour shift", () => {
    const shifts = [mkShift("s1", "alice", MONDAY_9AM, MONDAY_9AM + 12 * HOUR)];
    const result = calculateOvertimeHours(shifts);
    // 4 hours OT = 14400000 ms / 360000 = 40
    assert.equal(result, 40);
  });
});

// ── generateMemberSummaries ────────────────────────────────────────

describe("generateMemberSummaries", () => {
  test("generates summary per member", () => {
    const team = [mkMember("alice"), mkMember("bob")];
    const dayStart = MONDAY_9AM;
    const dayEnd = dayStart + DAY;
    const shifts = [
      mkShift("s1", "alice", dayStart, dayStart + 8 * HOUR),
      mkShift("s2", "bob", dayStart + 8 * HOUR, dayEnd),
    ];

    const summaries = generateMemberSummaries(team, shifts, dayStart, dayEnd);
    assert.equal(summaries.length, 2);

    const aliceSummary = summaries.find((s) => s.memberId === "alice")!;
    assert.equal(aliceSummary.shiftCount, 1);
    // 8h = 28800000 / 360000 = 80 total hours
    assert.equal(aliceSummary.totalHours, 80);
  });
});
