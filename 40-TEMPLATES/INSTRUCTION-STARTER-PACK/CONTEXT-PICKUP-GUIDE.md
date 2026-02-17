# Context Pickup Guide (Static Files + Rules)

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

## 2) GitHub Copilot repository instructions

From GitHub docs (`docs.github.com/.../add-repository-instructions`):

- `.github/copilot-instructions.md` = repo-wide instructions
- `.github/instructions/*.instructions.md` with `applyTo` = path/file scoped instructions
- If both match, both are used together
- `AGENTS.md` is supported by Copilot agents, nearest file in tree can take precedence

Writer's note:

- Put global baseline in one short file
- Put path-specific conventions in small `.instructions.md` files

## 3) VS Code custom instruction behavior

From VS Code docs (`code.visualstudio.com/.../custom-instructions`):

- `.github/copilot-instructions.md` is always-on in workspace
- `AGENTS.md` and `CLAUDE.md` are supported as always-on instruction files
- Multiple instruction files can be combined; no strict order guaranteed in some contexts
- Nested `AGENTS.md` is experimental and controlled by settings

Writer's note:

- Avoid contradictory guidance across files
- Keep one canonical source and mirror compatibility files lightly

## 4) Claude Code memory model (important)

From Anthropic docs (`docs.claude.com/en/docs/claude-code/memory`):

- Claude Code uses `CLAUDE.md` memory, not `AGENTS.md` as native standard.
- `CLAUDE.local.md` is project-local personal memory and is auto-added to `.gitignore`.
- Memory files are loaded recursively from cwd upward (`CLAUDE.md` and `CLAUDE.local.md`).
- More specific memory has higher precedence.
- Claude auto-memory also exists (`~/.claude/projects/.../memory/`) and only first 200 lines of `MEMORY.md` are preloaded.

Writer's note:

- If your shared standard is `AGENTS.md`, Claude Code needs a bridge file.
- Keep personal preferences in `CLAUDE.local.md` to avoid polluting team policy.

### Bridge patterns for cross-tool consistency

Pattern A (recommended): import once in `CLAUDE.md`

```md
# CLAUDE.md
@AGENTS.md
```

Pattern B: symlink for strict single-source setups

```bash
ln -s AGENTS.md CLAUDE.md
```

Notes:

- Import pattern is usually easier to reason about than symlink chains.
- Avoid duplicating identical content in both files to reduce context waste.
- Keep Claude-specific instructions in `CLAUDE.md` only when needed.

## 5) Why static reference docs still matter

Even if static docs are not always auto-injected, they still help because:

- they are discoverable via search/read workflows
- rules and prompts can explicitly point to them
- agents can be instructed to read them on trigger phrases or task types

## 6) Recommended pattern

1. Team baseline in `AGENTS.md` + `.cursor/rules/*.mdc`
2. Local taste in `AGENTS.local.md` / `CLAUDE.local.md` excluded from git
3. Copilot baseline in `.github/copilot-instructions.md`
4. Path-specific behavior in `.github/instructions/*.instructions.md`
5. Reference docs in blackboard with clear naming and lifecycle

## 7) Practical caveat

“Present in repo” is not equal to “always loaded.”

To make behavior reliable:

- keep core instructions short
- make triggers explicit
- test with real prompts
- prune low-signal instructions regularly
