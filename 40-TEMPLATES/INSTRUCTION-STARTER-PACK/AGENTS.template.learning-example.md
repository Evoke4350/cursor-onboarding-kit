# AGENTS.md Template: Learning Workshop Mode

## Provenance

This template is adapted from a shared AGENTS.md teaching pattern:

- https://gist.github.com/1cg/a6c6f2276a1fe5ee172282580a44a7ac

Use this as a lab/workshop starter. It also adapts well for student workflows.

## Scope

Use this template when you want the agent to accumulate durable repo knowledge while supporting hands-on implementation work.

## Rules

1. Read `README`, build config, and tests before proposing edits.
2. Keep changes narrow; one behavior theme per PR.
3. Never change tests just to force green.
4. When you discover a non-obvious repo fact, add it to `AGENTS.md` near the relevant section.
5. Prefer checkpoint-based guidance and verification steps over large speculative rewrites.

## What Counts As A Learning

- hidden file coupling
- runtime flags/env vars not obvious from docs
- misleading error that required a workaround
- command needed for reliable local verification
- setup pitfall that repeatedly slows execution

## Learning Entry Format

Use one compact bullet per learning:

- `area`: short scope label
- `fact`: non-obvious truth
- `evidence`: command, log, or file path
- `action`: what to do next time

## Workshop Notes

- Recommended for lab-style sessions where participants learn by shipping small increments.
- This template does not define assignment or exam policy because the current lab is not a formal course shell.

## Session Closeout

Before final response:

1. add new learnings to `AGENTS.md`
2. remove stale learnings replaced by better facts
3. include a short "Learned This Session" section in your handoff
