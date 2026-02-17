# Context Pickup Guide (Cursor-First)

This explains what gets loaded automatically, what gets loaded conditionally, and what needs explicit references.

## 1) Cursor rule loading

From Cursor docs (`cursor.com/docs/context/rules`):

- `.cursor/rules/*.mdc` supports `alwaysApply`, `globs`, and agent-decided modes.
- Rule content is included in model context when applied.
- Rule precedence: Team Rules -> Project Rules -> User Rules.
- `AGENTS.md` is supported as a simple markdown alternative, including nested support.

Writer's note:

- Static files do not magically apply unless connected through rule mode, references, or explicit reads.
- Keep high-signal constraints in always-apply or scoped globs.

## 2) Local instruction behavior

Recommended local pattern:

- Keep committed team policy in `AGENTS.md`.
- Keep personal preferences in `AGENTS.local.md`.
- Exclude local-only files via `.git/info/exclude`.

Writer's note:

- Keep one canonical source of truth for team behavior.
- Keep local taste out of shared policy unless it proves repeat value.

## 3) Why static reference docs still matter

Even if static docs are not always auto-injected, they still help because:

- they are discoverable via search/read workflows
- rules and prompts can explicitly point to them
- agents can be instructed to read them on trigger phrases or task types

## 4) Recommended pattern

1. Team baseline in `AGENTS.md` + `.cursor/rules/*.mdc`
2. Local taste in `AGENTS.local.md` excluded from git
3. Reference docs in blackboard with clear naming and lifecycle

## 5) Practical caveat

"Present in repo" is not equal to "always loaded."

To make behavior reliable:

- keep core instructions short
- make triggers explicit
- test with real prompts
- prune low-signal instructions regularly
