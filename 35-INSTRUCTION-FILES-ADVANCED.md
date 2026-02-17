# Instruction Files Advanced (AGENTS / CLAUDE / Local Variants)

This guide explains how to stay portable across toolchains that look for different instruction files.

## Naming Clarification

If your team says `clod.md` / `clodlocal.md`, treat that as shorthand for:

- `CLAUDE.md`
- `CLAUDE.local.md`

## Practical Compatibility Strategy

Use a layered approach:

1. `AGENTS.md` for broadly compatible agent instructions
2. `CLAUDE.md` as optional compatibility surface for Claude-oriented tooling
3. Local-only personal file (`AGENTS.local.md` or `CLAUDE.local.md`) excluded from git

## Recommended Source Of Truth

- Team policy: one canonical committed file (usually `AGENTS.md`)
- Optional compatibility: keep `CLAUDE.md` aligned to the canonical file
- Personal preferences: local-only file + local excludes

## Discovery Behavior (What To Expect)

Different tools may load:

- nearest `AGENTS.md` in directory tree
- repo-root `CLAUDE.md` / `GEMINI.md` alternatives
- project rules from `.cursor/rules/*.mdc`

Behavior varies by product/version, so optimize for redundancy without drift:

- Keep rules concise
- Avoid contradictory guidance between files
- Add a header note pointing to canonical source

## Drift Prevention Pattern

In compatibility files, add:

`Canonical policy lives in AGENTS.md. Keep this file aligned.`

## Local Personalization Pattern

Store local-only preferences in:

- `AGENTS.local.md` or `CLAUDE.local.md`
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
