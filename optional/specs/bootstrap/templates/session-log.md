# Session Log Template
# Location: {{MEMORY:short}}/sessions/YYYYMMDD-HHMM-session.md
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
| HH:MM | idle | triage | [what prompted this] | [key details] |
| HH:MM | triage | scope | [what prompted this] | [key details] |

## Decisions Made

### Decision 1: [Title]
- **Question:** [What was being decided]
- **Options:** [What was considered]
- **Choice:** [What was chosen]
- **Rationale:** [Why]
- **Alternatives rejected:** [What/why not]

## Artifacts Created

- `{{MEMORY:long}}/YYYYMMDD-HHMM-title.md` - [description]
- `{{MEMORY:state}}/checkpoint.yaml` - [checkpoint state]

## Final State

- **Task:** [Final task status]
- **Position:** [Where work ended]
- **Phase:** [Final phase]
- **Blockers:** [Any remaining, or "None"]

## Session Summary

[1-2 sentences about what was accomplished]

---

## For Next Session

If resuming:
1. Read `{{MEMORY:state}}/checkpoint.yaml`
2. Pick up at [position]
3. Continue with [next step]
