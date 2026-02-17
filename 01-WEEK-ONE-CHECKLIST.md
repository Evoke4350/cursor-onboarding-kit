# Week 1 Checklist: Cursor Onboarding (Fun + Real-World)

Use this as a practical week-one guide for new engineers shipping with Cursor.

## 🎯 Outcome by Friday

- [ ] Ship one reviewable PR with evidence.
- [ ] Use the discussion -> scope -> implement -> verify loop at least once.
- [ ] Keep personal preferences local-only (no repo noise).
- [ ] Leave a short note on what worked, what drifted, and what to keep.

---

## 🗓️ Day 1: Setup + Orientation

### Must-do

- [ ] Read `00-START-HERE.md` (quick runbook).
- [ ] Read `02-NO-FLUFF-OPERATING-GUIDE.md` (10-minute front door).
- [ ] Read `10-WORKFLOW-FOUNDATIONS.md` (daily loop).
- [ ] Read `03-EXPERIENCED-ENGINEER-LENS-QA.md` if you are skeptical or time-constrained.

### Environment and instruction setup

- [ ] Add team baseline instructions (`AGENTS.md`, Cursor rules).
- [ ] Add personal local files (`AGENTS.local.md`) if needed.
- [ ] Confirm local exclude setup (`.git/info/exclude`) for local-only files.
- [ ] Confirm you can run lint/typecheck/test commands for this repo.

### Quick confidence check

- [ ] Ask Cursor to restate one module's architecture in 5 bullets.
- [ ] Verify the summary against real files.

---

## 🧠 Day 2: Prompting Muscle Memory

### Run a discussion-first cycle

- [ ] Use: "Let's discuss first. No edits yet."
- [ ] Ask for: objective, non-goals, risks, done criteria.
- [ ] Lock scope before edits.

### Practice trigger phrases

- [ ] Try one planning trigger.
- [ ] Try one debug trigger.
- [ ] Try one question-tool trigger.

### Keep it small

- [ ] Select one medium-sized task only.
- [ ] Explicitly request minimal blast radius.

---

## 🛠️ Day 3: Implementation + Verification

### Implementation

- [ ] Execute only in-scope changes.
- [ ] Avoid unrelated refactors.
- [ ] Keep commit intent clear while working.

### Verification

- [ ] Run lint.
- [ ] Run typecheck.
- [ ] Run related tests.
- [ ] Note exact command results.

### Quality checks

- [ ] Confirm risk and rollback notes exist.
- [ ] Confirm no local-only files are staged.

---

## 🔍 Day 4: Review Loop + Drift Control

### Review behavior

- [ ] Ask for adversarial review ("what can break?").
- [ ] Ask for top 3 risks and missing tests.
- [ ] Ask for one simplification pass.

### Drift recovery drills

- [ ] Use: "Pause. You drifted scope. Re-ground and return revised plan only."
- [ ] If still drifting, reduce to one agent and one objective.

### Large context discipline

- [ ] For large JSON/docs, use REPL/script first.
- [ ] Ask for summary table before deep reads.

---

## 🚀 Day 5: Ship One PR

### PR packaging

- [ ] 3-bullet summary.
- [ ] Test checklist.
- [ ] Risk + rollback section.
- [ ] Explicit assumptions.

### Merge readiness

- [ ] Reviewer can understand the PR in <= 30 minutes.
- [ ] Commit history is human-readable.
- [ ] Evidence is included (not confidence-only language).

### Closeout note

- [ ] Create one short experiment note:
  - keep
  - modify
  - drop

---

## 🧩 Fast Prompt Pack (Copy/Paste)

### Discussion-first

`Let's discuss first. No edits yet. Restate objective, non-goals, risks, and done criteria in 5 bullets.`

### Scoped execution

`Implement this medium change with minimal blast radius. In-scope: <x>. Out-of-scope: <y>.`

### Verification closeout

`Return: files changed, commands run, command outcomes, risk notes, rollback plan.`

### Drift reset

`Pause. Scope drift detected. Re-ground in current objective and return revised plan only.`

### Structured question trigger

`Use the question tool and ask me one decision question before continuing.`

---

## ✅ Week 1 Self-Check

- [ ] I used discussion-first at least once.
- [ ] I locked scope before implementation.
- [ ] I used at least one mode trigger phrase.
- [ ] I ran verification commands before PR.
- [ ] I shipped one evidence-backed PR.
- [ ] I kept personal config out of commits.
- [ ] I used a drift reset prompt when needed.
- [ ] I produced risk + rollback notes.
- [ ] I used REPL/script for large payloads.
- [ ] I wrote one keep/modify/drop experiment note.

If most boxes are checked, you are ready for week-two advanced workflows.
