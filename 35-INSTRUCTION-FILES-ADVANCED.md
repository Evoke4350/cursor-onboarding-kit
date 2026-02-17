# Instruction Files Advanced (AGENTS + Local Variants)

This guide describes a Cursor-first instruction setup with one canonical policy file.

## Practical Strategy

Use a layered approach:

1. `AGENTS.md` as the single committed team policy
2. `.cursor/rules/*.mdc` for scoped behavior
3. `AGENTS.local.md` for local-only personal preferences

## Recommended Source Of Truth

- Team policy: one canonical committed file (`AGENTS.md`)
- Scoped rules: keep task or directory-specific constraints in `.cursor/rules/*.mdc`
- Personal preferences: local-only file + local excludes

## Discovery Behavior (What To Expect)

Cursor workflows typically rely on:

- nearest `AGENTS.md` in directory tree
- project rules from `.cursor/rules/*.mdc`

To reduce drift:

- Keep rules concise
- Avoid contradictory guidance between files
- Treat `AGENTS.md` as canonical

## Local Personalization Pattern

Store local-only preferences in:

- `AGENTS.local.md`
- `.cursor/local/**`
- private blackboard folders

Exclude them locally via `.git/info/exclude`.

## Nested AGENTS Files (Compression Pattern)

Use root instructions for global policy and leaf instructions for local deltas.

Recommended split:

- root `AGENTS.md`: non-negotiables, safety, repo-wide standards
- leaf `AGENTS.md`: only what differs for that subtree
- leaf file starts with: `Inherit root AGENTS.md policy; this file adds local overrides only.`

Keep leaf files compressed:

- 5-12 bullets max
- no repeated prose from root
- include only high-value, task-local constraints

## Compact Context Digest Format

When docs are newer than model training or very long, store compact digests near code and reference them from `AGENTS.md`.

Suggested shape:

```md
## Context Digest: <topic>
- source: <url or doc path>
- changed: <what changed in one line>
- do: <required behavior>
- avoid: <known failure mode>
- verify: <single command/check>
```

This keeps recency signals high and reduces drift.

## Columnar Encoding for Token Efficiency

For repeated config/rules data, use compact rows instead of paragraph prose.

Example:

```text
area|rule|why|verify
checkout|no silent fallback|prevents hidden failures|npm run test checkout
api-client|timeout 10s|limits hangs|unit:api-client-timeout
```

Use this for dense reference data, not narrative guidance.
