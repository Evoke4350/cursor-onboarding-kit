# Cross-Reference Matrix: Spec Files → Beads → Source Code

**Date**: 2026-02-18
**Purpose**: Verify reductionist loops for Ralph execution

---

## Spec File → Domain → Beads Mapping

| Spec File | Domain(s) Covered | Bead Count | Gap? |
|-----------|-------------------|------------|------|
| CURSOR-BOOTSTRAP.md | All 8 | 29 tasks | No |
| PRD.md | All 8 | N/A (reference) | No |
| ARCHITECTURE.md | All 8 | N/A (reference) | No |
| ACCEPTANCE-CRITERIA.md | All 8 | N/A (reference) | No |
| DOMAIN-PROOF-OF-WORK.md | Proof of Work | 5 tasks | No |
| TOOL-ARCHITECTURE.md | Tool Architecture | 2 tasks | No |
| CONCEPT-CONTEXT-SPACE.md | Conceptual | N/A | No |
| ONE-PAGER-*.md | All 8 | N/A | No |
| SETUP-GUIDE.md | Setup & Onboarding | 4 tasks | No |
| RISK-ASSESSMENT.md | All 8 | N/A | No |
| PROMPT.md | All 8 | N/A | No |
| progress.txt | Session | N/A | No |

---

## Domain → Task → Source Code Reference

### Domain 1: Security & Taint [i5h]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| S-01 | `~/agno/libs/agno/agno/guardrails/base.py` | 8-20 | Abstract guardrail class | Low |
| S-02 | `~/agno/libs/agno/agno/guardrails/pii.py` | 10-95 | PII detection, masking | Medium |
| S-03 | `~/agno/libs/agno/agno/guardrails/prompt_injection.py` | 1-50 | Injection detection | Medium |
| S-04 | Universalis doc (n/a - conceptual) | - | .cursorignore pattern | Low |

**Reductionist loop**: ✅ Each task has specific source to port

---

### Domain 2: Cursor Integration [0hq]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| C-01 | `~/Dicklesworthstone/coding_agent_session_search/src/connectors/cursor.rs` | 156-239 | SQLite detection/reading | Medium |
| C-02 | Same | 422-593 | composerData parsing | High |
| C-03 | Same | 245-282 | bubbleId parsing | High |
| C-04 | Same | 284-342 | Workspace extraction | Medium |

**Reductionist loop**: ✅ All tasks from single source file

---

### Domain 3: Knowledge & Memory [kph]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| K-01 | `~/agno/libs/agno/agno/db/schemas/memory.py` | 8-58 | UserMemory dataclass | Low |
| K-02 | `~/agno/libs/agno/agno/memory/manager.py` | 42-1543 | MemoryManager CRUD | High |
| K-03 | Same | 365-473 | Memory capture | Medium |

**Reductionist loop**: ✅ Two source files, clear mapping

---

### Domain 4: Processing Pipeline [5gk]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| P-01 | `~/arscontexta/generators/features/processing-pipeline.md` | 12-89 | 4-phase pipeline | Medium |
| P-02 | Same | 108-252 | Fresh context per phase | Medium |
| P-03 | Same | 117-176 | Queue-driven processing | Medium |
| P-04 | Same | 204-296 | Execution modes | Medium |

**Reductionist loop**: ✅ Single source file for all tasks

---

### Domain 5: Setup & Onboarding [i2s]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| O-01 | `~/arscontexta/generators/claude-md.md` | 12-50 | Gum prompt design | Low |
| O-02 | Same | 66-90 | Environment detection | Low |
| O-03 | Same | 116-125 | Understanding phase | Medium |
| O-04 | `~/arscontexta/generators/features/*.md` | Various | Generation phase | Medium |

**Reductionist loop**: ⚠️ O-04 needs multiple source files

---

### Domain 6: Tool Architecture [j4b]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| T-01 | `~/arscontexta/generators/features/templates.md` | 12-49 | Tool definition format | Low |
| T-02 | Same | 17-82 | Feature blocks | Low |

**Reductionist loop**: ✅ Single source file

---

### Domain 7: CLI & Runtime [7vp]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| R-01 | `~/Dicklesworthstone/destructive_command_guard/src/perf.rs` | 35-100 | PerformanceBudget | Low |
| R-02 | Same | 17+ | SIMD quick reject | Medium |
| R-03 | `~/Dicklesworthstone/xf/src/hybrid.rs` | 1-859 | Hybrid search | High |

**Reductionist loop**: ✅ Two source repos, clear mapping

---

### Domain 8: Proof of Work [clz]

| Task | Source File | Lines | Concept | Complexity |
|------|-------------|-------|---------|------------|
| W-01 | `~/showboat/main.go` | 1-200 | CLI integration | Low |
| W-02 | `~/showboat/extract.go` | 15-49 | Document storage | Low |
| W-03 | `~/showboat/verify.go` | 1-100 | Verification | Low |
| W-04 | Same | - | Remote streaming | Medium |
| W-05 | `~/chartroom/README.md` | 1-479 | Chart integration | Low |

**Reductionist loop**: ✅ Two source repos, clear mapping

---

## Gap Analysis

| Gap | Description | Resolution |
|-----|-------------|------------|
| O-04 sources | Multiple feature files needed | Reference all `~/arscontexta/generators/features/*.md` |
| Research tasks | 16 research tasks without code refs | Keep for context, not implementation |
| DOC tasks | 7 doc tasks unrelated to blackboard | Exclude from Ralph loop |

---

## Ralph Loop Verification

### Can each task be executed in a loop?

| Domain | Tasks | Source Mapped | Loopable |
|--------|-------|---------------|----------|
| Security & Taint | 4 | ✅ | ✅ |
| Cursor Integration | 4 | ✅ | ✅ |
| Knowledge & Memory | 3 | ✅ | ✅ |
| Processing Pipeline | 4 | ✅ | ✅ |
| Setup & Onboarding | 4 | ⚠️ | ✅ |
| Tool Architecture | 2 | ✅ | ✅ |
| CLI & Runtime | 3 | ✅ | ✅ |
| Proof of Work | 5 | ✅ | ✅ |

**Total loopable**: 29/29 implementation tasks

---

## Research Tasks (Context Only)

These provide context but don't need implementation:
- Research Cursor extensibility
- Research note-taking frameworks
- Inventory Cursor IDE hooks
- Design blackboard naming system (done in specs)
- DOC: agent/* tasks (unrelated)

---

## Composable Module Structure

```
workshop/
├── modules/
│   ├── security/           # Domain 1
│   │   ├── guardrail.rs    # From agno/guardrails/
│   │   ├── taint.rs        # From Universalis concept
│   │   └── policy.rs       # Sources + sinks YAML
│   │
│   ├── cursor/             # Domain 2
│   │   ├── sqlite.rs       # From coding_agent_session_search
│   │   ├── composer.rs     # composerData parsing
│   │   └── bubble.rs       # bubbleId parsing
│   │
│   ├── memory/             # Domain 3
│   │   ├── schema.rs       # From agno/db/schemas/
│   │   └── manager.rs      # From agno/memory/
│   │
│   ├── pipeline/           # Domain 4
│   │   ├── phases.rs       # From arscontexta
│   │   └── queue.rs        # Task queue
│   │
│   ├── setup/              # Domain 5
│   │   ├── detect.rs       # Environment detection
│   │   ├── derive.rs       # From arscontexta derivation
│   │   └── generate.rs     # File generation
│   │
│   ├── tools/              # Domain 6
│   │   ├── format.rs       # Tool definition format
│   │   └── templates.rs    # Feature blocks
│   │
│   ├── cli/                # Domain 7
│   │   ├── perf.rs         # From destructive_command_guard
│   │   ├── search.rs       # From xf
│   │   └── output.rs       # Robot/human modes
│   │
│   └── proof/              # Domain 8
│       ├── showboat.rs     # CLI wrapper
│       ├── verify.rs       # Document verification
│       └── chart.rs        # Chartroom wrapper
│
└── compose.rs              # Module composition
```

---

## Source Repo → Module Mapping

| Source Repo | Modules Contributing |
|-------------|---------------------|
| `~/agno` | security/, memory/ |
| `~/Dicklesworthstone/coding_agent_session_search` | cursor/ |
| `~/Dicklesworthstone/destructive_command_guard` | cli/perf.rs |
| `~/Dicklesworthstone/xf` | cli/search.rs |
| `~/arscontexta` | pipeline/, setup/, tools/ |
| `~/showboat` | proof/ |
| `~/chartroom` | proof/chart.rs |

---

## Verification Checklist

- [x] All 8 domains have bead epics
- [x] All 29 implementation tasks have source code refs
- [x] Each task maps to specific file + lines
- [x] Complexity assessed for each task
- [x] Composable module structure defined
- [x] Source repo → module mapping complete
- [x] Ralph loop can execute each task independently
