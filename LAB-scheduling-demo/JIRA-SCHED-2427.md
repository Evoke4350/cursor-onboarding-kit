# SCHED-2427: Escalation Blackhole Guard + Half-Hour Timezones

**Type:** Story  
**Priority:** High  
**Story Points:** 5  
**Reporter:** PM-Operations  
**Assignee:** Unassigned  
**Sprint:** TBD

## Summary

When escalation target is missing, alerts quietly die. Also APAC time-off validation is wrong for half-hour offsets. Fix both in one pass so we stop paginating the incident review doc with apologies.

## Context (Messy but Real)

Two separate complaints came in the same week:

- Incident review: unavailable primary had no escalation contact and no one got paged.
- APAC manager: "UTC+5:30 staff are always weirdly blocked/allowed."

These might be separate code paths, but operationally they feel like one reliability story: routing and timing confidence.

## Requested Change

### Part A: Escalation Resilience

If `sendWithEscalation` cannot find configured contact:

1. Fallback to any available lead.
2. If no lead is available, notify all managers (immediate, not digest-only).
3. Include a machine-readable fallback reason in resulting notification payloads (shape can be small, but must exist).

No branch should silently return only the primary notification when primary is known unavailable.

### Part B: Timezone Half-Hour Support

`parseTimezoneOffsetMs` should parse:

- `UTC+5:30`
- `UTC-3:30`
- `UTC+9:30`

while preserving:

- `UTC+N` and `UTC-N` whole-hour support.

Malformed timezone strings may still default to zero offset, but add a traceable warning path (log object, return side-channel, or documented helper).

## Acceptance Criteria

1. Unavailable member + no escalation contact still results in at least one additional sent notification.
2. Critical escalation fallback prefers lead role before manager role.
3. Half-hour timezone parse returns correct millisecond offset.
4. Availability conflict checks consume those offsets correctly.
5. Existing behavior outside these branches remains unchanged.

## Suggested Test Cases

- Member unavailable, no contact, one available lead, one manager.
- Member unavailable, no contact, no leads, two managers.
- `UTC+5:30` shift + request overlap boundary case.
- Bad timezone string still runs, but warning path is observable.

## Implementation Constraints

- No external timezone libs.
- Keep changes in existing modules unless extraction reduces risk.
- Do not remove deprecated SMS code in this ticket.

