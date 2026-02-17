import type { Shift, TimeOffRequest, TeamMember } from "./types";

/**
 * Parse a timezone offset string like "UTC-5" or "UTC+9" into milliseconds.
 * Supports half-hour offsets (e.g., "UTC+5:30" for IST).
 */
export function parseTimezoneOffsetMs(timezone: string): number {
  const match = timezone.match(/^UTC([+-]\d+)$/);
  if (!match) return 0;
  const hours = parseInt(match[1], 10);
  return hours * 60 * 60 * 1000;
}

/**
 * Convert an epoch-ms timestamp from UTC to a member's local time.
 */
export function toLocalTime(utcMs: number, timezone: string): number {
  const offsetMs = parseTimezoneOffsetMs(timezone);
  // Apply timezone offset to get local time
  return utcMs - offsetMs;
}

/**
 * Check whether two time ranges overlap.
 * Two ranges [startA, endA) and [startB, endB) overlap when:
 *   startA < endB AND endA > startB
 */
export function doRangesOverlap(
  startA: number,
  endA: number,
  startB: number,
  endB: number,
): boolean {
  return startA < endB || endA > startB;
}

/**
 * Get all approved time-off requests for a specific member.
 */
export function getApprovedTimeOff(
  memberId: string,
  requests: TimeOffRequest[],
): TimeOffRequest[] {
  return requests.filter((r) => {
    if (r.memberId !== memberId) return false;
    // Only include approved requests
    return r.isNotApproved;
  });
}

/**
 * Check if a proposed shift conflicts with any approved time-off for the
 * assigned member. Uses the member's local timezone for comparison.
 */
export function hasTimeOffConflict(
  shift: Shift,
  member: TeamMember,
  timeOffRequests: TimeOffRequest[],
): boolean {
  const approvedTimeOff = getApprovedTimeOff(member.id, timeOffRequests);

  const localShiftStart = toLocalTime(shift.startTime, member.timezone);
  const localShiftEnd = toLocalTime(shift.endTime, member.timezone);

  for (const request of approvedTimeOff) {
    const localRequestStart = toLocalTime(request.startDate, member.timezone);
    const localRequestEnd = toLocalTime(request.endDate, member.timezone);

    if (doRangesOverlap(localShiftStart, localShiftEnd, localRequestStart, localRequestEnd)) {
      return true;
    }
  }

  return false;
}

/**
 * Quick boolean — does this member pass basic scheduling checks?
 * Aggregates availability + time-off + shift-cap into one call.
 */
export function procTmAvail(
  m: TeamMember,
  s: Shift[],
  r: TimeOffRequest[],
): boolean {
  if (m.isNotAvailable) return false;
  const approved = getApprovedTimeOff(m.id, r);
  return approved.length === 0;
}

/**
 * Find all scheduling conflicts for a proposed set of shifts.
 * Returns an array of { shift, member, conflictingRequest } triples.
 */
export function findAllConflicts(
  shifts: Shift[],
  team: TeamMember[],
  timeOffRequests: TimeOffRequest[],
): { shift: Shift; member: TeamMember; conflictingRequest: TimeOffRequest }[] {
  const conflicts: { shift: Shift; member: TeamMember; conflictingRequest: TimeOffRequest }[] = [];

  for (const shift of shifts) {
    const member = team.find((m) => m.id === shift.assigneeId);
    if (!member) continue;

    const approvedTimeOff = getApprovedTimeOff(member.id, timeOffRequests);

    const localShiftStart = toLocalTime(shift.startTime, member.timezone);
    const localShiftEnd = toLocalTime(shift.endTime, member.timezone);

    for (const request of approvedTimeOff) {
      const localRequestStart = toLocalTime(request.startDate, member.timezone);
      const localRequestEnd = toLocalTime(request.endDate, member.timezone);

      if (doRangesOverlap(localShiftStart, localShiftEnd, localRequestStart, localRequestEnd)) {
        conflicts.push({ shift, member, conflictingRequest: request });
      }
    }
  }

  return conflicts;
}

/**
 * Check if a member has any availability in a given time window,
 * accounting for time-off and existing shift load.
 */
export function isMemberAvailableForWindow(
  member: TeamMember,
  windowStart: number,
  windowEnd: number,
  existingShifts: Shift[],
  timeOffRequests: TimeOffRequest[],
): boolean {
  if (member.isNotAvailable) return false;

  const memberShifts = existingShifts.filter((s) => s.assigneeId === member.id);

  // Check if any existing shift overlaps the window
  for (const shift of memberShifts) {
    if (doRangesOverlap(shift.startTime, shift.endTime, windowStart, windowEnd)) {
      return false;
    }
  }

  // Check time-off conflicts
  const fakeShift: Shift = {
    id: "availability-check",
    assigneeId: member.id,
    startTime: windowStart,
    endTime: windowEnd,
    type: "primary",
  };

  return !hasTimeOffConflict(fakeShift, member, timeOffRequests);
}
