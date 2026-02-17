# Other Agent Harnesses (Advanced, Optional)

This module is for teams that are primarily Cursor users but need compatibility with other agent harnesses.

If your team is Cursor-first, you can skip this on day one.

## Why this exists

- different tools load different instruction files
- compatibility can improve portability across teams and repos
- mixed harness setups can drift without a canonical source of truth

## Cursor-first default

Start with Cursor-native workflow and rules first:

- `02-NO-FLUFF-OPERATING-GUIDE.md`
- `10-WORKFLOW-FOUNDATIONS.md`
- `40-TEMPLATES/INSTRUCTION-STARTER-PACK/README.md`

Then add cross-tool compatibility only when needed.

## Recommended compatibility model

1. Keep one canonical team policy file (`AGENTS.md`).
2. Add compatibility surfaces (`CLAUDE.md`, Copilot instruction files) as thin shims.
3. Keep personal preferences local-only (`*.local.md`, local excludes).
4. Review for drift every 2-4 weeks.

## Claude Code insights workflow (optional)

- `/insights` can be used as a reflection surface for workflow tuning
- use it for pattern-finding, not automatic policy changes
- promote outputs only after evidence from shipped work

## What to read next

- `35-INSTRUCTION-FILES-ADVANCED.md` (AGENTS/CLAUDE compatibility)
- `75-GITHUB-COPILOT-CONFIG-ADVANCED.md` (Copilot layering)
- `40-TEMPLATES/INSTRUCTION-STARTER-PACK/CONTEXT-PICKUP-GUIDE.md` (loading behavior caveats)

## Guardrail

If compatibility work starts slowing delivery, pause and revert to Cursor-only baseline.
Add back compatibility one layer at a time.
