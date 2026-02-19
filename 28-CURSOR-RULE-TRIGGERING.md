# Cursor Rule Semantic Triggering

---

Cursor rules use semantic triggering. A rule is included in the conversation when its activation conditions are met.

## Activation Conditions

A rule activates when any of these are true:

1. `alwaysApply: true` — Rule is always included
2. Glob match + file in context — A file matching the rule's `globs` pattern is open, referenced, or in context
3. **Apply Intelligently** — Rule has no `globs`; the agent may include it when the rule is relevant (e.g. description matches the task). See *27-CURSOR-META-EXTRACTION.md* → Rule Anatomy.

## Example Rules

### Styling Rule (File-Triggered)

```yaml
---
description: CSS and Tailwind conventions
globs: "**/*.css, **/tailwind.config.*"
alwaysApply: false
---
```

Activates when: `app/globals.css`, `tailwind.config.js`, or any matching file is open or referenced.

### Types Rule (Directory-Triggered)

```yaml
---
description: TypeScript type definitions
globs: "**/types/**/*, **/*.d.ts"
alwaysApply: false
---
```

Activates when: Any file under `types/` or any `*.d.ts` file is open or referenced.

## Open Files

Cursor tracks open files in the editor. When a rule has `globs` defined:

1. Cursor checks if any open file matches the pattern
2. If yes, the rule is injected into context
3. If no, the rule is dormant

Opening a file changes which rules are active.

## Explicit Reference

Naming a file in the request activates matching rules:

```
"Update types/images.d.ts"
```

This activates any rule with `globs` matching `types/images.d.ts`, even if the file isn't open.

## Widening Scope

Widen globs to activate more often:

| Current | Widened | Effect |
|---------|---------|--------|
| `**/*.d.ts` | `**/*.ts` | Any TypeScript file |
| `**/types/**/*` | `**/*.ts, **/*.tsx` | All TS/TSX |
| `**/*.css` | `**/*.{css,scss}` | CSS and SCSS |

## Narrowing Scope

- Use specific paths: `src/components/**/*.tsx`
- Use `alwaysApply: false` (default)
- Avoid `**/*`

## Interaction with alwaysApply

| `alwaysApply` | `globs` | Behavior |
|---------------|---------|----------|
| `true` | Any | Always active (globs ignored for activation) |
| `false` | Present | Active when glob matches open/referenced file |
| `false` | Absent | Active when agent selects as relevant (Apply Intelligently); otherwise not included |

## Implications

1. Open files affect which rules are active
2. Naming a file activates its rules
3. Glob design is activation design
4. Context carries forward — rules stay active while the file is in context

## Anti-Patterns

- `**/*` negates semantic triggering
- `alwaysApply: true` with narrow globs is redundant
- Assuming rules with no globs are "never active" — they can still activate via **Apply Intelligently**; add globs when you need deterministic, file-based activation

## References

- https://cursor.com/docs/context/rules
- `27-CURSOR-META-EXTRACTION.md`
