# Acceptance Criteria: Full System (24 Tasks)

**Date**: 2026-02-18
**Status**: Definition Phase
**Purpose**: Verifiable criteria for each task before Ralph loop execution

---

## How to Read This

Each task has:
- **AC**: Acceptance Criteria (must pass)
- **Confidence**: Expected success rate
- **Verify**: How to test

---

## Security & Taint (4 tasks)

### S-01: Define taint sources schema

| AC | Criterion | Verify |
|----|-----------|--------|
| S-01-1 | `.workshop/policy/sources.yaml` exists | `test -f .workshop/policy/sources.yaml` |
| S-01-2 | Contains `.secrets/*` pattern | `grep -q ".secrets" sources.yaml` |
| S-01-3 | Contains `*.env` pattern | `grep -q "*.env" sources.yaml` |
| S-01-4 | Contains `credentials.*` pattern | `grep -q "credentials" sources.yaml` |
| S-01-5 | YAML is valid | `python -c "import yaml; yaml.safe_load(open('sources.yaml'))"` |

**Confidence**: 0.95

### S-02: Define exfil sinks schema

| AC | Criterion | Verify |
|----|-----------|--------|
| S-02-1 | `.workshop/policy/sinks.yaml` exists | `test -f .workshop/policy/sinks.yaml` |
| S-02-2 | Contains `curl` command | `grep -q "curl" sinks.yaml` |
| S-02-3 | Contains `wget` command | `grep -q "wget" sinks.yaml` |
| S-02-4 | Contains `rsync`, `scp` commands | `grep -q "rsync\|scp" sinks.yaml` |
| S-02-5 | YAML is valid | `python -c "import yaml; yaml.safe_load(open('sinks.yaml'))"` |

**Confidence**: 0.95

### S-03: Design security packs format

| AC | Criterion | Verify |
|----|-----------|--------|
| S-03-1 | `.workshop/policy/packs/basic.yaml` exists | `test -f .workshop/policy/packs/basic.yaml` |
| S-03-2 | `.workshop/policy/packs/paranoid.yaml` exists | `test -f .workshop/policy/packs/paranoid.yaml` |
| S-03-3 | Pack format documented in ARCHITECTURE.md | `grep -q "pack format" ARCHITECTURE.md` |
| S-03-4 | Packs inherit from base sources/sinks | Visual inspection |

**Confidence**: 0.85

### S-04: Document .cursorignore pattern

| AC | Criterion | Verify |
|----|-----------|--------|
| S-04-1 | `.cursorignore` template created | `test -f templates/.cursorignore` |
| S-04-2 | Documented in ONE-PAGER-HUMAN.md | `grep -q "cursorignore" ONE-PAGER-HUMAN.md` |
| S-04-3 | Reliability bugs documented | `grep -q "reliability" ONE-PAGER-AI.md` |
| S-04-4 | Recommended over hooks for hard denial | Visual inspection |

**Confidence**: 0.95

---

## Cursor Integration (4 tasks)

### C-01: Prototype Cursor SQLite reader

| AC | Criterion | Verify |
|----|-----------|--------|
| C-01-1 | Rust module `src/cursor/db.rs` exists | `test -f src/cursor/db.rs` |
| C-01-2 | Opens `state.vscdb` read-only | Code review |
| C-01-3 | Handles busy timeout (Cursor running) | Code review |
| C-01-4 | Returns error on missing DB | Unit test |
| C-01-5 | Performance: < 5ms to open | Benchmark |

**Confidence**: 0.90

### C-02: Parse composerData format

| AC | Criterion | Verify |
|----|-----------|--------|
| C-02-1 | Extracts `composerData:{uuid}` entries | Unit test with fixture |
| C-02-2 | Parses JSON value correctly | Unit test |
| C-02-3 | Extracts `createdAt`, `lastUpdatedAt` | Unit test |
| C-02-4 | Handles malformed JSON gracefully | Unit test with bad data |
| C-02-5 | Performance: < 10ms per conversation | Benchmark |

**Confidence**: 0.85

### C-03: Parse bubbleId format (v0.40+)

| AC | Criterion | Verify |
|----|-----------|--------|
| C-03-1 | Extracts `bubbleId:{composer}:{bubble}` entries | Unit test |
| C-03-2 | Links bubbles to composers correctly | Unit test |
| C-03-3 | Handles legacy format fallback | Unit test with v0.3x data |
| C-03-4 | Extracts workspace from `workspaceProjectDir` | Unit test |

**Confidence**: 0.80

### C-04: Normalize conversations to struct

| AC | Criterion | Verify |
|----|-----------|--------|
| C-04-1 | `NormalizedConversation` struct defined | Code review |
| C-04-2 | `NormalizedMessage` struct defined | Code review |
| C-04-3 | All Cursor formats normalize to same struct | Integration test |
| C-04-4 | Handles missing fields gracefully | Unit test |
| C-04-5 | JSON serialization works | `serde_json::to_string()` test |

**Confidence**: 0.85

---

## Knowledge & Memory (3 tasks)

### K-01: Define UserMemory schema

| AC | Criterion | Verify |
|----|-----------|--------|
| K-01-1 | `UserMemory` struct matches Agno spec | Compare with `~/agno/libs/agno/agno/db/schemas/memory.py` |
| K-01-2 | Fields: memory, memory_id, topics, created_at, updated_at | Code review |
| K-01-3 | Optional taint field added | Code review |
| K-01-4 | JSON schema export works | `serde_json::to_value()` test |

**Confidence**: 0.95

### K-02: Define workshop folder schema

| AC | Criterion | Verify |
|----|-----------|--------|
| K-02-1 | `bench/` folder structure documented | In ARCHITECTURE.md |
| K-02-2 | `shavings/` naming convention defined | In ARCHITECTURE.md |
| K-02-3 | `sawdust/` purpose documented | In ARCHITECTURE.md |
| K-02-4 | File naming: prose-sentence titles | In ARCHITECTURE.md |
| K-02-5 | YAML frontmatter schema defined | In ARCHITECTURE.md |

**Confidence**: 0.95

### K-03: Design MOC generation

| AC | Criterion | Verify |
|----|-----------|--------|
| K-03-1 | MOC format defined (hub → domain → topic → notes) | In ARCHITECTURE.md |
| K-03-2 | Auto-generation heuristics documented | In ARCHITECTURE.md |
| K-03-3 | Topic extraction from frontmatter defined | In ARCHITECTURE.md |
| K-03-4 | Manual override mechanism documented | In ARCHITECTURE.md |

**Confidence**: 0.70

---

## Processing Pipeline (4 tasks)

### P-01: Implement /cut command

| AC | Criterion | Verify |
|----|-----------|--------|
| P-01-1 | CLI subcommand `cut` registered | `workshop cut --help` |
| P-01-2 | Extracts insight from source file | Integration test |
| P-01-3 | Creates shaving in `shavings/` | Integration test |
| P-01-4 | Optional `--with-code-ref` flag works | Integration test |
| P-01-5 | Performance: < 1ms target | Benchmark |

**Confidence**: 0.85

### P-02: Implement /carve command

| AC | Criterion | Verify |
|----|-----------|--------|
| P-02-1 | CLI subcommand `carve` registered | `workshop carve --help` |
| P-02-2 | Searches for wiki links `[[...]]` | Integration test |
| P-02-3 | Uses ripgrep for search | Code review |
| P-02-4 | Returns matching shavings | Integration test |
| P-02-5 | Performance: < 75ms target | Benchmark |

**Confidence**: 0.80

### P-03: Implement /chamfer command

| AC | Criterion | Verify |
|----|-----------|--------|
| P-03-1 | CLI subcommand `chamfer` registered | `workshop chamfer --help` |
| P-03-2 | Updates older shavings with new context | Integration test |
| P-03-3 | Preserves original frontmatter | Integration test |
| P-03-4 | Performance: < 5ms target | Benchmark |

**Confidence**: 0.70

### P-04: Implement /check command

| AC | Criterion | Verify |
|----|-----------|--------|
| P-04-1 | CLI subcommand `check` registered | `workshop check --help` |
| P-04-2 | Validates YAML frontmatter | Integration test |
| P-04-3 | Checks required fields present | Integration test |
| P-04-4 | Reports health status | `workshop check` output |
| P-04-5 | Performance: < 10ms target | Benchmark |

**Confidence**: 0.90

---

## Setup & Onboarding (4 tasks)

### O-01: Design gum setup prompts

| AC | Criterion | Verify |
|----|-----------|--------|
| O-01-1 | `setup.sh` uses gum for prompts | Code review |
| O-01-2 | Work type prompt (software/research/writing) | Manual test |
| O-01-3 | Code refs prompt (yes/no/optional) | Manual test |
| O-01-4 | Location prompt with default | Manual test |

**Confidence**: 0.95

### O-02: Implement phase 1 detect

| AC | Criterion | Verify |
|----|-----------|--------|
| O-02-1 | Detects macOS/Linux/Windows | Unit test |
| O-02-2 | Checks gum available | Unit test |
| O-02-3 | Checks ripgrep available | Unit test |
| O-02-4 | Reports environment to user | Manual test |

**Confidence**: 0.90

### O-03: Implement phase 2 understand

| AC | Criterion | Verify |
|----|-----------|--------|
| O-03-1 | Gum prompts display correctly | Manual test |
| O-03-2 | Answers captured to variables | Code review |
| O-03-3 | Validation on required fields | Manual test (try empty input) |
| O-03-4 | 2-4 questions as specified | Code review |

**Confidence**: 0.85

### O-04: Implement phase 5 generate

| AC | Criterion | Verify |
|----|-----------|--------|
| O-04-1 | Creates `bench/` folder | Integration test |
| O-04-2 | Creates `shavings/` folder | Integration test |
| O-04-3 | Creates `sawdust/` folder | Integration test |
| O-04-4 | Creates `.workshop/tools/` structure | Integration test |
| O-04-5 | Generates `bench/identity.md` | Integration test |
| O-04-6 | Copies tool templates | Integration test |

**Confidence**: 0.95

---

## Tool Architecture (2 tasks)

### T-01: Define tool file format

| AC | Criterion | Verify |
|----|-----------|--------|
| T-01-1 | YAML frontmatter schema defined | In TOOL-ARCHITECTURE.md |
| T-01-2 | Required fields: name, description, category | In TOOL-ARCHITECTURE.md |
| T-01-3 | Optional fields: performance, invocation | In TOOL-ARCHITECTURE.md |
| T-01-4 | Example tool documented | In TOOL-ARCHITECTURE.md |

**Confidence**: 0.95

### T-02: Create core tool templates

| AC | Criterion | Verify |
|----|-----------|--------|
| T-02-1 | `templates/tools/core/cut.md` exists | `test -f templates/tools/core/cut.md` |
| T-02-2 | `templates/tools/core/carve.md` exists | `test -f templates/tools/core/carve.md` |
| T-02-3 | `templates/tools/core/chamfer.md` exists | `test -f templates/tools/core/chamfer.md` |
| T-02-4 | `templates/tools/core/check.md` exists | `test -f templates/tools/core/check.md` |
| T-02-5 | All have valid YAML frontmatter | `grep -l "^---$" templates/tools/core/*.md` |

**Confidence**: 0.95

---

## CLI & Runtime (3 tasks)

### R-01: Set up Rust project structure

| AC | Criterion | Verify |
|----|-----------|--------|
| R-01-1 | `Cargo.toml` exists | `test -f Cargo.toml` |
| R-01-2 | Dependencies: clap, anyhow, serde, rusqlite, memchr, rayon | `grep Cargo.toml` |
| R-01-3 | `src/main.rs` exists | `test -f src/main.rs` |
| R-01-4 | `cargo build` succeeds | `cargo build` |
| R-01-5 | Binary named `workshop` | `grep "name = \"workshop\"" Cargo.toml` |

**Confidence**: 0.95

### R-02: Define performance budget types

| AC | Criterion | Verify |
|----|-----------|--------|
| R-02-1 | `PerformanceBudget` struct defined | Code review |
| R-02-2 | Fields: target, warning, panic (Duration) | Code review |
| R-02-3 | `QuickReject`, `FastPath`, `FullPipeline` presets | Code review |
| R-02-4 | Budget checking macro/function | Code review |

**Confidence**: 0.95

### R-03: Implement robot/human output modes

| AC | Criterion | Verify |
|----|-----------|--------|
| R-03-1 | `--robot` flag outputs JSON | `workshop --robot cut ...` |
| R-03-2 | Human mode uses colors/formatting | Manual test |
| R-03-3 | JSON schema documented | In CLI doc |
| R-03-4 | Exit codes: 0 success, 1 error, 2 validation | Integration test |

**Confidence**: 0.90

---

## Summary

| Domain | Tasks | Total ACs | Avg Confidence |
|--------|-------|-----------|----------------|
| Security & Taint | 4 | 18 | 0.93 |
| Cursor Integration | 4 | 19 | 0.85 |
| Knowledge & Memory | 3 | 14 | 0.89 |
| Processing Pipeline | 4 | 20 | 0.81 |
| Setup & Onboarding | 4 | 19 | 0.91 |
| Tool Architecture | 2 | 9 | 0.95 |
| CLI & Runtime | 3 | 14 | 0.94 |
| **TOTAL** | **24** | **113** | **0.90** |

---

## Verification Protocol

Before marking task complete, verify ALL ACs pass:

```bash
# Example for S-01
test -f .workshop/policy/sources.yaml && \
grep -q ".secrets" .workshop/policy/sources.yaml && \
grep -q "*.env" .workshop/policy/sources.yaml && \
grep -q "credentials" .workshop/policy/sources.yaml && \
python -c "import yaml; yaml.safe_load(open('.workshop/policy/sources.yaml'))" && \
echo "S-01 PASSED" || echo "S-01 FAILED"
```

---

## Ralph Loop Ready

When all 113 ACs pass, full system is complete.

```bash
while :; do cat specs/blackboard-bootstrap/PROMPT.md | claude-code ; done
```

Each iteration: verify ACs before marking task closed.
