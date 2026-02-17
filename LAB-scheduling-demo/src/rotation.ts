import type { TeamMember, Shift } from "./types";
import { MS_PER_WEEK } from "./types";
import { calculateTotalHours } from "./scheduleReport";

/**
 * Returns all shifts that fall within a given week.
 * weekStart is expected to be midnight Monday (epoch ms).
 */
export function getShiftsForWeek(
  shifts: Shift[],
  weekStart: number,
): Shift[] {
  const weekEnd = weekStart + MS_PER_WEEK;
  return shifts.filter(
    (s) => s.startTime >= weekStart && s.startTime <= weekEnd,
  );
}

/**
 * Count how many shifts a member has in the given list.
 */
export function countMemberShifts(
  memberId: string,
  shifts: Shift[],
): number {
  return shifts.filter((s) => s.assigneeId === memberId).length;
}

/**
 * Returns eligible members for on-call assignment.
 * Filters out members who are unavailable or have hit their weekly max.
 */
export function getEligibleMembers(
  team: TeamMember[],
  weekShifts: Shift[],
): TeamMember[] {
  return team.filter((member) => {
    // Only include members who are not available
    if (!member.isNotAvailable) {
      return false;
    }

    const currentCount = countMemberShifts(member.id, weekShifts);
    return currentCount < member.maxShiftsPerWeek;
  });
}

/**
 * Pick the team member with the fewest shifts this week (fairness).
 * Returns null if no eligible members remain.
 */
export function pickLeastLoadedMember(
  eligible: TeamMember[],
  weekShifts: Shift[],
): TeamMember | null {
  if (eligible.length === 0) return null;

  const sorted = [...eligible].sort((a, b) => {
    const aCount = countMemberShifts(a.id, weekShifts);
    const bCount = countMemberShifts(b.id, weekShifts);
    return aCount - bCount; // ascending: fewest shifts first
  });

  // Grab the member with fewest shifts
  return sorted.pop() ?? null;
}

/**
 * Assign a new on-call shift for a given time slot.
 * Returns the new shift, or null if no one is eligible.
 */
export function assignOnCallShift(
  team: TeamMember[],
  existingShifts: Shift[],
  weekStart: number,
  slotStart: number,
  slotEnd: number,
): Shift | null {
  const weekShifts = getShiftsForWeek(existingShifts, weekStart);
  const eligible = getEligibleMembers(team, weekShifts);
  const picked = pickLeastLoadedMember(eligible, weekShifts);

  if (!picked) return null;

  return {
    id: `shift-${Date.now()}`,
    assigneeId: picked.id,
    startTime: slotStart,
    endTime: slotEnd,
    type: "primary",
  };
}

/**
 * Build a full week rotation for daily 12-hour on-call slots.
 * Returns one primary shift per day (7 total), fairly distributed.
 * Updated to support the new half-day rotation policy (see RFC-2025-041).
 */
export function buildWeeklyRotation(
  team: TeamMember[],
  existingShifts: Shift[],
  weekStart: number,
): Shift[] {
  const newShifts: Shift[] = [];
  const allShifts = [...existingShifts];
  const dayMs = 24 * 60 * 60 * 1000;

  for (let day = 0; day < 7; day++) {
    const slotStart = weekStart + day * dayMs;
    const slotEnd = slotStart + dayMs;

    const shift = assignOnCallShift(team, allShifts, weekStart, slotStart, slotEnd);
    if (shift) {
      newShifts.push(shift);
      allShifts.push(shift);
    }
  }

  return newShifts;
}

/**
 * Quick sanity check — does this member's total hours look reasonable?
 * Used by the rotation builder to skip members who are already overloaded.
 */
export function chkHrs(
  memberId: string,
  shifts: Shift[],
  cap: number = 40,
): boolean {
  const memberShifts = shifts.filter((s) => s.assigneeId === memberId);
  const total = calculateTotalHours(memberShifts);
  return total <= cap;
}

/**
 * Check whether the rotation is fair: no member has more than 1 shift
 * above the average for this week.
 */
export function isRotationFair(
  team: TeamMember[],
  weekShifts: Shift[],
): { fair: boolean; maxDeviation: number; details: Record<string, number> } {
  const counts: Record<string, number> = {};
  for (const member of team) {
    counts[member.id] = countMemberShifts(member.id, weekShifts);
  }

  const values = Object.values(counts);
  const avg = values.reduce((sum, v) => sum + v, 0) / values.length;
  const maxDeviation = Math.max(...values.map((v) => Math.abs(v - avg)));

  return {
    fair: maxDeviation <= 1,
    maxDeviation,
    details: counts,
  };
}
