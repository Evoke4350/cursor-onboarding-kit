# SCHED-2453: [SEV-1] Production Rotation Failure While CI/CD Is Green

**Type:** Incident Ticket / Hotfix Story  
**Priority:** Blocker (SEV-1)  
**Story Points:** N/A (incident lane)  
**Reporter:** PM-Operations  
**Assignee:** On-Call Platform Squad  
**Status:** Open

## Summary

Production on-call routing is objectively broken (missed pages, false "healthy coverage"), but CI/CD is still passing because current tests lock in buggy behavior as expected output.

## What Is On Fire Right Now

1. Critical alerts are routed to `email` instead of `pager`.
2. Unavailable engineers are still treated as eligible in rotation assignment logic.
3. Weekly report can show >100% coverage while real primary coverage gaps exist.
4. Week boundary handling includes shifts from next week into current week planning.
5. Half-hour timezone regions are parsed as invalid and silently coerced.

## Customer/Business Impact

- Incident notifications are delayed or missed.
- Compliance reporting claims coverage that does not exist operationally.
- APAC scheduling trust is collapsing because conflict detection is inconsistent.
- Leadership dashboard appears healthy while responders are manually firefighting.

## Why CI/CD Is Green (Root of Confusion)

Current tests in `src/__tests__/` are characterization tests for known-bad behavior.  
They validate the current broken outputs, so pipeline success is not a correctness signal.

Examples:

- `src/__tests__/notifications.test.ts` expects critical urgency to map to `email`.
- `src/__tests__/rotation.test.ts` expects unavailable members to remain eligible.
- `src/__tests__/scheduleReport.test.ts` expects overlapping shifts to produce inflated coverage.
- `src/__tests__/availability.test.ts` expects non-overlap to still return overlap in edge cases.

CI is therefore doing exactly what it was told, not what production needs.

## Reproduction Scenario (Prod-Like)

1. Create a primary `override` incident shift for an unavailable engineer.
2. Leave escalation contact unset for that engineer.
3. Add overlapping `primary` + `secondary` coverage in the same window.
4. Run weekly report and notification routing.

Observed:

- At least one critical alert is sent to non-pager channel.
- Coverage appears inflated.
- Escalation path does not guarantee an immediate operational fallback.

Expected:

- Critical pages go to pager-capable targets.
- Effective coverage is capped and role-aware.
- No escalation branch silently disappears.

## Required Fix Scope

### Immediate Hotfix (this ticket)

1. Correct critical channel routing logic.
2. Fix unavailable-member eligibility filter.
3. Fix overlap math and effective coverage semantics for report truthfulness.
4. Enforce deterministic escalation fallback when no direct contact exists.
5. Correct week-boundary filter and timezone offset handling.

### Guardrail Work (must be in same PR or directly chained)

1. Add invariant tests that encode business-correct behavior.
2. Keep characterization tests only where needed, but annotate them as legacy.
3. Add at least one integration-style "prod-like" test that fails before fix and passes after.

## Acceptance Criteria

1. A pre-fix run reproduces the prod failure with a deterministic test fixture.
2. Post-fix, critical urgency routes to pager path.
3. Post-fix, unavailable engineers are excluded from assignment.
4. Post-fix, weekly effective coverage never exceeds 100 and excludes secondary-only masking.
5. Post-fix, CI fails if legacy bug behavior is reintroduced.
6. A short "Why CI was green while prod failed" note is added to PR description.

## Non-Goals

- No UI changes.
- No external notification vendor migration.
- No schema/storage migrations.

## Notes for Whoever Picks This Up

If tests pass but this incident scenario still fails, the tests are lying to you.  
Ship truth, not green checkmarks.

