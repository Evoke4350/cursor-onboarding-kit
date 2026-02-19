# PROMPT: Bootstrap Agentic Blackboard

## START HERE

**Read `specs/blackboard-bootstrap/CURSOR-BOOTSTRAP.md` first.**

That file contains everything you need to operate this system in Cursor.

---

## Guiding Principle

> "Ask me clarifying questions until you know what I want to build and walk me through the setup step by step."

The system is self-configuring. If the user describes what they want, you derive the configuration from conversation — no human expert needed to set it up for them.

---

## Ralph Loop Protocol

You are executing a **Ralph loop**: fresh context per iteration, state in files.

### Iteration Sequence

```
┌─────────────────────────────────────────────────────────────┐
│  1. ORIENT                                                  │
│     ├─ Read CURSOR-BOOTSTRAP.md                             │
│     ├─ Find next P0 task                                    │
│     └─ Check dependencies resolved                          │
├─────────────────────────────────────────────────────────────┤
│  2. CLAIM                                                   │
│     └─ bd update <task-id> --status in_progress             │
├─────────────────────────────────────────────────────────────┤
│  3. EXECUTE                                                 │
│     ├─ Read source from CROSS-REFERENCE-MATRIX.md           │
│     ├─ Read acceptance criteria from ACCEPTANCE-CRITERIA.md │
│     ├─ Implement task                                       │
│     └─ Verify ALL ACs pass                                  │
├─────────────────────────────────────────────────────────────┤
│  4. COMMIT                                                  │
│     ├─ git add <specific files>                             │
│     ├─ git commit (format below)                            │
│     └─ git push                                             │
├─────────────────────────────────────────────────────────────┤
│  5. CLOSE                                                   │
│     ├─ bd update <task-id> --status closed                  │
│     ├─ Update progress.txt                                  │
│     └─ echo "DONE"                                          │
└─────────────────────────────────────────────────────────────┘
```

---

## Commit Format

```
<type>(<scope>): <description>

Task: <task-id>
Source: <source-file>:<lines>
Acceptance: <ac-ids>

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
```

**ONE TASK = ONE COMMIT = ONE PUSH**

---

## After Compaction

If context was reset, recover state:

```bash
# 1. Check current task
cat workshop/sawdust/state/current-task.md

# 2. Check taint state
cat workshop/sawdust/state/taint-state.txt

# 3. Check git
git status && git stash list

# 4. Resume task (do NOT start new)
bd update <task-id> --status in_progress
```

Read: `specs/blackboard-bootstrap/COMPACTION-PROTOCOL.md`

---

## Task Priority

### P0 (31 tasks) - Implement First
```
Security:  S-01, S-02
Cursor:    C-01, C-02, C-03, C-04
Knowledge: K-01, K-02
Pipeline:  P-01, P-02, P-03, P-04
Setup:     O-01, O-02, O-03, O-04
Tools:     T-01, T-02
CLI:       R-01, R-02
Proof:     W-01, W-02, W-03
Integration: INT-01, INT-02
```

### P1 (8 tasks) - Polish Later
```
Security:  S-03, S-04
Knowledge: K-03
CLI:       R-03
Proof:     W-04, W-05
```

---

## File Hierarchy

```
specs/blackboard-bootstrap/
├── CURSOR-BOOTSTRAP.md    ← START HERE
├── PRD.md                 ← What we're building
├── ARCHITECTURE.md        ← Full system design
├── ACCEPTANCE-CRITERIA.md ← How to verify (141 ACs)
├── CROSS-REFERENCE-MATRIX.md ← Source code for each task
├── SCAFFOLD.md            ← Folder structure to create
├── TAINT-FORMAL-SPEC.md   ← Taint analysis specification
├── COMPACTION-PROTOCOL.md ← State preservation
├── COMMIT-PROTOCOL.md     ← Git discipline
├── GAP-TO-95-PERCENT.md   ← What's needed for production
└── progress.txt           ← Session notes
```

---

## Quick Reference

### Task Prefixes
- **S-** = Security domain
- **C-** = Cursor Integration domain
- **K-** = Knowledge/Memory domain
- **P-** = Processing Pipeline domain
- **O-** = Setup/Onboarding domain
- **T-** = Tool Architecture domain
- **R-** = CLI/Runtime domain
- **W-** = Proof of Work domain
- **INT-** = Integration domain

### Key Commands
```bash
# Find next task
bd list -t task -s open | grep "P0" | head -1

# Check dependencies
bd dep tree <task-id>

# Claim task
bd update <task-id> --status in_progress

# Commit and push
git add <files> && git commit && git push

# Close task
bd update <task-id> --status closed
```

### Source Repos
- `~/arscontexta` - Research claims, 6-phase setup
- `~/agno` - Memory, knowledge, guardrails
- `~/Dicklesworthstone/coding_agent_session_search` - Cursor SQLite connector
- `~/Dicklesworthstone/destructive_command_guard` - Performance budgets
- `~/Dicklesworthstone/xf` - Hybrid search
- `~/showboat` - Executable documents
- `~/chartroom` - Charts with alt text

---

## Success Criteria

Each iteration succeeds when:

1. **One task completed** — Status changed to closed
2. **All ACs verified** — Every acceptance criterion passes
3. **Committed AND pushed** — Changes in git history
4. **DONE printed** — Signals loop to continue

---

## Loop Termination

- All P0 tasks closed, OR
- Human intervention, OR
- Three consecutive gaps found

---

**End of prompt. Execute the loop. One task at a time.**
