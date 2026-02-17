# Instruction Starter Pack (Cursor-First)

This pack mirrors a Cursor-first layered setup:

- `AGENTS.md` (team baseline)
- `AGENTS.local.md` (local-only personal overrides)
- Cursor project rules in `.cursor/rules/*.mdc`
- `CONTEXT-PICKUP-GUIDE.md` for loading behavior
- `YOLO-TROUBLESHOOT-BONUS.sh` for local approval/sandbox diagnostics

All examples are generic and reference the insurance lab only:

- `src/PolicyQuoteScreen.tsx`
- `src/eligibility.ts`
- `src/telemetry.ts`

## Extra AGENTS Templates

- `AGENTS.template.learning-example.md`
- `AGENTS.template.automatic-session-review.md`

Use these as drop-in starting points when you want explicit learning capture or a strict end-of-session review loop.

## Recommended Usage

1. Copy `AGENTS.md` and `.cursor/rules/*.mdc` into a repo as team defaults.
2. Keep `AGENTS.local.md` local-only (do not commit).
3. Edit commands, paths, and constraints to match your stack.

## Notes

- Keep team files short, specific, and testable.
- Keep personal flavor in local files only.
- Prefer scoped rules over one large global block.
