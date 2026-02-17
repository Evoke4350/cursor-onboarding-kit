# AGENTS.md Template: Learning Example

## Scope

Use this template when you want the agent to accumulate durable repo knowledge from real work.

## Rules

1. Read `README`, build config, and tests before proposing edits.
2. Keep changes narrow; one behavior theme per PR.
3. Never change tests just to force green.
4. When you discover a non-obvious repo fact, add it to `AGENTS.md` near the relevant section.

## What Counts As A Learning

- hidden file coupling
- runtime flags/env vars not obvious from docs
- misleading error that required a workaround
- command needed for reliable local verification

## Learning Entry Format

Use one compact bullet per learning:

- `area`: short scope label
- `fact`: non-obvious truth
- `evidence`: command, log, or file path
- `action`: what to do next time

## Session Closeout

Before final response:

1. add new learnings to `AGENTS.md`
2. remove stale learnings replaced by better facts
3. include a short "Learned This Session" section in your handoff
