# Architecture: Bootstrap Agentic Blackboard (Workshop)

**Date**: 2026-02-18
**Status**: Draft - Decomposition Phase
**Parent Epic**: cursor-onboarding-kit-a4z

---

## Overview

The Agentic Blackboard is a **runtime-enforced cognitive architecture** for AI agents that provides:

- **AGENTIC MEMORY** — Persistent knowledge across sessions
- **AGENTIC HISTORY** — Conversation and execution tracking
- **AGENTIC CONTEXT** — Automatic context injection based on task
- **AGENTIC LEARNING** — Pattern extraction and refinement
- **AGENTIC CONTROL** — Guardrails, taint analysis, exfil prevention
- **AGENTIC TOOLS** — File-based tool discovery and execution

All as defaults. No MCP server required. Works with Cursor IDE and Claude Code.

---

## Theoretical Foundation

### Parameter Space vs Context Space

| Dimension | Parameter Space | Context Space |
|-----------|----------------|---------------|
| **Cost** | Expensive (GPU hours) | Cheap (tokens) |
| **Reversibility** | No (weight updates) | Yes (git rollback) |
| **Generality** | Destroys base model | Preserves base model |
| **Techniques** | SFT, RL, GRPO | Experience library |
| **Failure mode** | Overfitting, catastrophic forgetting | Context pollution |

**Parameter space is unmanageable.** Even techniques like Training-Free GRPO (arXiv:2510.08191) that try to avoid parameter updates still operate in a regime where:
- The optimization landscape is treacherous
- Small changes can cascade unpredictably
- The base model's capabilities can degrade

**Context space is the legitimate alternative.** The key insight from Training-Free GRPO:

> "LLMs can achieve a similar effect on the output distribution by learning experiential knowledge as a token prior."

The mechanism:
1. **Semantic advantage** — Model introspects attempts, identifies what worked
2. **Natural language rules** — Extracts patterns as prose, not weights
3. **Experience library** — Stores rules for future retrieval
4. **Token prior injection** — Seamlessly integrated during API calls

### How Blackboard Maps to This

| GRPO Concept | Blackboard Implementation |
|--------------|---------------------------|
| Semantic advantage | `/carve` finds connections, `/check` validates quality |
| Natural language rules | Shavings stored as markdown prose |
| Experience library | `shavings/` folder with wiki links |
| Token prior injection | Context files (`CLAUDE.md`, `AGENTS.md`) |
| Multi-epoch distillation | Ralph loop: fresh context, persistent state |

**This is what we're fucking doing.** Managing context space instead of parameter space.

---

## Domain Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    BOOTSTRAP AGENTIC BLACKBOARD                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   SECURITY   │  │   CURSOR     │  │  KNOWLEDGE   │  │  PIPELINE    │ │
│  │   & TAINT    │  │ INTEGRATION  │  │   & MEMORY   │  │   5 Cs       │ │
│  │   [i5h]      │  │   [0hq]      │  │   [kph]      │  │   [5gk]      │ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                   │
│  │    SETUP     │  │    TOOL      │  │   CLI &      │                   │
│  │ & ONBOARDING │  │ ARCHITECTURE │  │   RUNTIME    │                   │
│  │   [i2s]      │  │   [j4b]      │  │   [7vp]      │                   │
│  └──────────────┘  └──────────────┘  └──────────────┘                   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Domain 1: Security & Taint [cursor-onboarding-kit-i5h]

### Purpose
Protect sensitive data from exfiltration by tracking data flow from sources to sinks.

### Concepts (from Universalis doc)
- **Sources** — Operations that produce sensitive data (`read_file`, `secrets/*`)
- **Sinks** — Operations that can send data externally (`curl`, `wget`, `rsync`)
- **Taint** — Mark on conversation when source is touched
- **Policy** — Rules defining what's protected and where it can go

### Components

```
.workshop/
├── policy/
│   ├── sources.yaml      # Protected paths and operations
│   ├── sinks.yaml        # Exfil operations to block/monitor
│   └── packs/            # Pre-configured security packs
│       ├── basic.yaml
│       ├── paranoid.yaml
│       └── development.yaml
```

### Cursor Integration
- `.cursor/hooks.json` — before_file_read, before_shell_execution, before_mcp_execution
- `.cursorignore` — Hard denial (recommended by Cursor staff)
- `.secrets/` — Workspace-relative protected directory

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| S-01 | As a dev, I want `.secrets/*` to automatically taint conversations | P0 |
| S-02 | As a dev, I want to block exfil commands (curl, wget, rsync) when tainted | P0 |
| S-03 | As a dev, I want to configure security packs like Dicklesworth | P1 |
| S-04 | As a dev, I want `.cursorignore` support for hard denial | P1 |
| S-05 | As a dev, I want hooks to reliably deny/redact (document reliability bugs) | P1 |

---

## Domain 2: Cursor Integration [cursor-onboarding-kit-0hq]

### Purpose
Hook into Cursor IDE without requiring official API or permissions.

### Two Integration Points

#### A. Documented Hooks (unreliable)
```json
// .cursor/hooks.json
{
  "hooks": {
    "before_file_read": "workshop check-read $FILE",
    "before_shell_execution": "workshop check-shell $CMD",
    "before_mcp_execution": "workshop check-mcp $TOOL"
  }
}
```

**Known issues**: Reliability bugs in certain Cursor versions. `.cursorignore` recommended.

#### B. Undocumented SQLite Hooks (gold)
From `coding_agent_session_search`:

```rust
// Cursor stores in SQLite:
~/Library/Application Support/Cursor/User/
├── globalStorage/state.vscdb       # Global conversations
└── workspaceStorage/{id}/state.vscdb  # Per-workspace

// Tables:
- cursorDiskKV: composerData:{uuid}, bubbleId:{composer}:{bubble}
- ItemTable: aichat data (legacy)
```

**What we can extract**:
- Full conversation history
- Workspace/project context
- Message timestamps
- Model used
- Code references

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| C-01 | As a dev, I want to extract Cursor chat history via SQLite | P0 |
| C-02 | As a dev, I want to inject context into Cursor via files | P0 |
| C-03 | As a dev, I want session capture on Cursor close | P1 |
| C-04 | As a dev, I want workspace-aware blackboard (per-project) | P1 |
| C-05 | As a dev, I want to sync blackboard state to Cursor's DB | P2 |

---

## Domain 3: Knowledge & Memory [cursor-onboarding-kit-kph]

### Purpose
Persistent knowledge storage with semantic retrieval, inspired by Agno.

### Hybrid Schema

#### Internal (Agno-style)
```python
@dataclass
class UserMemory:
    memory: str                    # Content
    memory_id: str                 # UUID
    topics: List[str]              # Categories
    user_id: str                   # Owner
    created_at: int                # Timestamp
    updated_at: int                # Last modified
    feedback: Optional[str]        # User feedback
    agent_id: str                  # Source agent
    taint: Optional[List[str]]     # Security marks
```

#### External (Workshop schema)
```
~/workshop/
├── bench/           # MOCs, identity, methodology
│   ├── index.md     # Hub
│   ├── identity.md  # Who am I, what do I know
│   └── standards/   # Extracted patterns (agent-os style)
├── shavings/        # Atomic insights
│   └── *.md         # Individual notes
└── sawdust/         # Ephemeral
    ├── sessions/    # Session logs
    └── queue/       # Processing queue
```

### Agno Patterns to Borrow
| Pattern | Application |
|---------|-------------|
| `MemoryManager` | Agentic (tool-based) vs auto (after-run) memory capture |
| `KnowledgeRow` | Structured storage with metadata, access tracking |
| `RRF Fusion` | Hybrid search (semantic + keyword) |
| `User-level isolation` | Multi-user blackboard support |

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| K-01 | As a dev, I want shavings to use Agno's memory schema internally | P0 |
| K-02 | As a dev, I want semantic search across shavings (ripgrep + optional vectors) | P0 |
| K-03 | As a dev, I want standards extraction like agent-os discover-standards | P1 |
| K-04 | As a dev, I want contextual injection like agent-os inject-standards | P1 |
| K-05 | As a dev, I want MOC auto-generation from shavings | P1 |

---

## Domain 4: Processing Pipeline [cursor-onboarding-kit-5gk]

### Purpose
The 5 Cs framework for engineering-focused note processing.

### The 5 Cs (Construction)

| Phase | What | Command | Budget |
|-------|------|---------|--------|
| **Capture** | Zero-friction capture to sawdust/inbox | Manual | - |
| **Cut** | Extract atomic insight with code context | `/cut` | < 1ms |
| **Carve** | Find connections, join with joints (wiki links) | `/carve` | < 75ms |
| **Chamfer** | Smooth edges — update older shavings | `/chamfer` | < 5ms |
| **Check** | Caliper validation + vault health | `/check` | < 10ms |

Plus meta-cognitive:
| **Sharpen** | Refine the system itself | `/sharpen` | < 100ms |

### Fresh Context Per Phase (from arscontexta)
Each phase spawns a fresh agent to prevent context pollution ("the gutter"). State persists in files and git, not LLM memory.

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| P-01 | As a dev, I want /cut to extract insights with optional code refs | P0 |
| P-02 | As a dev, I want /carve to find connections via ripgrep + wiki links | P0 |
| P-03 | As a dev, I want /chamfer to update older shavings with new context | P1 |
| P-04 | As a dev, I want /check to validate schema + run health checks | P1 |
| P-05 | As a dev, I want /sharpen to refine my blackboard configuration | P2 |

---

## Domain 5: Setup & Onboarding [cursor-onboarding-kit-i2s]

### Purpose
Gum-based interactive setup that derives configuration from conversation.

### 6-Phase Process (from arscontexta)

| Phase | What |
|-------|------|
| **1. Detect** | Check environment (Cursor/Claude Code), available tools |
| **2. Understand** | 2-4 questions about your engineering domain |
| **3. Derive** | Map signals to configuration dimensions |
| **4. Propose** | Show what will be generated in workshop terms |
| **5. Generate** | Create folders, templates, commands, hooks config |
| **6. Validate** | Smoke test, show first-success guidance |

### Gum Implementation

```bash
#!/usr/bin/env bash
# setup.sh - 🪵 Welcome to the Workshop!

gum style --foreground 212 --bold "🪵 Welcome to the Workshop!"
echo ""

WORK_TYPE=$(gum choose --header "What kind of work do you do?" \
    "software" "research" "writing" "other")

CODE_REFS=$(gum choose --header "Should shavings reference code?" \
    "yes" "no" "optional")

# ... continues with derivation logic
```

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| O-01 | As a dev, I want gum-based interactive setup | P0 |
| O-02 | As a dev, I want the setup to derive config from my answers | P0 |
| O-03 | As a dev, I want workshop naming (bench/shavings/sawdust) | P0 |
| O-04 | As a dev, I want security pack selection during setup | P1 |
| O-05 | As a dev, I want optional semantic search (qmd) setup | P2 |

---

## Domain 6: Tool Architecture [cursor-onboarding-kit-j4b]

### Purpose
File-based tool discovery compatible with Cursor's dynamic context pattern.

### Structure
```
.workshop/tools/
├── core/
│   ├── cut.md          # Extract insight
│   ├── carve.md        # Find connections
│   ├── chamfer.md      # Update older
│   ├── check.md        # Validate
│   └── sharpen.md      # Meta refinement
├── search/
│   ├── rg.md           # Ripgrep
│   ├── semantic.md     # Vector (optional)
│   └── graph.md        # Graph traversal
└── setup/
    ├── init.md
    ├── calibrate.md
    └── health.md
```

### Tool Definition Format
```markdown
---
name: cut
description: Extract atomic insight from source
category: core
performance:
  target: 1ms
  warning: 10ms
  panic: 50ms
---

# Cut

[Full documentation...]
```

### Cursor Pattern (from blog)
> We create one folder per server, keeping each server's tools logically grouped. Files enable full `rg` + `jq` filtering.

**Result**: 46.9% token reduction vs always-loaded MCP tools.

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| T-01 | As a dev, I want tools as files, not MCP calls | P0 |
| T-02 | As a dev, I want Cursor to discover tools via rg | P0 |
| T-03 | As a dev, I want performance budgets per tool | P1 |
| T-04 | As a dev, I want tool packs like Dicklesworth | P1 |
| T-05 | As a dev, I want custom tool generation during setup | P2 |

---

## Domain 7: CLI & Runtime [cursor-onboarding-kit-7vp]

### Purpose
High-performance Rust CLI with SIMD acceleration and latency budgets.

### Performance Budgets (from destructive_command_guard)

| Tier | Target | Warning | Panic |
|------|--------|---------|-------|
| Quick reject | < 1μs | > 10μs | > 50μs |
| Fast path | < 75μs | > 200μs | > 500μs |
| Full pipeline | < 5ms | > 10ms | > 20ms |

### Key Technologies (from xf)
- `memchr` — SIMD-accelerated substring search
- `ripgrep` — Fast file search
- `tantivy` — Full-text search (optional)
- `rayon` — Parallel processing
- `clap` — CLI parsing

### Output Modes
- **Human mode** — Rich terminal output with colors
- **Robot mode** — JSON output for scripting/CI

### User Stories
| ID | Story | Priority |
|----|-------|----------|
| R-01 | As a dev, I want sub-millisecond search like xf | P0 |
| R-02 | As a dev, I want SIMD quick reject like dcg | P0 |
| R-03 | As a dev, I want robot mode JSON output | P1 |
| R-04 | As a dev, I want CI-enforced performance budgets | P1 |
| R-05 | As a dev, I want golden file testing like CASS | P2 |

---

## Cross-Cutting Concerns

### A. Agent OS Patterns (from buildermethods)

| Pattern | Application |
|---------|-------------|
| **Discover Standards** | Extract patterns from conversations → shavings |
| **Inject Standards** | Context-aware injection based on task |
| **Shape Spec** | Plan with standards context |
| **Index Standards** | Keep shavings discoverable via index.yml |

### B. Arca Contexta Research Claims

The 249 research claims provide theoretical grounding for:
- Three-space separation (bench/shavings/sawdust)
- Fresh context per phase
- Domain vocabulary transformation
- Justification chains for configuration decisions

### C. Universalis Taint Analysis

Security policy as executable logic:
- Sources defined in YAML
- Sinks defined in YAML
- Rules compute transitive closure
- Violations block execution

---

## Source Repositories

| Repo | What We Borrow |
|------|----------------|
| `arscontexta` | 6-phase setup, research claims, 6 Rs framework |
| `agent-os` (both) | Standards discovery/injection, shape-spec pattern |
| `agno` | Memory schema, knowledge base, guardrails, events |
| `coding_agent_session_search` | Cursor SQLite connector |
| `destructive_command_guard` | Performance budgets, SIMD, pack architecture |
| `xf` | Sub-millisecond hybrid search |
| `Universalis` | Taint analysis for AI workflows |

---

## Next Steps

1. **Spec each domain** — Create detailed spec files for each of the 7 domains
2. **Define user stories** — Flesh out all user stories with acceptance criteria
3. **Prototype cursor connector** — Extract Cursor chats via SQLite
4. **Design security packs** — Create pack format like Dicklesworth
5. **Write setup script** — Gum-based interactive onboarding

---

## Bead Summary

```
DOMAIN EPICS:
cursor-onboarding-kit-i5h  DOMAIN: Security & Taint
cursor-onboarding-kit-0hq  DOMAIN: Cursor Integration
cursor-onboarding-kit-kph  DOMAIN: Knowledge & Memory
cursor-onboarding-kit-5gk  DOMAIN: Processing Pipeline
cursor-onboarding-kit-i2s  DOMAIN: Setup & Onboarding
cursor-onboarding-kit-j4b  DOMAIN: Tool Architecture
cursor-onboarding-kit-7vp  DOMAIN: CLI & Runtime

PARENT EPIC:
cursor-onboarding-kit-a4z  Bootstrap Agentic Blackboard Epic
```
