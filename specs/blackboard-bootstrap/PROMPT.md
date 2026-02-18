# PROMPT: Bootstrap Agentic Blackboard

## START HERE

**Read `specs/blackboard-bootstrap/CURSOR-BOOTSTRAP.md` first.**

That file contains everything you need to operate this system in Cursor.

---

## Guiding Principle

> "Ask me clarifying questions until you know what I want to build and walk me through the setup step by step."

The system is self-configuring. If the user describes what they want, you derive the configuration from conversation — no human expert needed to set it up for them.

---

## Quick Start

1. Read `specs/blackboard-bootstrap/CURSOR-BOOTSTRAP.md`
2. Ask the human clarifying questions
3. Bootstrap the workshop
4. Create shavings as Showboat documents
5. Verify with `showboat verify`
6. Hand off session state

---

## File Hierarchy

```
specs/blackboard-bootstrap/
├── CURSOR-BOOTSTRAP.md    ← START HERE
├── PRD.md                 ← What we're building
├── ARCHITECTURE.md        ← Full system design
├── ACCEPTANCE-CRITERIA.md ← How to verify
├── CONCEPT-CONTEXT-SPACE.md ← Why this works
├── DOMAIN-PROOF-OF-WORK.md ← Showboat integration
├── ONE-PAGER-AI.md        ← For AI agents
├── ONE-PAGER-HUMAN.md     ← For humans
├── ONE-PAGER-PITCH.md     ← Why it matters
├── SETUP-GUIDE.md         ← Self-configuring setup
└── progress.txt           ← Session notes
```

---

## After Compaction

Load the last bead and continue. Do not start new work.

```bash
bd list -s in_progress
```

1. Find the next unblocked P0 task using `bd list -t task -s open | grep "P0"`
2. Check for dependencies with `bd dep tree <task-id>`
3. Claim the task with `bd update <task-id> --status in_progress`
4. Implement the task (create file, write code, etc.)
5. Commit changes with git
6. Update progress.txt with what you did
7. Mark task complete with `bd update <task-id> --status closed`

ONLY DO ONE TASK AT A TIME.

When complete, print "DONE"

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

### Key Files
- `specs/blackboard-bootstrap/ARCHITECTURE.md` - Full domain decomposition
- `specs/blackboard-bootstrap/PRD.md` - Product requirements
- `specs/blackboard-bootstrap/progress.txt` - Session notes
- `specs/blackboard-bootstrap/TOOL-ARCHITECTURE.md` - Tool file format

### Source Repos
- `~/arscontexta` - Research claims, 6-phase setup
- `~/agno` - Memory, knowledge, guardrails
- `~/Dicklesworthstone/coding_agent_session_search` - Cursor SQLite connector
