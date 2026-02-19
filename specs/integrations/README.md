# Integration Patterns: Summary

**Purpose:** Extract reusable patterns from source projects without adding dependencies

---

## Source Projects

| Project | Location | What We Extract |
|---------|----------|-----------------|
| **Agent OS** | `/Users/nateb/agent-os/` | Standards discovery, injection, spec shaping |
| **arscontexta** | `/Users/nateb/arscontexta/` | Ralph loop, queue processing, handoff protocol |
| **beads** | `/Users/nateb/beads/` | Typed dependencies, formulas (optional) |

---

## Core Patterns Extracted

### 1. Standards/Patterns System (from Agent OS)

**Discover** → Extract tribal knowledge from codebase
**Inject** → Context-aware loading of relevant patterns
**Index** → Searchable catalog for fast discovery

See: `AGENT-OS-PATTERNS.md`

### 2. Fresh Context Processing (from arscontexta)

**Ralph Loop** → Spawn subagent for each phase
**Handoff Protocol** → Structured communication between phases
**Queue State** → Files as source of truth, not memory

See: `ARSCONTEXTA-PATTERNS.md`

### 3. Typed Dependencies (from beads, optional)

**19 edge types** → blocks, assigned-to, authored-by, etc.
**Formulas** → Workflow templates that spawn linked hierarchies
**Agent-as-bead** → Agents modeled as issues

See: `BEADS-INTEGRATION-SPEC.md`

---

## What to Implement (No Dependencies)

### CLI Commands

```bash
# Already working
workshop init              # Create structure
workshop cut <file>        # Extract insight
workshop carve <query>     # Search
workshop chamfer <file>    # Update context
workshop check             # Validate

# To add (from patterns)
workshop discover [area]   # Extract patterns from codebase
workshop inject [pattern]  # Load relevant patterns
workshop index             # Rebuild search index
workshop shape             # Structured planning workflow
workshop ralph N           # Queue processing with fresh context
workshop pipeline <file>   # End-to-end processing
```

### File Structure

```
workshop/
├── bench/                 # Project identity (exists)
│   ├── identity.md
│   └── methodology.md
├── shavings/              # Knowledge items (exists)
│   ├── patterns/          # Extracted standards
│   └── index.yml          # Searchable index
├── sawdust/               # Ephemeral (exists)
│   ├── queue/             # Processing queue
│   │   └── queue.yaml
│   ├── plans/             # Planning artifacts
│   └── sessions/          # Session logs
└── .workshop/             # Config (exists)
    └── policy/            # Security policies
```

---

## Priority Order

1. **High**: `workshop discover` + `workshop inject` - Standards system
2. **High**: `workshop shape` - Structured planning
3. **Medium**: Queue system + `workshop ralph` - Fresh context processing
4. **Medium**: `workshop pipeline` - End-to-end automation
5. **Low**: Beads integration - Only for users who want it

---

## Key Insights

### From Agent OS
- "Write concise standards — every word costs tokens"
- "Lead with the rule, explain why second"
- "One standard per concept"

### From arscontexta
- "Fresh context per phase prevents context pollution"
- "The queue is the source of truth"
- "Handoff blocks capture learnings across tasks"

### From beads
- "The graph is the API"
- "Compaction preserves structure, discards text"
- "Forgetting is a feature"

---

## Next Steps

1. Review extracted patterns in detail
2. Prioritize which commands to implement
3. Create CLI specs for chosen commands
4. Implement in workshop-cli (Rust)
