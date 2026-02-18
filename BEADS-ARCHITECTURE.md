# Beads: Graph Issue Tracking for AI Agents

> Excerpt from "The Architecture of Open Source Applications" style chapter by Geoffrey Huntley

## The Agent Memory Problem

Ask an AI agent to build a feature. It creates files, writes tests, reports success. Next day you start a new session and ask it to fix a bug in that feature. It has no memory of what it built yesterday. It reads the code, guesses at intent, and breaks the tests it wrote 24 hours ago.

This is the **agent memory problem**. Context windows are finite. Chat histories are ephemeral. Markdown plans drift out of sync with reality the moment the first task completes.

Beads solves this with a **version-controlled database** (Dolt) that gives agents a persistent, structured memory in the form of a **dependency graph**. Instead of scribbling notes in markdown, an agent creates issues, links them with dependency edges, claims work, and marks tasks complete. The graph survives across sessions, across branches, and across multiple agents working in parallel.

## The Big Picture

The guiding metaphor: **issues chained together like beads on a string**. Each bead is a work item. The strings are dependency edges. The whole necklace is a directed acyclic graph (DAG).

| Component | Job |
|-----------|-----|
| **CLI Layer** | 80+ Cobra commands: create, update, close, ready, show, list, dep, compact, doctor |
| **Domain Types** | Issue, Dependency, Label, Comment, Event with validation and content hashing |
| **DoltStore** | SQL queries against Dolt with embedded and server modes |
| **Formulas** | Workflow templates in JSON/TOML that compile into issue hierarchies |
| **Molecules** | Hierarchical template catalog (built-in, user, project) |
| **Hooks** | Scripts that fire after create, update, close events |
| **Integrations** | Claude Code plugin, MCP server, Jira/Linear/GitLab adapters |

## Four Core Design Decisions

### 1. Version-Controlled Database (Dolt)

Most issue trackers use SQLite or PostgreSQL. But when three agents work on different branches simultaneously, their issue databases need to merge. Regular databases can't branch or merge.

Beads uses **Dolt** — git for SQL tables. Every INSERT/UPDATE/DELETE can be committed. Branch a database, make changes, merge back with cell-level merge resolution.

```bash
# Every write command produces a Dolt commit automatically
bd create "Add authentication"  # Creates issue + commits to Dolt
bd update bd-a3f --status done  # Updates + commits
```

Time-travel queries work via `bd vc log` and `bd show --as-of <commit>`.

### 2. Collision-Free IDs

When two agents create issues simultaneously on different branches, IDs must not collide. Auto-increment (`#1, #2, #3`) is the worst case. UUIDs work but are ugly and verbose.

Beads uses **adaptive hash length** based on the birthday paradox:

| ID Length | Good For Up To | Example |
|-----------|----------------|---------|
| 3 chars | ~160 issues | `bd-a1b` |
| 4 chars | ~980 issues | `bd-a1b2` |
| 5 chars | ~5,900 issues | `bd-a1b2c` |
| 6 chars | ~35,000 issues | `bd-a1b2c3` |

IDs are also hierarchical: `bd-a3f8` → `bd-a3f8.1` → `bd-a3f8.1.1`

### 3. The Dependency Graph

A flat todo list answers "what exists?" but not "what can I work on right now?"

Beads uses **typed dependency edges**:

**Workflow types** (affect `bd ready`):
- `blocks` — B cannot start until A is done
- `conditional-blocks` — B runs only if A fails (fallback workflows)
- `depends-on` — soft dependency for ordering

**Association types** (knowledge graph):
- `replies-to` — conversation threading
- `duplicates` — link identical work
- `supersedes` — version chains
- `tracks` — cross-project references

**Entity types** (HOP system):
- `authored-by` — who created it
- `approved-by` — who validated it
- `attests` — skill certification

### 4. Compaction: Graceful Forgetting

A 6-month project accumulates thousands of closed issues. Each has description, design notes, comments, audit trail. This bloats agent context windows.

**Compaction** summarizes old closed issues while preserving graph structure:

- **Tier 1** (30+ days): 70% text reduction, preserve dependencies/labels
- **Tier 2** (90+ days): 95% reduction, just identifier line

Critical: **dependencies survive compaction**. The graph structure is too valuable to lose.

```bash
bd compact --analyze --json  # Get candidates
bd compact --apply --id bd-42 --summary summary.txt
```

## The Most Important Command

```bash
bd ready
```

This answers "what should I work on next?" by finding all open issues with no open blocking dependencies. Default sort: recent issues by priority, older issues by age (prevent starvation).

Excludes by default: in-progress, deferred, ephemeral wisps, template molecules.

## Agent-as-Bead

Agents themselves are modeled as issues in the graph they manage. An agent bead tracks:
- Current state (idle, spawning, running, stuck, done, dead)
- Last activity timestamp
- Current work item ("hook bead")

```bash
bd agent state gt-claude running
bd agent heartbeat gt-claude
bd agent show gt-claude
```

The **Witness** monitors heartbeats and marks agents as `dead` if they don't report in, enabling automated recovery.

## Formulas and Molecules

A formula is a workflow template:

```json
{
  "formula": "mol-feature",
  "vars": { "component": { "required": true } },
  "steps": [
    {"id": "design", "title": "Design {{component}}"},
    {"id": "implement", "title": "Implement {{component}}", "depends_on": ["design"]},
    {"id": "test", "title": "Test {{component}}", "depends_on": ["implement"]},
    {"id": "review", "title": "Review {{component}}", "depends_on": ["test"]}
  ]
}
```

```bash
bd pour mol-feature --var component=auth
# Creates 4 linked issues: design → implement → test → review
```

**Advice rules** (AOP-style) let you weave in cross-cutting concerns:

```json
{
  "formula": "security-audit",
  "type": "aspect",
  "advice": [{
    "target": "*.implement",
    "after": {"id": "security-review-{step.id}", "title": "Security review"}
  }]
}
```

## Swarm Analysis

For epics with many subtasks:

```bash
bd swarm analyze <epic-id>
```

Computes **waves** — sets of tasks that can run in parallel. Tells you maximum parallelism: if max is 3, spinning up 10 agents is wasteful.

## Lessons Learned

1. **Version-controlled data beats version-controlled files** — The database IS the artifact
2. **Hash IDs with math beat auto-increment** — No coordination needed
3. **Forgetting is a feature** — Preserve graph structure, discard free text
4. **The graph is the API** — `bd ready` is the core query, everything else supports it
5. **Design for agents first, humans second** — Every command supports `--json`

## Common Commands Reference

```bash
# What should I work on?
bd ready

# Create structured work
bd create "Feature" --type epic
bd create "Subtask" --parent <epic-id>

# Link with dependencies
bd dep add <blocking-id> <blocked-id>  # A blocks B

# Claim work atomically
bd update <id> --claim gt-claude

# Track agent state
bd agent state gt-claude running
bd agent heartbeat gt-claude

# Session end
bd sync && git push
```

---

*Source: [latentpatterns.com/patterns/beads-graph-tracker](https://latentpatterns.com/patterns/beads-graph-tracker)*
