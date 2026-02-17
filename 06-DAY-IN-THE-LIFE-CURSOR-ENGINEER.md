# Day in the Life (9:00-5:00)

This guide shows what a full workday can look like using this workflow.

Use it as an operating rhythm, not a rigid schedule.

---

## Shared Rules (All Levels)

- One ticket, one objective, one clear done condition.
- Discussion-first for ambiguous work.
- Backpressure stays on (lint, typecheck, tests, review).
- Evidence over confidence language.
- If drift appears, pause and re-scope before continuing.

---

## Version A: Junior Engineer (First 1-3 Months)

### 9:00-9:30 - Intake + Scope Lock

- Read ticket and affected files.
- Ask agent to restate objective, non-goals, risks.
- Confirm in-scope/out-of-scope.

### 9:30-11:00 - Build Slice 1

- Request one medium change only.
- Keep blast radius minimal.
- Ask for assumptions before edits if unclear.

### 11:00-11:30 - Verify Early

- Run lint/typecheck/tests.
- Record failures and decide: fix now or reduce scope.

### 11:30-12:00 - PR Draft Notes (Early)

- Write first pass summary from evidence.
- Add known risks and rollback idea.

### 1:00-2:30 - Build Slice 2 (or Fixes)

- Finish in-scope implementation.
- Avoid unrelated cleanup.

### 2:30-3:00 - Adversarial Pass

- Ask agent for top 3 failure modes.
- Add missing tests or notes.

### 3:00-4:00 - Review Prep

- Re-run checks.
- Tighten PR summary and test plan.

### 4:00-5:00 - Submit + Learn

- Open PR.
- Capture keep/modify/drop note for tomorrow.

---

## Version B: Senior/Lead Engineer

### 9:00-9:20 - Portfolio Triage

- Choose 1 primary ticket + 1 fallback task.
- Decide if task is single-agent or orchestrated.

### 9:20-10:00 - Plan Layer

- Discussion-first prompt.
- Lock constraints and acceptance criteria.
- If cross-domain, split into clear sub-tasks.

### 10:00-12:00 - Parallelized Execution (When Justified)

- Run independent slices in parallel only.
- Keep one owner voice for integration decisions.

### 1:00-2:00 - Integration + Quality Gate

- Merge slices conceptually.
- Run hard gates.
- Resolve conflicting outputs.

### 2:00-3:00 - Backpressure + Design Debt Check

- Decide if a template artifact is needed:
  - RFC
  - runbook
  - handoff
  - decision record

### 3:00-4:00 - Reviewability Pass

- Curate commit narrative for humans.
- Ensure rollback and observability notes exist.

### 4:00-5:00 - Team Throughput Enablement

- Promote one proven local pattern to team docs.
- Prune one low-signal instruction.

---

## Version C: Bad Day (Drift, Failing Tests, Confusion)

When things go sideways, run this instead of pushing harder.

### 9:00-9:20 - Stop the Bleed

- Freeze scope expansion.
- Pick one failing path only.

### 9:20-10:00 - Reset Prompt

- "No edits yet. Restate current objective, failing behavior, and next smallest fix."

### 10:00-11:30 - Reproduce + Isolate

- Reproduce failure.
- Gather runtime evidence.
- Avoid broad refactors.

### 11:30-12:00 - Decision Gate

- If still unstable: de-scope and ship smaller.
- If stable: continue with one fix path.

### 1:00-2:30 - Single-Agent Recovery

- Remove parallel agents and extra memory layers.
- Use explicit contracts only.

### 2:30-3:30 - Re-verify

- Run full relevant checks.
- Validate rollback path.

### 3:30-5:00 - Safe Closeout

- Ship reduced scope PR or open technical debt follow-up.
- Document what caused drift and how it was resolved.

---

## Version D: Backpressure Time -> Artifact Time

Use blocked time to strengthen long-term flow.

Trigger conditions:

- waiting on review
- blocked on external dependency
- repeated drift on same class of work
- recurring handoff confusion

Do one of these:

- create/update RFC from template
- create/update runbook for recurring issue
- create handoff doc for cross-team work
- create decision record for important trade-off

Templates:

- `40-TEMPLATES/TEMPLATE-tech-writer-rfc.md`
- `40-TEMPLATES/TEMPLATE-runbook.md`
- `40-TEMPLATES/TEMPLATE-handoff.md`
- `40-TEMPLATES/TEMPLATE-decision-record.md`

---

## Semantic Delimiter Pattern (Automation-Friendly)

Use semantic delimiters so agents can parse intent and constraints predictably.

Example:

```md
[OBJECTIVE]
Ship PR-ready fix for <ticket>.

[IN_SCOPE]
- file/path A
- file/path B

[OUT_OF_SCOPE]
- refactors
- dependency upgrades

[CONSTRAINTS]
- minimal blast radius
- no test edits unless requested
- preserve telemetry contract semantics

[EVIDENCE_REQUIRED]
- commands run + outcomes
- risks
- rollback

[DELIVERABLES]
- code changes
- PR summary
- follow-up tasks
```

Why this helps:

- improves agent parsing of task boundaries
- reduces instruction collision
- makes delegation safer when pointing agents at specific files
