# Session Log Template
# Location: sawdust/sessions/YYYYMMDD-HHMM-session.md
# Purpose: Immutable log for reconstruction and debugging

---
session_id: YYYYMMDD-HHMM
started: YYYY-MM-DDTHH:MM:SSZ
ended: null
agent: null
status: active|completed|interrupted
---

## Initial State

- **Task:** [What was being worked on when session started]
- **Position:** [File:line or description]
- **Phase:** [idle|triage|scope|implement|verify|deliver]

## State Transitions

| Time | From | To | Trigger | Notes |
|------|------|-----|---------|-------|
| HH:MM | idle | triage | User: "fix the bug" | Identified auth.rs |
| HH:MM | triage | scope | Bug: timing attack | Defined fix boundaries |
| ... | ... | ... | ... | ... |

## Decisions Made

### Decision 1: [Title]
- **Question:** What approach for the fix?
- **Options:** A) Add constant-time compare, B) Add rate limiting, C) Both
- **Choice:** A
- **Rationale:** Minimal change, addresses root cause
- **Alternatives rejected:** B (doesn't fix timing), C (overkill)

## Artifacts Created

- `shavings/YYYYMMDD-HHMM-timing-attack-insight.md`
- `sawdust/state/current-task.yaml` (checkpoint)

## Final State

- **Task:** [Final task status]
- **Position:** [Where work ended]
- **Phase:** [Final phase]
- **Blockers:** [Any remaining]

## Session Summary

[1-2 sentences about what was accomplished]

---

## For Next Session

If resuming:
1. Read `sawdust/state/current-task.yaml`
2. Pick up at [position]
3. Continue with [next step]
