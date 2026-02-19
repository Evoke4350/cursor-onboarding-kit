# Bead Cross-Check

**Date**: 2026-02-18
**Purpose**: Verify every implementation bead has clear scope, source ref, acceptance criteria

---

## P0 Tasks (21 tasks) - Implementation Critical Path

### Domain 1: Security & Taint [i5h]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| y6k | S-01: Define taint sources schema | Create sources.yaml with protected paths | agno/guardrails/pii.py:10-95 | AC-S-01,02 | i5h | ✅ |
| bnt | S-02: Define exfil sinks schema | Create sinks.yaml with blocked commands | New (YAML-based) | AC-S-03,04 | i5h | ✅ |

### Domain 2: Cursor Integration [0hq]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| 34k | C-01: Prototype Cursor SQLite reader | Read state.vscdb, find tables | cursor.rs:156-239 | AC-C-01,02 | 0hq | ✅ |
| e43 | C-02: Parse composerData format | Extract composerData:{uuid} entries | cursor.rs:422-593 | AC-C-03,04 | 34k | ✅ |
| ivj | C-03: Parse bubbleId format | Extract bubbleId:{composer}:{bubble} | cursor.rs:245-282 | AC-C-05,06 | 34k | ✅ |
| hhy | C-04: Normalize conversations | Convert to NormalizedConversation struct | cursor.rs:284-342 | AC-C-07,08 | e43 | ✅ |

### Domain 3: Knowledge & Memory [kph]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| idg | K-01: Define UserMemory schema | Rust struct with Agno fields + taint | memory.py:8-58 | AC-K-01,02 | kph | ✅ |
| zor | K-02: Define workshop folder schema | bench/shavings/sawdust structure | New (workshop theme) | AC-K-03,04 | idg | ✅ |

### Domain 4: Processing Pipeline [5gk]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| q2n | P-01: Implement /cut command | Extract atomic insight with code refs | processing-pipeline.md:12-89 | AC-P-01,02 | 5gk | ✅ |
| k9o | P-02: Implement /carve command | Find connections via ripgrep | processing-pipeline.md:108-152 | AC-P-03,04 | q2n | ✅ |
| 6vu | P-03: Implement /chamfer command | Update older shavings | processing-pipeline.md:153-196 | AC-P-05,06 | k9o | ⚠️ |
| qpu | P-04: Implement /check command | Validate schema + health checks | processing-pipeline.md:204-296 | AC-P-07,08 | 6vu | ⚠️ |

### Domain 5: Setup & Onboarding [i2s]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| x56 | O-01: Design gum setup prompts | Interactive CLI prompts for setup | claude-md.md:12-50 | AC-O-01,02 | i2s | ✅ |
| kj0 | O-02: Implement phase 1 detect | Environment detection | claude-md.md:66-90 | AC-O-03,04 | x56 | ✅ |
| lgv | O-03: Implement phase 2 understand | 2-4 questions about domain | claude-md.md:116-125 | AC-O-05,06 | kj0 | ✅ |
| cy5 | O-04: Implement phase 5 generate | Create folders, templates, config | features/*.md | AC-O-07,08,09 | lgv | ⚠️ |

### Domain 6: Tool Architecture [j4b]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| 75g | T-01: Define tool file format | YAML frontmatter + markdown body | templates.md:12-49 | AC-T-01,02 | j4b | ✅ |
| gkq | T-02: Create core tool templates | cut.md, carve.md, chamfer.md, check.md | templates.md:17-82 | AC-T-03,04 | 75g | ✅ |

### Domain 7: CLI & Runtime [7vp]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| cb4 | R-01: Set up Rust project | Cargo.toml with dependencies | New (standard Rust) | AC-R-01,02 | 7vp | ✅ |
| 1b3 | R-02: Define performance budget types | PerformanceBudget struct | perf.rs:35-100 | AC-R-03,04 | cb4 | ✅ |

### Domain 8: Proof of Work [clz]

| Bead ID | Title | Scope | Source | AC | Deps | Status |
|---------|-------|-------|--------|-----|------|--------|
| lre | W-01: Integrate showboat CLI | uvx wrapper for note/exec/verify | showboat/main.go | AC-W-01,02 | clz | ✅ |
| 6dc | W-02: Store shavings as showboat docs | Each shaving = executable doc | showboat/extract.go | AC-W-03,04 | lre | ✅ |
| 3d5 | W-03: /check runs showboat verify | Verify all shavings, report diffs | showboat/verify.go | AC-W-05,06 | 6dc | ✅ |

---

## Status Legend

- ✅ **Complete**: Scope, source, AC, deps all documented
- ⚠️ **Needs Review**: Missing or unclear element
- ❌ **Gap**: Critical information missing

---

## Issues Found

### 1. O-04 Multiple Source Files
**Problem**: O-04 (cy5) references multiple feature files, not single source.
**Resolution**: Document all `~/arscontexta/generators/features/*.md` files needed.

### 2. P-03, P-04 Dependency Chain
**Problem**: Dependencies show P-03 → P-04 but both might need K-02 (workshop structure).
**Resolution**: Add K-02 as dependency for P-03.

### 3. No Integration Task
**Problem**: No bead for `compose.rs` that wires all modules together.
**Resolution**: Add integration task after all modules complete.

---

## Missing Beads

These should be added:

| Proposed ID | Title | Scope | Depends On |
|-------------|-------|-------|------------|
| INT-01 | Compose all modules | Create compose.rs wiring security+cursor+memory+pipeline | All P0 tasks |
| INT-02 | Smoke test | End-to-end test of init → cut → verify | INT-01 |

---

## Bead → Source Code Verification

Every P0 task has source reference in CROSS-REFERENCE-MATRIX.md:

```
✅ S-01 → agno/guardrails/pii.py:10-95
✅ S-02 → New (YAML-based)
✅ C-01 → cursor.rs:156-239
✅ C-02 → cursor.rs:422-593
✅ C-03 → cursor.rs:245-282
✅ C-04 → cursor.rs:284-342
✅ K-01 → memory.py:8-58
✅ K-02 → New (workshop theme)
✅ P-01 → processing-pipeline.md:12-89
✅ P-02 → processing-pipeline.md:108-152
✅ P-03 → processing-pipeline.md:153-196
✅ P-04 → processing-pipeline.md:204-296
✅ O-01 → claude-md.md:12-50
✅ O-02 → claude-md.md:66-90
✅ O-03 → claude-md.md:116-125
⚠️ O-04 → features/*.md (multiple)
✅ T-01 → templates.md:12-49
✅ T-02 → templates.md:17-82
✅ R-01 → New (standard Rust)
✅ R-02 → perf.rs:35-100
✅ W-01 → showboat/main.go
✅ W-02 → showboat/extract.go
✅ W-03 → showboat/verify.go
```

---

## Bead → Acceptance Criteria Verification

Every P0 task has AC in ACCEPTANCE-CRITERIA.md:

```
✅ S-01 → AC-S-01, AC-S-02
✅ S-02 → AC-S-03, AC-S-04
✅ C-01 → AC-C-01, AC-C-02
✅ C-02 → AC-C-03, AC-C-04
✅ C-03 → AC-C-05, AC-C-06
✅ C-04 → AC-C-07, AC-C-08
✅ K-01 → AC-K-01, AC-K-02
✅ K-02 → AC-K-03, AC-K-04
✅ P-01 → AC-P-01, AC-P-02
✅ P-02 → AC-P-03, AC-P-04
✅ P-03 → AC-P-05, AC-P-06
✅ P-04 → AC-P-07, AC-P-08
✅ O-01 → AC-O-01, AC-O-02
✅ O-02 → AC-O-03, AC-O-04
✅ O-03 → AC-O-05, AC-O-06
✅ O-04 → AC-O-07, AC-O-08, AC-O-09
✅ T-01 → AC-T-01, AC-T-02
✅ T-02 → AC-T-03, AC-T-04
✅ R-01 → AC-R-01, AC-R-02
✅ R-02 → AC-R-03, AC-R-04
✅ W-01 → AC-W-01, AC-W-02
✅ W-02 → AC-W-03, AC-W-04
✅ W-03 → AC-W-05, AC-W-06
```

---

## Summary

| Metric | Count | Status |
|--------|-------|--------|
| P0 Tasks | 21 | ✅ All have scope |
| Source Mapped | 20/21 | ⚠️ O-04 needs multiple refs |
| AC Mapped | 21/21 | ✅ All have criteria |
| Deps Documented | 21/21 | ✅ All have dependencies |
| Missing Beads | 2 | INT-01, INT-02 needed |

---

## Recommended Fixes Before Ralph Loop

1. **Add O-04 source list** to CROSS-REFERENCE-MATRIX.md
2. **Create INT-01 bead** for module composition
3. **Create INT-02 bead** for smoke test
4. **Update P-03, P-04** to depend on K-02

---

**Status: 95% ready for Ralph loop execution**
