# Beads Dependency Types Reference

Full list of typed dependency edges from `/Users/nateb/beads/internal/types/types.go`.

---

## Workflow Types (Affect `bd ready`)

These types directly affect what shows up in `bd ready`:

| Type | Usage | Description |
|------|-------|-------------|
| `blocks` | `bd dep add A B blocks` | B cannot start until A is done (hard blocker) |
| `conditional-blocks` | `bd dep add A B conditional-blocks` | B runs only if A fails (fallback workflows) |
| `waits-for` | `bd dep add parent child waits-for` | Fanout gate: wait for dynamic children |
| `parent-child` | `bd dep add parent child parent-child` | Hierarchical relationship |

## Association Types (Knowledge Graph)

Non-blocking edges that create connections:

| Type | Usage | Description |
|------|-------|-------------|
| `related` | `bd dep add A B related` | Generic association |
| `relates-to` | `bd dep add A B relates-to` | Loose knowledge graph edge |
| `replies-to` | `bd dep add comment issue replies-to` | Conversation threading |
| `duplicates` | `bd dep add A B duplicates` | Deduplication link |
| `supersedes` | `bd dep add new old supersedes` | Version chain (new replaces old) |
| `discovered-from` | `bd dep add finding source discovered-from` | Audit trail origin |

## Entity/Attribution Types (HOP System)

Hierarchical Ownership Protocol edges:

| Type | Usage | Description |
|------|-------|-------------|
| `authored-by` | `bd dep add issue agent authored-by` | Creator relationship |
| `assigned-to` | `bd dep add issue agent assigned-to` | Assignment relationship |
| `approved-by` | `bd dep add issue reviewer approved-by` | Approval relationship |
| `attests` | `bd dep add attestor target attests` | Skill attestation: X certifies Y has skill Z |
| `validates` | `bd dep add test code validates` | Approval/validation relationship |

## Control Flow Types

| Type | Usage | Description |
|------|-------|-------------|
| `tracks` | `bd dep add convoy issue tracks` | Cross-project reference (non-blocking) |
| `until` | `bd dep add A B until` | Active until target closes (e.g., muted until issue resolved) |
| `caused-by` | `bd dep add bug change caused-by` | Triggered by target (audit trail) |
| `delegated-from` | `bd dep add subtask parent delegated-from` | Work delegated from parent; completion cascades up |

---

## Which Types Affect `bd ready`?

From the source code, these types block work:

```
blocks          - Hard blocker
conditional-blocks - Conditional (only if source fails)
waits-for       - Fanout gate
```

All other types are non-blocking knowledge graph edges.

---

## Usage Examples

### Basic blocking workflow
```bash
bd create "Design auth"
bd create "Implement auth"
bd dep add bd-design bd-implement blocks
```

### Fallback workflow (conditional)
```bash
bd create "Try approach A"
bd create "Try approach B"
bd dep add bd-a bd-b conditional-blocks  # B runs only if A fails
```

### Agent attribution
```bash
bd agent create gt-claude
bd dep add bd-issue gt-claude authored-by
bd dep add bd-issue gt-claude assigned-to
```

### Cross-project tracking
```bash
bd dep add bd-local external:other-project:capability tracks
```

### Skill attestation
```bash
bd create "Migration expert" --type agent
bd dep add gt-expert gt-junior attests  # expert certifies junior's skill
```

---

## Current CLI Support

The `bd dep add` command accepts `--type` with these values:
- `blocks` (default)
- `related`
- `parent-child`
- `discovered-from`

However, the underlying type system supports any string. You can use custom types:
```bash
bd dep add A B --type custom-relationship
```
