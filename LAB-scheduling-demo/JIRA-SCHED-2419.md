# SCHED-2419: Bifocal Coverage + Shadow Gaps

**Type:** Story  
**Priority:** High  
**Story Points:** 8  
**Reporter:** PM-Operations  
**Assignee:** Unassigned  
**Sprint:** TBD

## Summary

Need reports to show both old coverage math and reality-based coverage math so leadership stops asking why we can be "200% covered" while also missing primary ownership.

## Background / Context Dump

During QBR prep, we had a slide where one week showed 143% coverage and 6 unresolved incidents. This created the exact type of trust vacuum that gets called "metrics theater." Also customer Success keeps saying backup != accountable, and they are unfortunately correct.

## Problem Statement

Current report math:

- double-counts overlaps,
- counts `secondary` as full coverage,
- can exceed 100%,
- does not classify "softly covered but not truly covered" windows.

## Requested Change

Implement new reporting outputs while preserving legacy ones.

### Engineering Intent (Non-Binding but Please Do This)

1. Add an "effective" coverage path:
   - dedupe overlaps,
   - only `primary` and `override`,
   - cap at 100.
2. Keep existing raw behavior accessible for backward compatibility.
3. Add a new `shadowGaps` output where only secondary exists.
4. Wire these outputs into weekly/member report objects.

## Acceptance Criteria

1. A full-day primary + full-day secondary overlap yields:
   - raw coverage `200`
   - effective coverage `100`
2. A day with only secondary shift yields:
   - at least one gap in effective coverage output
   - at least one item in `shadowGaps`
3. Existing tests for legacy raw behavior can still pass without rewrites.
4. New tests prove effective math and shadow-gap classification.
5. No new npm dependencies.

## Test Notes

- Prefer characterization-style tests first, then additive tests.
- Please include at least one multi-shift window crossing midnight.

## Risks / Weirdness

- Circular dependency is already present (`rotation` <-> `scheduleReport`), so avoid making that knot tighter.
- If you must add utility code, keep module boundaries obvious.

