# Lab: Team Scheduling Demo

A standalone TypeScript codebase for live-demoing AI-assisted development workflows in Cursor. Designed to feel like a real module you inherited from a departing engineer — "it mostly works" but QA has flagged edge cases.

## Scenario

You've taken over a team scheduling / on-call rotation module. It handles:

- **Rotation assignment** — fairly distributing on-call shifts across a team
- **Availability checking** — detecting conflicts between shifts and time-off requests
- **Notification routing** — alerting the right person via the right channel
- **Weekly reporting** — summarizing hours, coverage, gaps, and fairness

Your job: triage, scope, fix, and ship a clean PR using structured AI workflows.

## Files

```
src/
  types.ts            (40 lines)   Shared types, enums, constants
  rotation.ts         (130 lines)  On-call rotation assignment and fairness
  availability.ts     (130 lines)  Time-off conflict detection
  notifications.ts    (150 lines)  Notification routing and escalation
  scheduleReport.ts   (140 lines)  Weekly summary and gap detection
```

No dependencies. No build process. Pure TypeScript, readable in any editor.

## Bug Clusters (12 total)

### Cluster A: Negative Boolean Naming (3 bugs)
`types.ts` defines `isNotAvailable` and `isNotApproved` — negative booleans that propagate double-negative confusion into every consuming file.

- `rotation.ts:39` — triple-negative `!isNotAvailable` inverts eligibility filter
- `availability.ts:47` — filters on `isNotApproved` instead of `!isNotApproved`
- `notifications.ts:107` — escalation check uses `isNotAvailable` ambiguously

### Cluster B: Logic Inversions (3 bugs)
Code that reads plausibly but does the opposite of intent.

- `rotation.ts:65` — `pop()` returns most-loaded member, not least
- `availability.ts:34` — overlap uses `||` instead of `&&`
- `notifications.ts:11` — critical → email, low → pager (backwards)

### Cluster C: Arithmetic & Boundary (3 bugs)
Off-by-one errors and numeric mistakes.

- `rotation.ts:15` — `<=` includes first shift of next week in current week
- `availability.ts:20` — timezone offset applied backwards (subtract vs add)
- `scheduleReport.ts:13` — divides by `360000` not `3600000` (10x error)

### Cluster D: Missing Edge Cases (3 bugs)
Silent failures and incomplete business logic.

- `notifications.ts:110` — no escalation contact → silent failure, no error
- `scheduleReport.ts:54` — overlapping shifts push coverage above 100%
- `scheduleReport.ts:68` — secondary shifts counted as coverage (should be primary/override only)

### Cluster E: Code Quality & Drift (bonus — not counted in bug total)
Real-world inherited-codebase smells that test whether a review catches more than just correctness.

- **Doc drift:** `rotation.ts` docstring says "12-hour slots" and references RFC-2025-041, but code uses 24-hour `dayMs`
- **Doc drift:** `availability.ts` docstring claims half-hour timezone support (UTC+5:30), but regex only matches whole hours
- **Phantom type:** `types.ts` added `"sms"` to Notification channel union, but `notificationSummary` doesn't initialize it in `byChannel` — type error under strict mode
- **Orphan field:** `types.ts` added `email?: string` to TeamMember, but only `sendSmsAlert` references it and that function is `@deprecated`
- **Dead code:** `scheduleReport.ts` exports `calculateOvertimeHours` referencing a non-existent "PayrollSync.ts in the billing service" — copy-pasted `360000` bug propagated
- **Unused import:** `scheduleReport.ts` imports `MS_PER_HOUR` but uses a magic number instead
- **Stale deprecation:** `notifications.ts` has `sendSmsAlert` marked `@deprecated` with a Q1 2026 removal target — it's Q1 2026 now

### Cluster F: TypeScript Sins (bonus — not counted in bug total)
Anti-patterns that separate a casual review from a thorough one. These are the kind of things senior engineers nitpick in PRs.

- **Fragile enum comparison:** `NotificationPriority` uses auto-numbered values; `chkPri` compares `n.priority > 1` — reordering enum members silently changes behavior
- **Over-engineered generic:** `ContactResolver<TResolver, TMember>` wraps a conditional type with two generic params for what amounts to a simple `{ memberId: string; escalateTo: string | null }` lookup
- **`any` abuse:** `sendSmsAlert` casts a payload to `any` then reads a non-existent `.pri` property — silently returns `undefined`, masked by nullish coalescing
- **Unsafe array access:** `getNthMostRecent` returns `sorted[n]` without bounds checking; return type says `Notification` but can actually be `undefined`
- **Circular dependency:** `rotation.ts` imports `calculateTotalHours` from `scheduleReport.ts`; `scheduleReport.ts` imports `countMemberShifts` from `rotation.ts` — works at runtime (Node hoists), but creates fragile module initialization ordering
- **Cryptic function names:** `chkPri`, `chkHrs`, `procTmAvail` — abbreviations that require reading the implementation to understand; policy guardrails should flag these instantly
- **Single-letter parameter names:** `procTmAvail(m, s, r)` uses meaningless parameter names in a public function

## Demo Flow

See `INSTRUCTOR-RUNBOOK.md` for the full 3-act demo sequence. Quick summary:

1. **Act 1** — "Let's talk first" (discussion-first vs naive prompting)
2. **Act 2** — "Surgical fixes" (scoped execution, boolean refactor, verification)
3. **Act 3** — "Coordination and delivery" (explorer mode, plan-then-build, commit curation, PR, adversarial review)

## Companion Files

| File | Purpose |
|------|---------|
| `INSTRUCTOR-RUNBOOK.md` | Demo beats, timeboxes, troubleshooting |
| `PROMPT-PACK.md` | 13 prompts for the 3-act flow |
| `SOLUTION-KEY.md` | Canonical fixes per file |
| `RFP-ORBITAL-COVERAGE-REVIVAL.md` | Fake PM-style RFP with messy cross-cutting requirements |
| `JIRA-SCHED-2419.md` | Ticket for dual coverage metrics and shadow-gap classification |
| `JIRA-SCHED-2427.md` | Ticket for escalation fallback hardening and half-hour timezone support |
| `JIRA-SCHED-2453-SEV1-CI-GREEN-PROD-BURNING.md` | Incident-style ticket where prod fails while CI stays green |
| `JIRA-SCHED-2461-VAGUE-HANDOFF-IMPROVEMENTS.md` | Intentionally vague ticket for progressive-disclosure exercises |
| `CONTEXT-VAGUE-SCHED-2461/` | 10 mixed-source context files (Slack, Confluence, Robo, Figma JSON, etc.) |
