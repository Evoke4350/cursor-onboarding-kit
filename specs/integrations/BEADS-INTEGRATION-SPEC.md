# Spec: Beads Integration (Optional)

**Status:** Optional integration - not a dependency
**Date:** 2026-02-18

---

## Purpose

Document how the Agentic Blackboard could integrate with beads (graph issue tracker) for users who want persistent agent memory across sessions. This is **optional** - the blackboard works standalone.

---

## What Beads Provides

| Feature | Description |
|---------|-------------|
| **Dolt storage** | Version-controlled SQL database (git for data) |
| **Typed dependencies** | 19 edge types (blocks, assigned-to, authored-by, etc.) |
| **Formulas** | Workflow templates that spawn linked issue hierarchies |
| **Agent-as-bead** | Agents modeled as issues in their own graph |
| **Compaction** | Tiered summarization of old issues |
| **Swarm analysis** | Compute maximum parallelism for epics |

---

## Integration Points

### 1. Workshop CLI → Beads

```bash
# Optional: link shavings to beads
workshop cut src/auth.rs --link-bead
# Creates shaving + bead with discovered-from edge

# Optional: search includes bead IDs
workshop carve "security" --beads
# Returns file paths AND related bead IDs
```

### 2. Templates → Formulas

Templates in `40-TEMPLATES/` could convert to beads formulas:

```json
{
  "formula": "mol-bugfix",
  "steps": [
    {"id": "discuss", "title": "Discuss: {{bug}}"},
    {"id": "contract", "title": "Contract", "depends_on": ["discuss"]},
    {"id": "fix", "title": "Fix", "depends_on": ["contract"]},
    {"id": "verify", "title": "Verify", "depends_on": ["fix"]},
    {"id": "deliver", "title": "Deliver", "depends_on": ["verify"]}
  ]
}
```

### 3. Agent Lifecycle

If using beads for agent tracking:

```bash
# Session start
bd agent create gt-claude --role implementer
bd agent heartbeat gt-claude  # periodic

# Work assignment
bd dep add bd-task gt-claude assigned-to

# Session capture
workshop capture-session --agent gt-claude
```

### 4. Dependency Types

Beads supports 19 typed edges. Relevant for blackboard:

| Type | Usage |
|------|-------|
| `blocks` | Task B waits for Task A |
| `assigned-to` | Agent owns task |
| `authored-by` | Who created it |
| `discovered-from` | Source of insight |
| `supersedes` | New version replaces old |
| `tracks` | Cross-reference (non-blocking) |

---

## Implementation (If Desired)

### Phase 1: CLI Integration

1. Add `--bead` flag to workshop commands
2. Search returns bead IDs alongside file paths
3. Optional bead ID in shaving frontmatter

### Phase 2: Formula Conversion

1. Script to convert templates to formulas
2. `workshop template-to-formula` command
3. Formula catalog in `~/.workshop/molecules/`

### Phase 3: Agent Tracking

1. `workshop agent` subcommand
2. Heartbeat integration with session capture
3. `bd ready` shows available work

---

## Why Optional

- Beads requires Dolt (large dependency)
- Not all users need graph issue tracking
- Blackboard works fine with flat files + git
- Adding beads as hard dependency limits audience

---

## Alternative: File-Based Graph

If graph structure is needed without beads:

```
.workshop/
├── graph/
│   ├── nodes.jsonl      # {id, type, title, status, ...}
│   └── edges.jsonl      # {from, to, type, created}
```

Simple, portable, git-friendly. No Dolt required.

---

## References

- Beads source: `/Users/nateb/beads/`
- Dependency types: `BEADS-DEPENDENCY-TYPES.md`
- Architecture article: `BEADS-ARCHITECTURE.md`
