# Risk Assessment: Agentic Blackboard

**Applying arscontexta methodology + confidence thresholds + shape-spec process**

---

## Likelihood of Success: 70%

| Factor | Score | Reasoning |
|--------|-------|-----------|
| **Technical feasibility** | HIGH (90%) | All components exist in source repos |
| **Integration complexity** | MEDIUM (60%) | Cursor hooks unreliable, SQLite is stable |
| **Scope creep risk** | HIGH RISK | 7 domains, 24 tasks, grew from 1 to many |
| **Resource availability** | HIGH (85%) | User has budget, Ralph loop pattern established |
| **Acceptance criteria** | LOW (30%) | Not defined yet — see below |

---

## Acceptance Criteria (Missing — Proposed)

### Domain 1: Security & Taint

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| S-AC1 | `.secrets/*` read marks conversation tainted | 1.0 (deterministic) |
| S-AC2 | Tainted conversation blocks `curl` command | 0.9 (high confidence) |
| S-AC3 | `.cursorignore` hard-deny works | 0.95 (documented by Cursor) |
| S-AC4 | Hook reliability documented (known bugs) | 1.0 (informational) |

### Domain 2: Cursor Integration

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| C-AC1 | Can read `state.vscdb` without error | 0.95 (proven in Dicklesworth) |
| C-AC2 | Extract full conversation with messages | 0.85 (multiple formats supported) |
| C-AC3 | Parse v0.40+ bubbleId format | 0.8 (documented, tested) |
| C-AC4 | Workspace context extracted | 0.7 (varies by Cursor version) |

### Domain 3: Knowledge & Memory

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| K-AC1 | UserMemory schema defined | 1.0 (from Agno) |
| K-AC2 | Shavings persist as markdown | 1.0 (deterministic) |
| K-AC3 | MOC auto-generation | 0.6 (requires heuristics) |
| K-AC4 | Semantic search works | 0.7 (ripgrep reliable, vectors optional) |

### Domain 4: Processing Pipeline

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| P-AC1 | `/cut` extracts insight with code ref | 0.85 (pattern exists) |
| P-AC2 | `/carve` finds wiki links | 0.8 (ripgrep-based) |
| P-AC3 | `/chamfer` updates older notes | 0.7 (requires context matching) |
| P-AC4 | `/check` validates schema | 0.9 (deterministic) |

### Domain 5: Setup & Onboarding

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| O-AC1 | Gum prompts display correctly | 0.95 (proven CLI) |
| O-AC2 | Phase 1 detect environment | 0.9 (uname exists) |
| O-AC3 | Phase 5 generates folders | 1.0 (mkdir) |
| O-AC4 | 6-phase process completes | 0.8 (script complexity) |

### Domain 6: Tool Architecture

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| T-AC1 | Tools as markdown files | 1.0 (deterministic) |
| T-AC2 | Cursor discovers via rg | 0.85 (documented pattern) |
| T-AC3 | Performance budgets defined | 1.0 (specification) |

### Domain 7: CLI & Runtime

| ID | Criterion | Confidence Threshold |
|----|-----------|---------------------|
| R-AC1 | Rust project compiles | 0.95 (Cargo) |
| R-AC2 | Performance budget types defined | 1.0 (types) |
| R-AC3 | Robot mode JSON output | 0.9 (serde) |
| R-AC4 | Sub-millisecond search | 0.6 (SIMD tricky) |

---

## Risk Register

### HIGH RISK

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Scope creep** | 80% | HIGH | WIP limits, prioritize P0 only |
| **Cursor hook reliability** | 70% | HIGH | Use `.cursorignore` as primary, hooks as secondary |
| **No clear MVP** | 60% | HIGH | Define "done" for first pass |

### MEDIUM RISK

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **SQLite format changes** | 40% | MEDIUM | Version detection, multiple format support |
| **Context pollution** | 50% | MEDIUM | Ralph loop with fresh context per task |
| **Integration complexity** | 50% | MEDIUM | Incremental integration, test each gate |

### LOW RISK

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Rust compile issues** | 20% | LOW | Well-documented patterns |
| **Markdown portability** | 10% | LOW | Standard format |
| **Git workflow** | 30% | LOW | Established patterns |

---

## Confidence-Gated Response Pattern

From arscontexta methodology:

| Confidence | Action | Example |
|------------|--------|---------|
| **> 0.9** | AUTO-APPLY | Schema validation, file creation |
| **0.7 - 0.9** | SUGGEST | MOC generation, semantic search |
| **< 0.7** | LOG ONLY | Hook reliability, advanced features |

Apply this to task execution:
- P0 tasks with >0.9 confidence → execute immediately
- P0 tasks with 0.7-0.9 → execute with verification
- P1/P2 tasks → defer until P0 complete

---

## Success Definition (Proposed)

### Minimum Viable Blackboard (MVB)

```
┌─────────────────────────────────────────────────────────┐
│                    MVB CHECKLIST                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  [ ] C-AC1: Can read Cursor SQLite                     │
│  [ ] S-AC1: .secrets/* triggers taint                  │
│  [ ] S-AC2: Taint blocks exfil commands                │
│  [ ] K-AC2: Shavings persist as markdown               │
│  [ ] P-AC1: /cut command works                         │
│  [ ] O-AC3: Setup creates folder structure             │
│                                                         │
│  = 6 criteria for MVB                                  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Full System (Post-MVB)

```
┌─────────────────────────────────────────────────────────┐
│                    FULL SYSTEM                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  [ ] All 24 tasks complete                             │
│  [ ] All 28 acceptance criteria met                    │
│  [ ] 3 one-pagers reviewed                             │
│  [ ] Ralph loop tested for 10 iterations               │
│  [ ] Attack scenario verified (taint blocks exfil)     │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Recommended Next Steps

1. **Define MVP criteria** — User must confirm MVB checklist
2. **Prioritize P0 tasks** — 18 P0 tasks, focus on first 6
3. **Set WIP limit** — Max 3 tasks in progress
4. **Execute Ralph loop** — Start with C-01 (Cursor SQLite reader)

---

## Meta-Assessment

Using arscontexta's confidence threshold pattern:

- This assessment has **0.7 confidence** — I'm moderately certain
- Weakness: Acceptance criteria invented, not user-validated
- Strength: Based on proven patterns from Dicklesworth, Agno, arscontexta
- Risk: Over-engineering before validation

**Recommendation**: Validate MVB criteria with user before proceeding.
