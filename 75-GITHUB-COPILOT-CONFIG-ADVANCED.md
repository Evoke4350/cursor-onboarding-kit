# GitHub Copilot Configuration (Advanced, Generic)

Use this when you want Copilot behavior to mirror your Cursor instruction strategy.

## Recommended File Layout

- `.github/copilot-instructions.md` (repository-wide baseline)
- `.github/instructions/*.instructions.md` (path-specific rules)
- `AGENTS.md` (agent instruction compatibility)

## Path-Specific Instructions

Each `.instructions.md` file can use frontmatter:

```md
---
applyTo: "**/*.ts,**/*.tsx"
---
```

You can split by domain:

- `frontend.instructions.md`
- `api.instructions.md`
- `tests.instructions.md`

## Suggested Layering

1. Repository-wide baseline: quality gates, architecture orientation, command order
2. Path-specific files: high-friction directories and conventions
3. Agent file(s): execution behavior and guardrails

## Personalization Without Repo Noise

Keep personal prompt preferences local:

- local prompt files (if enabled in IDE)
- local instruction notes excluded via `.git/info/exclude`

Promote to team instructions only after repeated value.

## Rollout Plan

1. Start with one short repository-wide instruction file.
2. Add one path-specific file for a known pain area.
3. Track review outcomes for 2-4 weeks.
4. Prune verbose/low-signal instructions.

## Common Failure Modes

- Too much prose (models ignore long documents)
- Conflicting instructions across files
- No ownership/cadence for updates
- Mixing personal style with team policy

## Verify It Is Working

- Confirm instruction files are referenced in Copilot interactions where supported.
- Compare before/after PR quality (review noise, defect escapes, rework).

## Read More

- GitHub docs: adding repository custom instructions  
  https://docs.github.com/en/copilot/how-tos/configure-custom-instructions/add-repository-instructions?tool=vscode
- GitHub docs: custom instruction tutorials  
  https://docs.github.com/en/copilot/tutorials/use-custom-instructions
