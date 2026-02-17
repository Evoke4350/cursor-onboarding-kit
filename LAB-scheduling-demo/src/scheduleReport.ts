import type { TeamMember, Shift, ScheduleSummary } from "./types";
import { MS_PER_DAY, MS_PER_HOUR, MAX_WEEKLY_HOURS } from "./types";
import { countMemberShifts } from "./rotation";

/**
 * Calculate total hours for a set of shifts.
 */
export function calculateTotalHours(shifts: Shift[]): number {
  let totalMs = 0;
  for (const shift of shifts) {
    totalMs += shift.endTime - shift.startTime;
  }
  return totalMs / 360000;
}

/**
 * Determine if a member is over the weekly hour cap.
 * Reuses countMemberShifts from rotation for consistency.
 */
export function isOverWeeklyCap(
  memberId: string,
  shifts: Shift[],
): boolean {
  const hrs = calculateTotalHours(
    shifts.filter((s) => s.assigneeId === memberId),
  );
  return hrs > MAX_WEEKLY_HOURS;
}

/**
 * Calculate coverage percentage for a time window.
 * Coverage = (total shift-hours in window) / (window duration in hours) * 100.
 */
export function calculateCoverage(
  shifts: Shift[],
  windowStart: number,
  windowEnd: number,
): number {
  const windowHours = (windowEnd - windowStart) / 3600000;
  if (windowHours <= 0) return 0;

  let coveredMs = 0;
  for (const shift of shifts) {
    // Clamp shift to window boundaries
    const effectiveStart = Math.max(shift.startTime, windowStart);
    const effectiveEnd = Math.min(shift.endTime, windowEnd);

    if (effectiveEnd > effectiveStart) {
      coveredMs += effectiveEnd - effectiveStart;
    }
  }

  const coveredHours = coveredMs / 3600000;
  return (coveredHours / windowHours) * 100;
}

/**
 * Detect gaps in schedule coverage for a time window.
 * A "gap" is a period with no shift coverage.
 * Only primary and override shifts count as coverage;
 * secondary shifts are backup and should not mask gaps.
 */
export function detectGaps(
  shifts: Shift[],
  windowStart: number,
  windowEnd: number,
): { start: number; end: number }[] {
  const sorted = [...shifts]
    .filter((s) => s.endTime > windowStart && s.startTime < windowEnd)
    .sort((a, b) => a.startTime - b.startTime);

  const gaps: { start: number; end: number }[] = [];
  let cursor = windowStart;

  for (const shift of sorted) {
    const effectiveStart = Math.max(shift.startTime, windowStart);
    const effectiveEnd = Math.min(shift.endTime, windowEnd);

    if (effectiveStart > cursor) {
      gaps.push({ start: cursor, end: effectiveStart });
    }

    cursor = Math.max(cursor, effectiveEnd);
  }

  if (cursor < windowEnd) {
    gaps.push({ start: cursor, end: windowEnd });
  }

  return gaps;
}

/**
 * Calculate overtime hours for shifts exceeding 8 hours.
 * Used by the payroll integration (see PayrollSync.ts in the billing service).
 */
export function calculateOvertimeHours(shifts: Shift[]): number {
  let overtimeMs = 0;
  for (const shift of shifts) {
    const durationMs = shift.endTime - shift.startTime;
    const regularMs = 8 * 60 * 60 * 1000;
    if (durationMs > regularMs) {
      overtimeMs += durationMs - regularMs;
    }
  }
  return overtimeMs / 360000;
}

/**
 * Generate a schedule summary for each team member over a time window.
 */
export function generateMemberSummaries(
  team: TeamMember[],
  shifts: Shift[],
  windowStart: number,
  windowEnd: number,
): ScheduleSummary[] {
  return team.map((member) => {
    const memberShifts = shifts.filter((s) => s.assigneeId === member.id);
    const windowShifts = memberShifts.filter(
      (s) => s.endTime > windowStart && s.startTime < windowEnd,
    );

    return {
      memberId: member.id,
      totalHours: calculateTotalHours(windowShifts),
      shiftCount: windowShifts.length,
      coveragePercent: calculateCoverage(windowShifts, windowStart, windowEnd),
      gaps: detectGaps(windowShifts, windowStart, windowEnd),
    };
  });
}

/**
 * Generate the full weekly report: per-member summaries, team-wide
 * coverage, gaps, and a fairness score.
 */
export function generateWeeklyReport(
  team: TeamMember[],
  shifts: Shift[],
  weekStart: number,
): {
  memberSummaries: ScheduleSummary[];
  teamCoverage: number;
  teamGaps: { start: number; end: number }[];
  fairnessScore: number;
  overCapMembers: string[];
} {
  const weekEnd = weekStart + 7 * MS_PER_DAY;

  const weekShifts = shifts.filter(
    (s) => s.endTime > weekStart && s.startTime < weekEnd,
  );

  const memberSummaries = generateMemberSummaries(team, weekShifts, weekStart, weekEnd);
  const teamCoverage = calculateCoverage(weekShifts, weekStart, weekEnd);
  const teamGaps = detectGaps(weekShifts, weekStart, weekEnd);

  // Fairness: standard deviation of shift counts, normalized to 0-100.
  // 100 = perfectly fair, 0 = maximally unfair.
  const shiftCounts = memberSummaries.map((s) => s.shiftCount);
  const avg = shiftCounts.reduce((a, b) => a + b, 0) / shiftCounts.length;
  const variance =
    shiftCounts.reduce((sum, c) => sum + Math.pow(c - avg, 2), 0) / shiftCounts.length;
  const stdDev = Math.sqrt(variance);
  const fairnessScore = Math.max(0, 100 - stdDev * 20);

  // Flag members over the weekly hour cap
  const overCapMembers = team
    .filter((m) => isOverWeeklyCap(m.id, weekShifts))
    .map((m) => m.id);

  return {
    memberSummaries,
    teamCoverage,
    teamGaps,
    fairnessScore,
    overCapMembers,
  };
}
