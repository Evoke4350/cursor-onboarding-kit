# Extracted Patterns: Agent OS → Blackboard

**Source:** `/Users/nateb/agent-os/commands/agent-os/`
**Purpose:** Reusable prompt templates adapted for workshop/blackboard context

---

## Core Commands Extracted

| Command | Purpose | Blackboard Adaptation |
|---------|---------|----------------------|
| `discover-standards` | Extract tribal knowledge from codebase | `workshop discover` - extract patterns to shavings |
| `inject-standards` | Context-aware standard injection | `workshop inject` - load relevant shavings |
| `index-standards` | Build searchable index | `workshop index` - index shavings for search |
| `shape-spec` | Structured planning workflow | `workshop shape` - plan with context |
| `plan-product` | Product vision documentation | `workshop plan-product` - mission/roadmap/tech-stack |

---

## Pattern 1: Discover Standards → Extract Patterns

### Original (Agent OS)
Extract tribal knowledge from codebase into documented standards.

### Adapted for Blackboard

```markdown
# /workshop discover

Extract patterns from your codebase into shavings.

## Process

1. **Scan structure** — Analyze folders, file types, naming patterns
2. **Identify areas** — API, database, UI, auth, testing, etc.
3. **Find patterns** — Look for:
   - Unusual or unconventional approaches
   - Opinionated choices that could have gone differently
   - Tribal knowledge a new dev wouldn't know
   - Consistent patterns across multiple files
4. **Create shavings** — One insight per file with code context

## Output

Creates shavings in `shavings/patterns/`:
- `YYYYMMDD-HHMM-api-response-format.md`
- `YYYYMMDD-HHMM-database-migrations.md`

## Format

```markdown
# [Pattern Name]

## The Rule

[What to do - lead with this]

## Why

[Why this pattern exists - optional]

## Example

[Code snippet showing the pattern]
```

## Key Principle

Write concise. Every word costs tokens. Lead with the rule, show code, skip the obvious.
```

---

## Pattern 2: Inject Standards → Load Context

### Original (Agent OS)
Context-aware injection of relevant standards based on current work.

### Adapted for Blackboard

```markdown
# /workshop inject [area]

Load relevant shavings into current context.

## Modes

### Auto-Suggest (no arguments)
Analyzes current task and suggests relevant shavings:
```
Based on your task, these patterns may be relevant:
1. api/response-format — Response envelope structure
2. api/error-handling — Error codes and handling

Load these? (yes / just 1 / add: database/migrations)
```

### Explicit (with arguments)
```
/workshop inject api                    # All API patterns
/workshop inject api/response-format    # Single pattern
/workshop inject api database           # Multiple areas
```

## Detection

Three scenarios:
1. **Conversation** — Implementation work, load full content
2. **Planning** — Building a spec, return file references
3. **Creating skill** — Building reusable command, offer embed or reference

## Output (Conversation Mode)

```
Loaded patterns:

--- Pattern: api/response-format ---

[Full content]

--- End Pattern ---

Key points:
- All responses use { success, data, error } envelope
- Error codes follow AUTH_xxx pattern
```
```

---

## Pattern 3: Shape Spec → Plan Work

### Original (Agent OS)
Structured planning workflow with context gathering.

### Adapted for Blackboard

```markdown
# /workshop shape

Gather context and structure planning for significant work.

## Process

### Step 1: Clarify Scope

```
What are we building? Please describe the feature or change.
```

### Step 2: Gather Visuals

```
Do you have visuals to reference?
- Mockups or wireframes
- Screenshots of similar features
- Examples from other apps
```

### Step 3: Find References

```
Is there similar code in this codebase I should reference?
```

### Step 4: Check Product Context

If `bench/identity.md` or `bench/methodology.md` exist:
```
Your project context:
- Identity: [from bench/identity.md]
- Methodology: [from bench/methodology.md]

Should this align with any specific goals?
```

### Step 5: Surface Patterns

Read relevant shavings:
```
Based on what we're building, these patterns apply:
1. api/response-format
2. database/migrations

Include these in the plan?
```

### Step 6: Structure Output

Create `sawdust/plans/YYYY-MM-DD-HHMM-{slug}/`:
- `plan.md` — Full implementation plan
- `context.md` — Shaping decisions
- `patterns.md` — Which patterns apply
- `references.md` — Similar code to study

### Step 7: Task Structure

```
## Task 1: Save Planning Artifacts
[Create the plan folder]

## Task 2: [First implementation task]
[Description based on scope]

## Task 3: [Next task]
...
```
```

---

## Pattern 4: Product Planning → Project Identity

### Original (Agent OS)
Establish mission, roadmap, tech stack.

### Adapted for Blackboard

Already implemented as `workshop init` with:
- `bench/identity.md` — Who/what this workshop is
- `bench/methodology.md` — How you work

Can extend with:
- `bench/roadmap.md` — Phases and milestones
- `bench/tech-stack.md` — Technologies used

---

## File Structure Mapping

| Agent OS | Blackboard | Purpose |
|----------|------------|---------|
| `agent-os/standards/` | `shavings/patterns/` | Extracted patterns |
| `agent-os/standards/index.yml` | `shavings/index.yml` | Searchable index |
| `agent-os/specs/` | `sawdust/plans/` | Planning artifacts |
| `agent-os/product/` | `bench/` | Project identity |

---

## Implementation Notes

1. **Don't copy verbatim** — Adapt language to workshop metaphor
2. **Keep prompts concise** — Token costs matter
3. **One pattern per shaving** — Atomic insights
4. **Index for search** — Enables fast injection
5. **Context-aware injection** — Don't load everything

---

## CLI Commands to Add

```bash
workshop discover [area]     # Extract patterns from codebase
workshop inject [pattern]    # Load relevant patterns
workshop index               # Rebuild search index
workshop shape               # Structured planning workflow
```

These complement existing:
```bash
workshop cut                 # Extract single insight
workshop carve               # Search connections
workshop chamfer             # Update older shavings
workshop check               # Validate structure
```
