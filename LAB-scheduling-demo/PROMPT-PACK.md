# Prompt Pack: Team Scheduling Demo

13 prompts for the 3-act demo flow. Each prompt is copy-paste ready. Steering notes in *italics* are for the instructor, not part of the prompt.

---

## Act 1: "Let's Talk First"

### Prompt 0 (Anti-Pattern — run this first to set up contrast)

```
Fix all the bugs in this scheduling code.
```

*Let the model run. Show the audience the unfocused multi-file diff. Point out: no prioritization, no scope boundary, unreviewable. Then say: "Now let me show you how to actually do this."*

---

### Prompt 1: Discussion-First Triage

```
No edits yet. Read all five source files in src/.

Tell me what's likely broken. Bucket the issues into clusters (naming, logic, arithmetic, edge cases). For each cluster, list: file, line range, what's wrong, severity.

Then give me a prioritized fix order in 5 bullets — which cluster to fix first and why.
```

*Workflow concept: discussion-first. The model analyzes without touching code. Compare this output to the anti-pattern diff.*

---

### Prompt 2: Contract Definition

```
Based on the triage above, write a strict contract for fixing the rotation assignment bugs only.

Format:
- Objective (1 sentence)
- In-scope files and functions
- Out-of-scope (explicit)
- Done condition (measurable, not vibes)
- Risk notes
```

*Workflow concept: completion contract. This becomes the governing document for the next prompt.*

---

## Act 2: "Surgical Fixes"

### Prompt 3: Scoped Rotation Fix

```
Implement the rotation fixes defined in the contract above.

Scope: rotation.ts only. Minimal blast radius — fix the three bugs (week boundary, isNotAvailable filter, sort direction) without restructuring the file. If you need to touch other files, stop and ask first.

Show me what changed and why for each fix.
```

*Workflow concept: scoped execution, minimal blast radius. The diff should be 3 surgical edits, not a rewrite.*

---

### Prompt 4: Multi-File Boolean Rename

```
The negative boolean names `isNotAvailable` and `isNotApproved` in types.ts propagate confusion into every consuming file.

Rename:
- `isNotAvailable` → `isAvailable` (invert all usages)
- `isNotApproved` → `isApproved` (invert all usages)

Update types.ts, rotation.ts, availability.ts, and notifications.ts.

Critical constraint: behavior must be identical before and after. Show me a rename map (file, line, old expression → new expression) before making changes.
```

*Workflow concept: multi-file refactor with behavior-preservation constraint. This is the "wow moment" — the model traces 4 files and inverts every conditional. If it misses one, the verification step catches it.*

---

### Prompt 5: Verification Table

```
Build a 6-row scenario table for rotation assignment fairness.

Include these edge cases:
1. Shift landing exactly on midnight Monday (week boundary)
2. Member at maxShiftsPerWeek limit
3. Single-member team
4. All members unavailable
5. Member with 0 existing shifts (should be picked first)
6. Two members tied on shift count

Columns: Scenario | Input | Expected Result | Actual (with current bugs) | Pass/Fail

Use the FIXED code from Prompts 3-4, not the original buggy code.
```

*Workflow concept: verification table, evidence-based completion. The audience sees proof, not "looks good to me."*

---

### Prompt 6: Explorer Audit — Notifications

```
Run an audit pass on notifications.ts. No edits — this is findings only.

For each issue found, report:
- Line range
- What the code does
- What it should do
- Severity (silent-failure, incorrect-behavior, cosmetic)
- Suggested fix approach (1 sentence)

If you start editing code, you have failed this assignment.
```

*Workflow concept: explorer mode. The model produces a clean findings table without touching code. Show: this is how you'd brief a reviewer or a junior engineer.*

---

## Act 3: "Coordination and Delivery"

### Prompt 7: Parallel Suitability Check

```
Can I fix availability.ts and notifications.ts in parallel (two separate sessions), or do they share coupling that requires sequential fixes?

Binary answer first, then two concrete reasons.
```

*Workflow concept: parallel suitability. Quick decision moment — 30 seconds. The answer should be "mostly yes" with a caveat about the shared isNotAvailable type (already renamed in Prompt 4).*

---

### Prompt 8: Plan-Then-Build — Timezone Bug

```
The timezone offset in availability.ts toLocalTime() is applied backwards. This is a hard bug — timezone math is error-prone.

Give me a 2-step fix plan:
- Step 1: What to change and why (with before/after examples for UTC-5 and UTC+9)
- Step 2: How to verify the fix is correct

Do not write code yet. If you start coding before I approve the plan, you have failed this assignment.
```

*After the model produces the plan, review it aloud with the audience. Then:*

```
Plan approved. Execute step 1 only. Show what changed and why.
```

*Workflow concept: plan-then-build. The hardest bug gets the most structured treatment. The audience sees control, not just capability.*

---

### Prompt 9: Fix scheduleReport Bugs

```
Fix the three bugs in scheduleReport.ts:
1. calculateTotalHours divides by 360000 instead of 3600000
2. calculateCoverage can exceed 100% due to overlapping shifts
3. detectGaps counts secondary shifts as coverage (should be primary/override only)

Scope: scheduleReport.ts only. Minimal blast radius. Show each fix separately.
```

*Workflow concept: scoped execution. Three targeted fixes in one file. The coverage fix requires either interval deduplication or a Math.min clamp — the model should pick the simpler approach.*

---

### Prompt 10: Commit Curation

```
Assume all bugs are now fixed across all files. Propose 4 commits that tell a story a reviewer can follow.

Group by intent, not by file:
1. Correctness fixes (logic inversions, arithmetic)
2. Naming improvements (boolean renames)
3. Edge case hardening (silent failures, missing guards)
4. Reporting accuracy (coverage, gaps, hours)

For each commit: one-line message, files touched, what changed.

No git commands — just the grouping and narrative.
```

*Workflow concept: commit curation. Raw AI history is not review-friendly. This step shows how to curate it into a clean story.*

---

### Prompt 11: PR Draft

```
Draft a pull request for the full set of changes.

Format:
## Summary
(3 bullets max)

## Changes by commit
(reference the 4 commits from above)

## Test plan
(checklist, 5 items max)

## Risks and rollback
(2 bullets max)

Keep the entire PR readable in under 60 seconds.
```

*Workflow concept: PR discipline. The audience sees AI-heavy work packaged as a human-readable delivery artifact.*

---

### Prompt 12: Adversarial Review

```
Before I ship this, run a devil's-advocate pass.

Top 5 production failure risks for this scheduling system — not generic risk poetry. For each:
- Concrete failure scenario (what breaks, when, for whom)
- One stress test that would catch it
- One owner who should sign off

Be adversarial. I would rather hear hard truths now than get paged at 2 AM.
```

*Workflow concept: adversarial review. This is the "wow moment" where the AI does something most humans skip entirely.*

---

### Prompt 13: Cleanup and Reflection

```
We've completed the triage → fix → verify → package loop. Quick meta-reflection:

1. Which prompt structure produced the best outcome? Why?
2. Where did you (the AI) drift or need correction? What would have prevented it?
3. One sentence: what should I remember for next time?
```

*Workflow concept: meta-reflection. Optional closing beat — skip in fast demos. Shows the team that the workflow is self-improving.*

---

## Prompt Index

| # | Name | Act | Concept | Time |
|---|------|-----|---------|------|
| 0 | Anti-pattern | Setup | Contrast | 1 min |
| 1 | Discussion-first triage | 1 | Discussion-first | 2 min |
| 2 | Contract definition | 1 | Completion contract | 2 min |
| 3 | Scoped rotation fix | 2 | Scoped execution | 3 min |
| 4 | Multi-file boolean rename | 2 | Refactor, blast radius | 4 min |
| 5 | Verification table | 2 | Evidence-based completion | 3 min |
| 6 | Explorer audit | 2 | Explorer mode | 3 min |
| 7 | Parallel suitability | 3 | Parallel coordination | 1 min |
| 8 | Plan-then-build (timezone) | 3 | Plan mode | 4 min |
| 9 | Fix scheduleReport | 3 | Scoped execution | 3 min |
| 10 | Commit curation | 3 | Commit narrative | 2 min |
| 11 | PR draft | 3 | PR discipline | 2 min |
| 12 | Adversarial review | 3 | Devil's advocate | 2 min |
| 13 | Cleanup and reflection | 3 | Meta-reflection | 1 min |
