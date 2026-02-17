# AGENTS.md Template: Automatic Session Review

## Scope

Use this template to force an end-of-session review loop before any task is considered complete.

## Required Review Cycle

At the end of every implementation task, run this cycle in order:

1. Re-state requested scope in one sentence.
2. List files changed.
3. Run relevant checks and report exact commands.
4. Identify regressions, risks, and unverified assumptions.
5. Propose one follow-up task with smallest next value.

## Review Output Contract

Return the final response with these sections:

- `Scope Delivered`
- `Files Changed`
- `Validation Evidence`
- `Risks / Gaps`
- `Next Small Step`

## Failure Conditions (Must Not Ship)

- checks not run but claimed as passing
- scope drift without explicit note
- unresolved error logs ignored
- docs changed without behavioral verification

## Memory Update Rule

If the session produced a reusable rule, add it to `AGENTS.md` before handoff.
