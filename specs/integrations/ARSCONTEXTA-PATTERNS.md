# Extracted Patterns: arscontexta → Blackboard

**Source:** `/Users/nateb/arscontexta/`
**Purpose:** Knowledge system patterns including the Ralph Loop

---

## The Ralph Loop Pattern

The core insight: **Fresh context per phase prevents context pollution.**

### Problem
When processing multiple tasks in sequence, later tasks run on degraded attention. The LLM's context accumulates noise, and quality drops.

### Solution
Spawn a subagent for each phase. The lead session only orchestrates - it never executes tasks directly.

```
Lead Session (orchestration only)
|
+-- Phase A: Subagent processes task 1 with fresh context
+-- Phase B: Subagent processes task 1's next phase with fresh context
+-- Phase C: Subagent processes task 2 with fresh context
...
```

### Implementation

```markdown
# /ralph N [--parallel] [--batch id] [--type phase]

Process N tasks from queue with fresh context per phase.

## Arguments
- N: number of tasks to process
- --parallel: concurrent workers (max 5)
- --batch [id]: process only tasks from specific batch
- --type [phase]: process only tasks at a specific phase
- --dry-run: show what would execute

## Mandatory Constraint

**You MUST use the Task tool to spawn a subagent for EVERY task.**

The lead session's ONLY job is:
1. Read queue
2. Spawn subagent
3. Evaluate return
4. Update queue
5. Repeat
```

---

## Phase Configuration

Each knowledge item goes through phases:

| Phase | Purpose | Output |
|-------|---------|--------|
| **extract** | Extract insights from source | Task entries in queue |
| **create** | Write the insight file | `shavings/YYYYMMDD-title.md` |
| **reflect** | Find connections | Wiki links, topic maps |
| **reweave** | Update older items | Backlinks from old to new |
| **verify** | Quality check | Pass/fail validation |

---

## Handoff Protocol

Between phases, subagents output structured handoff blocks:

```
=== RALPH HANDOFF: {phase} ===
Target: {task}

Work Done:
- [what was accomplished]

Learnings:
- [Friction]: {description} | NONE
- [Surprise]: {description} | NONE
- [Methodology]: {description} | NONE

Queue Updates:
- [status changes]
=== END HANDOFF ===
```

This enables:
- Lead session captures learnings across tasks
- Queue state stays synchronized
- Resumability after interruption

---

## Queue Schema

```yaml
phase_order:
  claim: [create, reflect, reweave, verify]
  enrichment: [enrich, reflect, reweave, verify]

tasks:
  - id: source-name-001
    type: claim
    status: pending
    target: "Insight title"
    batch: source-name
    file: source-name-001.md
    current_phase: reflect
    completed_phases: [create]
    created: "2026-02-18T10:00:00Z"
```

---

## Cross-Connect Pattern

After all tasks in a batch complete, run cross-connect validation:

1. Collect all created items
2. Verify sibling connections exist
3. Add any connections missed because siblings didn't exist yet

```bash
# Post-batch validation
ralph-cross-connect --batch {batch-id}
```

---

## Parallel Processing

For batches with multiple independent items:

```
Phase A: Parallel workers (max 5 concurrent)
|   +-- worker-001: all 4 phases for item 001
|   +-- worker-002: all 4 phases for item 002
|   +-- worker-003: all 4 phases for item 003
|
Phase B: Cross-connect validation (one pass)
```

Workers receive sibling awareness upfront so they can link proactively.

---

## Mapping to Blackboard

| arscontexta | Blackboard | Notes |
|-------------|------------|-------|
| `notes/` | `shavings/` | Knowledge items |
| `ops/queue/` | `sawdust/queue/` | Processing queue |
| `topic-maps/` | `bench/mocs/` | Maps of content |
| `/reduce` | `/cut` | Extract from source |
| `/reflect` | `/carve` | Find connections |
| `/reweave` | `/chamfer` | Update older items |
| `/verify` | `/check` | Validate structure |

---

## Vocabulary Transformation

arscontexta uses domain derivation to customize terminology:

```yaml
# derivation-manifest.md
vocabulary:
  note: "shaving"
  notes: "shavings"
  topic_map: "moc"
  reduce: "mill"
  reflect: "join"
  reweave: "chamfer"
  verify: "check"
```

This allows the same skills to work with different naming schemes.

---

## Skills to Port

From `/Users/nateb/arscontexta/platforms/shared/skill-blocks/`:

| Skill | Purpose | Priority |
|-------|---------|----------|
| `ralph.md` | Queue processing with fresh context | HIGH |
| `pipeline.md` | End-to-end source processing | HIGH |
| `seed.md` | Create extract task from source | MEDIUM |
| `reduce.md` | Extract insights from source | MEDIUM |
| `reflect.md` | Find connections | MEDIUM |
| `reweave.md` | Update older items | MEDIUM |
| `verify.md` | Quality validation | MEDIUM |
| `remember.md` | Capture learnings | LOW |
| `rethink.md` | Meta refinement | LOW |

---

## Implementation Priority

1. **Phase 1: Queue + Ralph**
   - Queue schema in `sawdust/queue/queue.yaml`
   - `/ralph` command for processing

2. **Phase 2: Core Skills**
   - `/cut` → extract from source
   - `/carve` → find connections
   - `/chamfer` → update older

3. **Phase 3: Pipeline**
   - `/pipeline` → end-to-end processing
   - Cross-connect validation

4. **Phase 4: Vocabulary**
   - Domain derivation
   - Custom terminology

---

## Key Principles

1. **Fresh context per phase** - Always spawn subagents
2. **Queue is source of truth** - State lives in files, not memory
3. **Handoff protocol** - Structured communication between phases
4. **Resumability** - Can interrupt and resume at any point
5. **Cross-connect** - Validate sibling connections after batch completes
