import { strict as assert } from "node:assert";
import { describe, test } from "node:test";
import fc from "fast-check";

import { TimeWindow } from "../TimeWindow";

const MS_PER_HOUR = 60 * 60 * 1000;

describe("TimeWindow (property-based)", () => {
  test("durationHours matches ms->hours conversion (spec) @spec", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: 0, max: 1_000_000_000_000 }),
        fc.integer({ min: 0, max: 7 * 24 * MS_PER_HOUR }),
        (start, durationMs) => {
          const tw = new TimeWindow(start, start + durationMs);
          const expected = durationMs / MS_PER_HOUR;
          const actual = tw.durationHours;

          // This is a spec oracle: it will fail against the intentionally buggy baseline.
          assert.ok(
            Math.abs(actual - expected) < 1e-9,
            `durationHours expected=${expected} actual=${actual}`,
          );
        },
      ),
      { numRuns: 50 },
    );
  });

  test("overlaps matches half-open interval predicate (spec) @spec", () => {
    const windowArb = fc
      .tuple(
        fc.integer({ min: 0, max: 1_000_000_000_000 }),
        fc.integer({ min: 0, max: 7 * 24 * MS_PER_HOUR }),
      )
      .map(([start, duration]) => new TimeWindow(start, start + duration));

    fc.assert(
      fc.property(windowArb, windowArb, (a, b) => {
        const expected = a.start < b.end && a.end > b.start;
        const actual = a.overlaps(b);

        // This is a spec oracle: it will fail against the intentionally buggy baseline.
        assert.equal(actual, expected);
      }),
      { numRuns: 200 },
    );
  });
});

