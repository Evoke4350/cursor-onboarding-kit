# Instruction Starter Pack (Generic)

This pack mirrors a layered setup:

- `AGENTS.md` (team baseline)
- `AGENTS.local.md` (local-only personal overrides)
- `CLAUDE.md` and `CLAUDE.local.md` compatibility files
- Cursor project rules in `.cursor/rules/*.mdc`
- GitHub Copilot instruction files in `.github/**`
- `SETUP-COMPAT-INSTRUCTIONS.sh` for fast mixed-tool setup
- `YOLO-TROUBLESHOOT-BONUS.sh` for local approval/sandbox diagnostics

All examples are generic and reference the insurance lab only:

- `src/PolicyQuoteScreen.tsx`
- `src/eligibility.ts`
- `src/telemetry.ts`

## Recommended Usage

1. Copy `AGENTS.md` and `.cursor/rules/*.mdc` into a repo as team defaults.
2. Keep `AGENTS.local.md` and `CLAUDE.local.md` local-only (do not commit).
3. Copy `.github/copilot-instructions.md` and `.github/instructions/*.instructions.md` for Copilot behavior.
4. Edit commands, paths, and constraints to match your stack.

## Claude Code Compatibility

Claude Code currently centers on `CLAUDE.md` memory files. If your team standard is `AGENTS.md`, use one of these bridges:

Option A (recommended): import `AGENTS.md` from `CLAUDE.md`

```md
# CLAUDE.md
@AGENTS.md
```

Option B: symlink

```bash
ln -s AGENTS.md CLAUDE.md
```

Use `CLAUDE.local.md` for personal project-specific preferences. Anthropic docs indicate this file is loaded as local memory and auto-added to `.gitignore`.

## Quick Setup

From repository root:

```bash
bash .agentic-blackboard/CURSOR-ONBOARDING-KIT/40-TEMPLATES/INSTRUCTION-STARTER-PACK/SETUP-COMPAT-INSTRUCTIONS.sh . import
```

If you prefer symlink mode:

```bash
bash .agentic-blackboard/CURSOR-ONBOARDING-KIT/40-TEMPLATES/INSTRUCTION-STARTER-PACK/SETUP-COMPAT-INSTRUCTIONS.sh . symlink
```

The script:
- sets `AGENTS.md` as canonical
- creates Claude bridge (`import` or `symlink`)
- updates `.git/info/exclude` for local-only files
- creates local starter files when missing

Bonus diagnostics:

```bash
bash .agentic-blackboard/CURSOR-ONBOARDING-KIT/40-TEMPLATES/INSTRUCTION-STARTER-PACK/YOLO-TROUBLESHOOT-BONUS.sh
```

## Notes

- Keep team files short, specific, and testable.
- Keep personal flavor in local files only.
- Prefer scoped rules over one large global block.
- In mixed-tool teams, pick one canonical source and bridge everything else to it.
