# Workshop Scaffold

**Purpose**: The actual folder structure that gets created when the system is built.

---

## Root Structure

```
workshop/
├── bench/                    # K-02: Identity, methodology, MOCs
│   ├── identity.md           # Who this workshop serves
│   ├── methodology.md        # How we work
│   └── moc/                  # Maps of Content
│       ├── security.md       # Security domain MOC
│       ├── cursor.md         # Cursor integration MOC
│       └── pipeline.md       # Processing pipeline MOC
│
├── shavings/                 # K-02: Atomic insights (Showboat docs)
│   ├── 2026-02-18-*.md       # Date-prefixed insight files
│   └── ...
│
├── sawdust/                  # K-02: Operational ephemera
│   ├── sessions/             # Session logs
│   ├── state/                # Compaction recovery state
│   │   ├── current-task.md
│   │   └── taint-state.txt
│   ├── audit/                # Security audit log
│   └── queue.json            # P-03: Task queue
│
└── .workshop/                # K-02: System configuration
    ├── config.yaml           # Derived configuration
    ├── derivation.md         # Why configured this way
    │
    ├── policy/               # S-01, S-02: Security policy
    │   ├── sources.yaml      # Protected paths
    │   └── sinks.yaml        # Exfiltration sinks
    │
    ├── tools/                # T-01, T-02: Tool definitions
    │   ├── core/
    │   │   ├── cut.md        # P-01: Extract insight
    │   │   ├── carve.md      # P-02: Find connections
    │   │   ├── chamfer.md    # P-03: Update older work
    │   │   ├── check.md      # P-04: Validate everything
    │   │   └── sharpen.md    # Meta: Refine system
    │   └── setup/
    │       ├── init.md       # O-04: Bootstrap workshop
    │       └── calibrate.md  # O-04: Adjust configuration
    │
    └── templates/            # O-04: Generation templates
        ├── identity.md.tmpl
        └── policy.yaml.tmpl
```

---

## Rust Project Structure

```
src/
├── main.rs                   # CLI entry point
├── cli.rs                    # R-01: clap definitions
│
├── modules/
│   ├── security/             # S-01, S-02, S-03, S-04
│   │   ├── mod.rs
│   │   ├── guardrail.rs      # From agno/guardrails/base.py
│   │   ├── taint.rs          # From Universalis concept
│   │   └── policy.rs         # sources.yaml + sinks.yaml
│   │
│   ├── cursor/               # C-01, C-02, C-03, C-04
│   │   ├── mod.rs
│   │   ├── sqlite.rs         # From cursor.rs:156-239
│   │   ├── composer.rs       # From cursor.rs:422-593
│   │   └── bubble.rs         # From cursor.rs:245-282
│   │
│   ├── memory/               # K-01, K-02, K-03
│   │   ├── mod.rs
│   │   ├── schema.rs         # From memory.py:8-58
│   │   └── manager.rs        # From manager.py:42-1543
│   │
│   ├── pipeline/             # P-01, P-02, P-03, P-04
│   │   ├── mod.rs
│   │   ├── phases.rs         # From processing-pipeline.md
│   │   └── queue.rs          # Task queue
│   │
│   ├── setup/                # O-01, O-02, O-03, O-04
│   │   ├── mod.rs
│   │   ├── detect.rs         # Environment detection
│   │   ├── derive.rs         # From claude-md.md
│   │   └── generate.rs       # File generation
│   │
│   ├── tools/                # T-01, T-02
│   │   ├── mod.rs
│   │   ├── format.rs         # Tool definition format
│   │   └── templates.rs      # Feature blocks
│   │
│   ├── cli/                  # R-01, R-02, R-03
│   │   ├── mod.rs
│   │   ├── perf.rs           # From perf.rs:35-100
│   │   ├── search.rs         # From hybrid.rs:1-859
│   │   └── output.rs         # Robot/human modes
│   │
│   └── proof/                # W-01, W-02, W-03, W-04, W-05
│       ├── mod.rs
│       ├── showboat.rs       # CLI wrapper
│       ├── verify.rs         # Document verification
│       └── chart.rs          # Chartroom wrapper
│
├── compose.rs                # Module composition
└── lib.rs                    # Library exports
```

---

## File → Bead Mapping

| File | Bead | Domain |
|------|------|--------|
| `.workshop/policy/sources.yaml` | S-01 (y6k) | Security |
| `.workshop/policy/sinks.yaml` | S-02 (bnt) | Security |
| `src/modules/security/mod.rs` | S-03 (irt) | Security |
| `.cursorignore` | S-04 (go5) | Security |
| `src/modules/cursor/sqlite.rs` | C-01 (34k) | Cursor |
| `src/modules/cursor/composer.rs` | C-02 (e43) | Cursor |
| `src/modules/cursor/bubble.rs` | C-03 (ivj) | Cursor |
| `src/modules/cursor/mod.rs` | C-04 (hhy) | Cursor |
| `src/modules/memory/schema.rs` | K-01 (idg) | Knowledge |
| `workshop/` structure | K-02 (zor) | Knowledge |
| `src/modules/memory/manager.rs` | K-03 (r5s) | Knowledge |
| `src/modules/pipeline/phases.rs` | P-01 (q2n) | Pipeline |
| `.workshop/tools/core/carve.md` | P-02 (k9o) | Pipeline |
| `.workshop/tools/core/chamfer.md` | P-03 (6vu) | Pipeline |
| `.workshop/tools/core/check.md` | P-04 (qpu) | Pipeline |
| `src/modules/setup/detect.rs` | O-01 (x56) | Setup |
| `src/modules/setup/detect.rs` | O-02 (kj0) | Setup |
| `src/modules/setup/derive.rs` | O-03 (lgv) | Setup |
| `src/modules/setup/generate.rs` | O-04 (cy5) | Setup |
| `src/modules/tools/format.rs` | T-01 (75g) | Tools |
| `src/modules/tools/templates.rs` | T-02 (gkq) | Tools |
| `Cargo.toml` | R-01 (cb4) | CLI |
| `src/modules/cli/perf.rs` | R-02 (1b3) | CLI |
| `src/modules/cli/output.rs` | R-03 (cj2) | CLI |
| `src/modules/proof/showboat.rs` | W-01 (lre) | Proof |
| `shavings/*.md` | W-02 (6dc) | Proof |
| `src/modules/proof/verify.rs` | W-03 (3d5) | Proof |
| `src/modules/proof/showboat.rs` | W-04 (1xr) | Proof |
| `src/modules/proof/chart.rs` | W-05 (xbb) | Proof |

---

## Initialization Sequence

When `blackboard init` runs (O-04):

```bash
# 1. Detect environment
detect_env() -> Environment

# 2. Ask questions (via gum if available)
ask_questions() -> Answers

# 3. Derive configuration
derive_config(answers) -> DerivedConfig

# 4. Create folder structure
mkdir -p workshop/{bench,shavings,sawdust,.workshop/{policy,tools,templates}}

# 5. Generate files
generate_identity(config) -> workshop/bench/identity.md
generate_policy(config) -> workshop/.workshop/policy/{sources,sinks}.yaml
generate_tools(config) -> workshop/.workshop/tools/core/*.md

# 6. Create .cursorignore if security enabled
if config.security_level != "none":
    generate_cursorignore() -> .cursorignore

# 7. Initialize sawdust state
mkdir -p workshop/sawdust/{sessions,state,audit}
touch workshop/sawdust/state/{current-task.md,taint-state.txt}
```

---

## Dependency Order for Implementation

Based on bead dependencies, implement in this order:

### Phase 1: Foundation (no dependencies)
```
R-01 (cb4) - Cargo.toml setup
K-01 (idg) - UserMemory schema
T-01 (75g) - Tool file format
S-01 (y6k) - sources.yaml schema
```

### Phase 2: Core (depends on Phase 1)
```
S-02 (bnt) - sinks.yaml (depends on S-01)
K-02 (zor) - Workshop structure (depends on K-01)
C-01 (34k) - SQLite reader (no deps, but needs R-01)
T-02 (gkq) - Tool templates (depends on T-01)
R-02 (1b3) - Performance budgets (depends on R-01)
```

### Phase 3: Integration (depends on Phase 2)
```
C-02 (e43) - composerData (depends on C-01)
C-03 (ivj) - bubbleId (depends on C-01)
P-01 (q2n) - /cut command (depends on K-02)
W-01 (lre) - Showboat CLI (no deps)
W-02 (6dc) - Shavings as Showboat (depends on K-02)
```

### Phase 4: Commands (depends on Phase 3)
```
C-04 (hhy) - Normalize (depends on C-02)
P-02 (k9o) - /carve (depends on P-01)
W-03 (3d5) - /check verify (depends on W-02)
O-01 (x56) - Gum prompts (no deps)
```

### Phase 5: Setup (depends on Phase 4)
```
P-03 (6vu) - /chamfer (depends on P-02)
O-02 (kj0) - Detect phase (depends on O-01)
O-03 (lgv) - Understand phase (depends on O-02)
```

### Phase 6: Generate (depends on Phase 5)
```
P-04 (qpu) - /check validate (depends on P-03)
O-04 (cy5) - Generate phase (depends on O-03)
```

### Phase 7: Polish (P1 tasks)
```
S-03 (irt) - Security packs
S-04 (go5) - .cursorignore docs
K-03 (r5s) - MOC generation
R-03 (cj2) - Output modes
W-04 (1xr) - Remote streaming
W-05 (xbb) - Chartroom
```
