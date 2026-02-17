/**
 * Characterization tests for rotation.ts
 *
 * Snapshot of behavior as of the v2.3 handoff. These tests were written
 * against the running system — if they break, confirm with the team lead
 * before changing expected values.
 */
import { strict as assert } from "node:assert";
import { test, describe } from "node:test";
import type { TeamMember, Shift } from "../types";
import { MS_PER_WEEK } from "../types";
import {
  getShiftsForWeek,
  countMemberShifts,
  getEligibleMembers,
  pickLeastLoadedMember,
} from "../rotation";

const MONDAY = 1704067200000; // 2024-01-01 00:00 UTC (Monday)
const HOUR = 3600000;
const DAY = 86400000;

function mkShift(
  id: string,
  assigneeId: string,
  startTime: number,
  endTime: number,
): Shift {
  return { id, assigneeId, startTime, endTime, type: "primary" };
}

function mkMember(
  id: string,
  overrides: Partial<TeamMember> = {},
): TeamMember {
  return {
    id,
    name: id,
    role: "engineer",
    isNotAvailable: false,
    timezone: "UTC+0",
    maxShiftsPerWeek: 5,
    ...overrides,
  };
}

// ── getShiftsForWeek ───────────────────────────────────────────────

describe("getShiftsForWeek", () => {
  test("includes shifts within the week", () => {
    const shifts = [
      mkShift("s1", "alice", MONDAY + 1 * HOUR, MONDAY + 9 * HOUR),
      mkShift("s2", "bob", MONDAY + 2 * DAY, MONDAY + 2 * DAY + 8 * HOUR),
    ];
    const result = getShiftsForWeek(shifts, MONDAY);
    assert.equal(result.length, 2);
  });

  test("includes shift starting exactly at week boundary", () => {
    // A shift starting at exactly weekEnd (next Monday midnight)
    // should arguably belong to the NEXT week, but current behavior includes it
    const nextMonday = MONDAY + MS_PER_WEEK;
    const shifts = [
      mkShift("s1", "alice", nextMonday, nextMonday + 8 * HOUR),
    ];
    const result = getShiftsForWeek(shifts, MONDAY);
    // Current behavior: included (startTime <= weekEnd)
    assert.equal(result.length, 1);
  });

  test("excludes shifts before the week", () => {
    const shifts = [
      mkShift("s1", "alice", MONDAY - DAY, MONDAY - DAY + 8 * HOUR),
    ];
    const result = getShiftsForWeek(shifts, MONDAY);
    assert.equal(result.length, 0);
  });
});

// ── getEligibleMembers ─────────────────────────────────────────────

describe("getEligibleMembers", () => {
  test("available members with capacity are eligible", () => {
    const team = [
      mkMember("alice", { isNotAvailable: false }),
      mkMember("bob", { isNotAvailable: false }),
    ];
    const shifts: Shift[] = [];
    const eligible = getEligibleMembers(team, shifts);
    // Note: current behavior filters these OUT because of the isNotAvailable check
    // Available members have isNotAvailable=false, !false=true → return false → filtered out
    assert.equal(eligible.length, 0);
  });

  test("unavailable members are handled", () => {
    const team = [
      mkMember("alice", { isNotAvailable: true }),
      mkMember("bob", { isNotAvailable: true }),
    ];
    const shifts: Shift[] = [];
    const eligible = getEligibleMembers(team, shifts);
    // Unavailable members: isNotAvailable=true, !true=false → passes filter
    assert.equal(eligible.length, 2);
  });
});

// ── pickLeastLoadedMember ──────────────────────────────────────────

describe("pickLeastLoadedMember", () => {
  test("picks from eligible members", () => {
    const team = [mkMember("alice"), mkMember("bob")];
    const shifts = [
      mkShift("s1", "alice", MONDAY, MONDAY + 8 * HOUR),
      mkShift("s2", "alice", MONDAY + DAY, MONDAY + DAY + 8 * HOUR),
      mkShift("s3", "bob", MONDAY, MONDAY + 8 * HOUR),
    ];
    // Alice: 2 shifts, Bob: 1 shift
    // Sort ascending by count: [bob(1), alice(2)]
    // pop() returns alice (most loaded) — current behavior
    const picked = pickLeastLoadedMember(team, shifts);
    assert.equal(picked?.id, "alice");
  });

  test("returns null for empty list", () => {
    const picked = pickLeastLoadedMember([], []);
    assert.equal(picked, null);
  });
});
