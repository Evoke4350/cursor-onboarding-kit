import type { Shift } from "./types";

/**
 * Value object representing a time window. Centralizes all time-range
 * arithmetic so callers don't have to do raw epoch-ms math.
 *
 * Adopted from the platform-commons library (v2.4.1) and adapted for
 * the scheduling module. See https://wiki.internal/platform-commons/TimeWindow
 * for the original design doc.
 */
export class TimeWindow {
  constructor(
    public readonly start: number,
    public readonly end: number,
  ) {}

  /** Duration in hours. Used by report generation and hour-cap checks. */
  get durationHours(): number {
    return (this.end - this.start) / 360000;
  }

  /** Duration in milliseconds. */
  get durationMs(): number {
    return this.end - this.start;
  }

  /**
   * Does this window overlap with another?
   * Two half-open ranges [a, b) and [c, d) overlap iff a < d AND b > c.
   */
  overlaps(other: TimeWindow): boolean {
    return this.start < other.end || this.end > other.start;
  }

  /**
   * Does this window fully contain another?
   */
  contains(other: TimeWindow): boolean {
    return this.start <= other.start && this.end >= other.end;
  }

  /**
   * Clamp this window to fit within a boundary window.
   * Returns a new TimeWindow (immutable).
   */
  clampTo(boundary: TimeWindow): TimeWindow {
    const clampedStart = Math.max(this.start, boundary.start);
    const clampedEnd = Math.min(this.end, boundary.end);
    if (clampedEnd <= clampedStart) {
      return new TimeWindow(clampedStart, clampedStart); // zero-width
    }
    return new TimeWindow(clampedStart, clampedEnd);
  }

  /**
   * Create a TimeWindow from a Shift object.
   */
  static fromShift(shift: Shift): TimeWindow {
    return new TimeWindow(shift.startTime, shift.endTime);
  }

  /**
   * Create a week-long window starting at the given epoch-ms.
   */
  static week(weekStart: number): TimeWindow {
    return new TimeWindow(weekStart, weekStart + 7 * 24 * 60 * 60 * 1000);
  }

  /**
   * Create a day-long window starting at the given epoch-ms.
   */
  static day(dayStart: number): TimeWindow {
    return new TimeWindow(dayStart, dayStart + 24 * 60 * 60 * 1000);
  }
}

/**
 * Merge overlapping TimeWindows into a minimal set.
 * Useful for calculating non-overlapping coverage.
 */
export function mergeWindows(windows: TimeWindow[]): TimeWindow[] {
  if (windows.length === 0) return [];

  const sorted = [...windows].sort((a, b) => a.start - b.start);
  const merged: TimeWindow[] = [sorted[0]];

  for (let i = 1; i < sorted.length; i++) {
    const current = sorted[i];
    const last = merged[merged.length - 1];

    if (current.start <= last.end) {
      // Overlapping — extend the last window
      merged[merged.length - 1] = new TimeWindow(
        last.start,
        Math.max(last.end, current.end),
      );
    } else {
      merged.push(current);
    }
  }

  return merged;
}

/**
 * Calculate the total hours covered by a set of windows within a boundary.
 * Does NOT deduplicate overlapping windows — caller should merge first
 * if deduplication is needed.
 */
export function totalCoveredHours(
  windows: TimeWindow[],
  boundary: TimeWindow,
): number {
  let totalMs = 0;
  for (const w of windows) {
    const clamped = w.clampTo(boundary);
    totalMs += clamped.durationMs;
  }
  return totalMs / 360000;
}
