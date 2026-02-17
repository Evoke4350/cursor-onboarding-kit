# RFP: Orbital Coverage Revival (v0.9 Draft From Product)

## Why This Exists (Typed In One Sitting, Sorry)

Hi engineering team, this is from me (PM) after three customer calls, two airport delays, and one board prep where I got asked "do we *actually* know when on-call is covered?" and I did not enjoy that question.

I know we have a "working" scheduling module, but we need it to become narratively trustworthy, not just technically truthful. Legal also asked if a secondary shift "counts" and I said "it depends," which was apparently not leadership energy.

## Outcome We Want

We need a **Coverage Confidence Layer** that can tell two truths at once:

1. The mechanical truth: what the old math says.
2. The human truth: what someone would reasonably call "actually covered."

At the same time, we need escalation behavior that never disappears into a void, and timezone parsing that survives half-hour offsets because APAC keeps escalating this.

## Scope (High-Level)

- Scheduling/reporting outputs in `scheduleReport.ts`
- Availability/timezone handling in `availability.ts`
- Escalation routing in `notifications.ts`
- Shared type additions in `types.ts`
- Tests proving old behavior can still be surfaced where needed

## Detailed Requirements

### R1: Dual Coverage Metrics ("Bifocal Coverage")

Add two explicit metrics in weekly and member report paths:

- `coverageRawPercent`: legacy overlap-permissive behavior (can exceed 100).
- `coverageEffectivePercent`: deduped, clamped, and counting only `primary` + `override`.

Rules:

- `secondary` shifts are backup-only and **must not** count toward `coverageEffectivePercent`.
- `coverageEffectivePercent` is capped at 100.
- If `coverageRawPercent - coverageEffectivePercent >= 20`, flag as "inflated coverage".

### R2: Shadow Gap Semantics

When a window has no effective coverage but does have secondary coverage, classify as:

- "shadow-covered gap" (still a gap, but operationally softer).

This should appear as a first-class output list in report generation.

### R3: Escalation Never Vanishes

If primary assignee is unavailable and no direct escalation target exists:

1. Attempt escalation to any available `lead` in team order.
2. If none available, send a manager summary alert immediately (not end-of-week only).
3. Include a reason code so audit can see fallback path.

No silent drop. Ever.

### R4: Timezone Parsing Upgrade

`parseTimezoneOffsetMs` must support:

- `UTC+5:30`, `UTC-3:30`, `UTC+9:30`
- existing whole-hour format (`UTC+9`, etc.)

If malformed:

- return `0` as today (to avoid hard break),
- but surface a warning path we can inspect later (shape is up to engineering).

### R5: Backward-Compatible Output

Some consumers likely still rely on today’s broken-ish values, so we need both:

- legacy fields preserved where currently returned,
- new explicit fields added (do not replace silently).

## Non-Functional Notes

- No new dependencies.
- Keep this as plain TypeScript.
- Prefer targeted changes over architectural rewrites.
- Keep docs updated because this is training material.

## Suggested Acceptance Checks

1. Overlapping primary+secondary full-day shifts produce:
   - raw > 100
   - effective = 100
2. Pure-secondary day produces:
   - effective gap present
   - shadow-covered gap present
3. Critical escalation with no contact still emits at least one sent notification.
4. Half-hour timezone strings parse and round-trip through conflict checks.

## Explicit Out-of-Scope

- PagerDuty API integration.
- SMS re-platform.
- UI redesign.
- Anything involving monthly planning calendars (different roadmap lane).

## PM Vocabulary Glossary (Because I Keep Saying This)

- "Narratively trustworthy": output that makes sense to non-engineers in incident review.
- "Coverage inflation": overlap math saying we are safer than we are.
- "Shadow-covered gap": secondary exists, but primary responsibility is still missing.

