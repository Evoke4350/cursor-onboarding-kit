# Solution Key: Team Scheduling Demo

Canonical fixes for all 12 bugs. Use this to verify demo outcomes.

---

## types.ts — Naming Fixes

**No bugs in types.ts itself**, but the negative boolean names propagate confusion. The rename (Prompt 4) changes:

| Field | Before | After |
|-------|--------|-------|
| `TeamMember.isNotAvailable` | `isNotAvailable: boolean` | `isAvailable: boolean` |
| `TimeOffRequest.isNotApproved` | `isNotApproved: boolean` | `isApproved: boolean` |

All consuming files must invert their conditionals.

---

## rotation.ts — 3 Fixes

### Bug 1: Week boundary off-by-one (line 18)

**Before:**
```typescript
(s) => s.startTime >= weekStart && s.startTime <= weekEnd,
```

**After:**
```typescript
(s) => s.startTime >= weekStart && s.startTime < weekEnd,
```

**Why:** A shift starting exactly at `weekEnd` (midnight next Monday) belongs to the next week. Using `<=` includes it in both weeks, inflating the current week's shift count for fairness calculations.

---

### Bug 2: isNotAvailable logic inversion (line 38-40)

**Before:**
```typescript
// Only include members who are not available
if (!member.isNotAvailable) {
  return false;
}
```

**After (with rename):**
```typescript
if (!member.isAvailable) {
  return false;
}
```

**After (without rename):**
```typescript
if (member.isNotAvailable) {
  return false;
}
```

**Why:** The triple-negative is the bug. Trace it:
- Available member: `isNotAvailable = false` → `!false = true` → `return false` → filtered OUT (wrong!)
- Unavailable member: `isNotAvailable = true` → `!true = false` → passes filter (wrong!)

The function is supposed to return eligible (available) members, but does the opposite. The misleading comment "Only include members who are not available" reads like it could be intentional, which is why this passes casual code review.

**Instructor note:** This is the poster child for why negative boolean naming is dangerous. The triple-negative (`!isNotAvailable` to check availability) defeats human reasoning. After the rename to `isAvailable`, the fix becomes `if (!member.isAvailable) return false` — immediately clear.

---

### Bug 3: Sort picks most-loaded instead of least (line 64)

**Before:**
```typescript
return sorted.pop() ?? null;
```

**After:**
```typescript
return sorted[0] ?? null;
```

**Why:** `sort((a, b) => aCount - bCount)` puts fewest-shifts first. `pop()` returns the last element (most shifts). For fairness, we want the first element (fewest shifts).

---

## availability.ts — 3 Fixes

### Bug 4: Overlap uses OR instead of AND (line 46)

**Before:**
```typescript
return startA < endB || endA > startB;
```

**After:**
```typescript
return startA < endB && endA > startB;
```

**Why:** Two half-open ranges [A, B) and [C, D) overlap when `A < D AND B > C`. With `OR`, the function returns true whenever *either* condition holds, which is almost always true for valid time ranges.

---

### Bug 5: Approved filter inverted (line 68)

**Before:**
```typescript
return r.isNotApproved;
```

**After (with rename):**
```typescript
return r.isApproved;
```

**After (without rename):**
```typescript
return !r.isNotApproved;
```

**Why:** `isNotApproved = true` means the request is NOT approved (pending/rejected). To get approved requests, we need `isNotApproved = false`, i.e., `!r.isNotApproved`. The original returns unapproved requests, so approved time-off is ignored and pending requests block scheduling.

---

### Bug 6: Timezone offset applied backwards (line 29)

**Before:**
```typescript
return utcMs - offsetMs;
```

**After:**
```typescript
return utcMs + offsetMs;
```

**Why:** `parseTimezoneOffsetMs("UTC-5")` returns `-5 * 3600000` (a negative number). To convert UTC to local: `utcMs + offset`. For UTC-5: `17:00 UTC + (-5h) = 12:00 local`. The buggy code: `17:00 UTC - (-5h) = 22:00 local` (wrong).

**Verification examples:**

| Timezone | Input (UTC) | Correct local | Buggy local |
|----------|-------------|---------------|-------------|
| UTC-5 | 17:00 | 12:00 | 22:00 |
| UTC+9 | 17:00 | 02:00 (+1d) | 08:00 |
| UTC+0 | 17:00 | 17:00 | 17:00 |

Note: UTC+0 is unaffected, which is why this bug only manifests for non-UTC members.

---

## notifications.ts — 3 Fixes

### Bug 7: Escalation silent failure (line 90)

**Before:**
```typescript
if (escalationTarget) {
  const escalation = buildShiftNotification(escalationTarget, shift, urgency);
  notifications.push(markAsSent(escalation));
}
// (function returns normally — no error, no warning)
```

**After (one approach):**
```typescript
if (escalationTarget) {
  const escalation = buildShiftNotification(escalationTarget, shift, urgency);
  notifications.push(markAsSent(escalation));
} else {
  // Fallback: notify all managers when no escalation target exists
  const managers = team.filter((m) => m.role === "manager");
  for (const manager of managers) {
    const fallback = buildShiftNotification(manager, shift, urgency);
    notifications.push(markAsSent(fallback));
  }
}
```

**Why:** When `escalationTarget` is null, the unavailable member's notification goes nowhere useful. There's no error, no fallback, no alert. The shift is effectively unacknowledged. A production-safe fix adds a fallback path.

**Instructor note:** There are multiple valid fixes here (throw an error, log a warning, notify managers, return a flag). The point is that silence is never the right answer for an unacknowledged on-call shift.

---

### Bug 8: Channel-urgency mapping backwards (line 17)

**Before:**
```typescript
if (urgency === "critical") return "email";
if (urgency === "medium") return "slack";
return "pager";
```

**After:**
```typescript
if (urgency === "critical") return "pager";
if (urgency === "medium") return "slack";
return "email";
```

**Why:** Critical alerts should go to pager (immediate, wakes people up). Low-priority notifications should go to email (async, non-disruptive). The original sends critical alerts to email and low-priority ones to pager.

---

### Bug 9: sentAt truthy coercion (line 61)

**Before:**
```typescript
return !!notification.sentAt;
```

**After:**
```typescript
return notification.sentAt !== null;
```

**Why:** `sentAt` is `number | null`. The truthy check `!!0` returns `false`, so a notification with `sentAt = 0` (epoch start, or a test/mock value) would be reported as unsent. The explicit null check handles all numeric values correctly.

---

## scheduleReport.ts — 3 Fixes

### Bug 10: Hours divides by 360000 instead of 3600000 (line 17)

**Before:**
```typescript
return totalMs / 360000;
```

**After:**
```typescript
return totalMs / 3600000;
```

**Or, using the constant from types.ts:**
```typescript
import { MS_PER_HOUR } from "./types";
// ...
return totalMs / MS_PER_HOUR;
```

**Why:** 1 hour = 3,600,000 ms. Missing a zero makes every result 10x too large. An 8-hour shift reports as 80 hours.

---

### Bug 11: Coverage exceeds 100% (line 47)

**Before:**
```typescript
const coveredHours = coveredMs / 3600000;
return (coveredHours / windowHours) * 100;
```

**After (simple clamp):**
```typescript
const coveredHours = coveredMs / 3600000;
return Math.min((coveredHours / windowHours) * 100, 100);
```

**After (proper deduplication — more correct but more complex):**
```typescript
// Merge overlapping shift intervals before summing
const intervals = shifts
  .map((s) => ({
    start: Math.max(s.startTime, windowStart),
    end: Math.min(s.endTime, windowEnd),
  }))
  .filter((i) => i.end > i.start)
  .sort((a, b) => a.start - b.start);

const merged: { start: number; end: number }[] = [];
for (const interval of intervals) {
  const last = merged[merged.length - 1];
  if (last && interval.start <= last.end) {
    last.end = Math.max(last.end, interval.end);
  } else {
    merged.push({ ...interval });
  }
}

const coveredMs = merged.reduce((sum, i) => sum + (i.end - i.start), 0);
const coveredHours = coveredMs / 3600000;
return Math.min((coveredHours / windowHours) * 100, 100);
```

**Why:** When primary + secondary shifts cover the same slot, both contribute to `coveredMs`. A 24-hour window with both a primary and secondary shift shows 200% coverage.

**Instructor note:** The simple `Math.min` clamp is usually the right answer in a demo. The full deduplication is more correct but introduces complexity. This is a good moment to discuss "minimal blast radius" — sometimes a clamp is enough.

---

### Bug 12: Gap detection ignores shift type (line 64)

**Before:**
```typescript
const sorted = [...shifts]
  .filter((s) => s.endTime > windowStart && s.startTime < windowEnd)
  .sort((a, b) => a.startTime - b.startTime);
```

**After:**
```typescript
const sorted = [...shifts]
  .filter((s) =>
    s.endTime > windowStart &&
    s.startTime < windowEnd &&
    (s.type === "primary" || s.type === "override")
  )
  .sort((a, b) => a.startTime - b.startTime);
```

**Why:** The docstring says "only primary and override shifts count as coverage; secondary shifts are backup and should not mask gaps." But the implementation doesn't filter by type, so a slot with only a secondary shift won't appear as a gap.

---

## Fix Summary

| # | File | Bug | Fix | Difficulty |
|---|------|-----|-----|------------|
| 1 | rotation.ts:18 | `<=` week boundary | Change to `<` | Easy |
| 2 | rotation.ts:38 | `!isNotAvailable` triple-negative | Remove `!` (or rename) | Easy-Med |
| 3 | rotation.ts:64 | `pop()` picks most-loaded | Use `sorted[0]` | Easy |
| 4 | availability.ts:33 | `\|\|` overlap check | Change to `&&` | Easy |
| 5 | availability.ts:46 | isNotApproved filter | Invert condition (or rename) | Easy |
| 6 | availability.ts:19 | Timezone offset backwards | Change `-` to `+` | Medium |
| 7 | notifications.ts:90 | Silent escalation failure | Add fallback path | Medium |
| 8 | notifications.ts:10 | Channel-urgency backwards | Swap email↔pager | Easy |
| 9 | notifications.ts:51 | sentAt truthy coercion | Use `!== null` | Easy |
| 10 | scheduleReport.ts:12 | `/360000` | Change to `/3600000` | Easy |
| 11 | scheduleReport.ts:38 | Coverage >100% | Add `Math.min` clamp | Easy-Med |
| 12 | scheduleReport.ts:53 | Gaps count secondary | Filter by shift type | Easy-Med |

---

## Bonus: Code Quality & Drift Issues

These aren't counted in the "12 bugs" but test whether a thorough review catches more than correctness. An AI review tool with high recall (see Qodo benchmark methodology) should surface these.

| Issue | File | What's wrong | Fix |
|-------|------|-------------|-----|
| Doc drift | rotation.ts | Docstring says "12-hour slots" and references RFC-2025-041; code uses 24h `dayMs` | Update docstring to match code (24-hour slots) |
| Doc drift | availability.ts | Docstring claims half-hour offset support (UTC+5:30); regex only matches whole hours | Either update regex to support half-hours or correct docstring |
| Phantom type | types.ts + notifications.ts | `"sms"` added to Notification channel union; `notificationSummary` `byChannel` initializer missing `sms: 0` — type error under `--strict` | Add `sms: 0` to the initializer, or remove `"sms"` from the union until routing exists |
| Orphan field | types.ts | `email?: string` added to TeamMember but only consumed by deprecated `sendSmsAlert` | Remove field, or add actual email-based notification routing |
| Dead code | scheduleReport.ts | `calculateOvertimeHours` exported but never called; references non-existent PayrollSync.ts; propagates the `/360000` bug | Delete function (or fix and integrate if payroll integration is real) |
| Unused import | scheduleReport.ts | `MS_PER_HOUR` imported but `calculateTotalHours` uses magic number `360000` instead | Replace `360000` with `MS_PER_HOUR` (also fixes the 10x bug) |
| Stale deprecation | notifications.ts | `sendSmsAlert` marked `@deprecated` with Q1 2026 removal target — it's now Q1 2026 | Execute the removal or update the target date |

**Instructor note:** These are the issues that separate a "fix the bugs" pass from a "leave the codebase better than you found it" pass. The adversarial review prompt (Prompt 12) should surface most of these. If it doesn't, that's a teaching moment about prompt specificity.

---

## Bonus: TypeScript Sins

These exercise the "policy guardrails" demo — showing how AI tools can instantly flag anti-patterns that humans nitpick in code review.

| Issue | File | What's wrong | Fix |
|-------|------|-------------|-----|
| Fragile enum | types.ts | `NotificationPriority` uses auto-numbered values; `chkPri` in notifications.ts does `n.priority > 1` — reordering enum members silently changes behavior | Use explicit enum values (`FYI = 0, ActionItem = 1, ...`) or compare by name (`n.priority >= NotificationPriority.Urgent`) |
| Over-engineered generic | types.ts | `ContactResolver<TResolver, TMember>` is a conditional type with two generic params; only one instantiation (`ContactResolver<"sync">`) is ever used anywhere in the codebase | Replace with the concrete type `{ memberId: string; escalateTo: string \| null }` and delete the generic |
| `any` abuse | notifications.ts | `sendSmsAlert` casts payload to `any`, then reads `payload.pri` (non-existent property); silently returns `undefined`, masked by `??` fallback | Type the payload properly; remove `any`; rename `.pri` to `.priority` or remove the dead field access |
| Unsafe array access | notifications.ts | `getNthMostRecent` returns `sorted[n]` without bounds check; return type says `Notification` but can be `undefined` at runtime | Add bounds check, or fix return type to `Notification \| undefined` |
| Circular dependency | rotation.ts ↔ scheduleReport.ts | `rotation.ts` imports `calculateTotalHours` from `scheduleReport.ts`; `scheduleReport.ts` imports `countMemberShifts` from `rotation.ts` | Extract shared utilities into a `utils.ts` or restructure the dependency graph so it's acyclic |
| Cryptic names | rotation.ts, notifications.ts, availability.ts | `chkPri`, `chkHrs`, `procTmAvail` — abbreviated function names that require reading the body to understand | Rename: `chkPri` → `isPageWorthy`, `chkHrs` → `isUnderHourCap`, `procTmAvail` → `isMemberSchedulable` |
| Single-letter params | availability.ts | `procTmAvail(m, s, r)` — meaningless parameter names on a public API | Rename: `m` → `member`, `s` → `shifts`, `r` → `timeOffRequests` |

**Instructor note:** These TypeScript sins are excellent for demonstrating how policy guardrails (`.cursor/rules`, project rules, system prompts) can catch style/naming issues on the first pass — freeing the human reviewer to focus on logic and architecture. The circular dependency is particularly evil because it *works* at runtime (Node.js hoists the modules) but creates fragile initialization ordering that breaks under refactoring.
