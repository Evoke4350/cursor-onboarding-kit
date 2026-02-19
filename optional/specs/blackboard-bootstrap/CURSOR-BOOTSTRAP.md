# CURSOR BOOTSTRAP: Agentic Blackboard

**The only file an agentic operator needs to read.**

---

## You Are an Agentic Operator

You are an AI agent operating in Cursor. Your job is to:
1. Understand the workspace
2. Extract knowledge as you work
3. Prove what you did
4. Persist everything for future operators

This file tells you how.

---

## The System in One Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    AGENTIC BLACKBOARD IN CURSOR                  │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    CURSOR CONTEXT                         │   │
│  │                                                          │   │
│  │   .cursor/hooks.json ──→ Gate 1: before_file_read       │   │
│  │                      ──→ Gate 2: before_shell_execution │   │
│  │                      ──→ Gate 3: before_mcp_execution   │   │
│  │                                                          │   │
│  │   .cursorignore ──→ Hard deny (reliable)                │   │
│  │                                                          │   │
│  │   state.vscdb ──→ Undocumented: full conversation       │   │
│  │                   history (read-only, no hooks)          │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    WORKSHOP (Blackboard)                  │   │
│  │                                                          │   │
│  │   bench/           ──→ Identity, methodology, MOCs       │   │
│  │   shavings/        ──→ Atomic insights (Showboat docs)   │   │
│  │   sawdust/         ──→ Sessions, queues, ephemera        │   │
│  │   .workshop/       ──→ Tools, policy, config             │   │
│  │                                                          │   │
│  │   Commands: /cut /carve /chamfer /check /sharpen        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                 PROOF OF WORK                             │   │
│  │                                                          │   │
│  │   Each shaving = Showboat document                       │   │
│  │   - Commentary (what you thought)                        │   │
│  │   - Executable code (what you ran)                       │   │
│  │   - Captured output (what happened)                      │   │
│  │   - Verifiable (showboat verify)                         │   │
│  │                                                          │   │
│  │   Charts: chartroom with auto-alt text                   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## The Core Concept

**Parameter space is unmanageable. Context space is the answer.**

From arXiv:2510.08191 (Training-Free GRPO):
> LLMs can achieve similar output distribution effects by learning experiential knowledge as a token prior.

**Your shavings ARE the experience library.** You:
1. Introspect your attempts
2. Extract what worked as natural language rules
3. Store in `shavings/`
4. Inject as context (token prior) in future prompts

No fine-tuning. No RL. Just context.

---

## Bootstrapping a New Workshop

### Phase 1: Detect
```bash
# Check environment
uname -s                    # macOS/Linux/Windows
which gum ripgrep uvx       # Available tools?
ls -la .cursor/             # Cursor config exists?
```

### Phase 2: Understand
Ask the human:
1. What kind of work do you do? (software/research/writing)
2. Should shavings reference code? (yes/no/optional)
3. Where should the workshop live? (./workshop ~/workshop)
4. Security level? (basic/paranoid/development)

### Phase 3: Generate
```bash
mkdir -p workshop/{bench,shavings,sawdust,.workshop/{tools,templates,policy}}

# Create identity
cat > workshop/bench/identity.md << 'EOF'
---
created: 2026-02-18
work_type: software
code_refs: optional
---

# Workshop Identity

This workshop belongs to [human name]. I help with [work type].

My methodology: Extract insights, find connections, update older work.
EOF

# Create policy (if security needed)
cat > workshop/.workshop/policy/sources.yaml << 'EOF'
sources:
  - pattern: ".secrets/*"
    taint: high
  - pattern: "*.env"
    taint: high
  - pattern: "credentials.*"
    taint: high
EOF

cat > workshop/.workshop/policy/sinks.yaml << 'EOF'
sinks:
  - command: "curl"
    block_if_tainted: true
  - command: "wget"
    block_if_tainted: true
  - command: "rsync"
    block_if_tainted: true
EOF

# Create .cursorignore (reliable hard deny)
cat > .cursorignore << 'EOF'
.secrets/
*.env
*.pem
*.key
credentials.*
EOF
```

### Phase 4: Validate
```bash
# Check structure
ls -la workshop/bench/identity.md
ls -la workshop/.workshop/policy/
test -f .cursorignore && echo "Security configured"
```

---

## Your Commands

### /cut — Extract Insight
```bash
# Create a new shaving
uvx showboat init workshop/shavings/$(date +%Y%m%d-%H%M)-insight.md "Insight: [title]"
uvx showboat note workshop/shavings/*.md "I discovered that..."
uvx showboat exec workshop/shavings/*.md bash "command you ran"
# Output captured automatically
```

### /carve — Find Connections
```bash
# Search all shavings for connections
ripgrep -i "keyword" workshop/shavings/
# Or with semantic search (if configured)
# qmd query "find related concepts"
```

### /chamfer — Update Older Work
```bash
# Edit older shaving with new context
# Showboat docs are just markdown
```

### /check — Verify Everything
```bash
# Verify all shavings
for f in workshop/shavings/*.md; do
  uvx showboat verify "$f" || echo "FAILED: $f"
done
```

### /sharpen — Refine System
```bash
# Update workshop/bench/methodology.md
# Refine policy in workshop/.workshop/policy/
# Adjust identity based on experience
```

---

## Security Gates

### Gate 1: Before File Read
If reading `.secrets/*`, `*.env`, `credentials.*`:
- Mark conversation as **tainted**
- Continue (allow read)

### Gate 2: Before Shell Execution
If conversation **tainted** and command is `curl`, `wget`, `rsync`, `scp`:
- **BLOCK** execution
- Tell human: "Exfil blocked: conversation tainted"

### Gate 3: Before MCP Execution
If conversation **tainted** and tool can send data externally:
- **BLOCK** execution
- Tell human: "MCP tool blocked: conversation tainted"

### Reliable Alternative: .cursorignore
Put secrets in `.cursorignore` for **hard deny**. This is more reliable than hooks.

---

## Session Handoff Protocol

### When You Stop
```bash
# Create session log
cat > workshop/sawdust/sessions/$(date +%Y%m%d-%H%M%S).md << EOF
# Session: $(date)

## Completed
- [what you did]

## In Progress
- [what was interrupted]

## Next
- [what should happen next]

## Context
- Files modified: [list]
- Shavings created: [list]
- Taint state: [clean/tainted]
EOF
```

### When You Resume
```bash
# Read last session
ls -t workshop/sawdust/sessions/ | head -1 | xargs cat

# Read identity
cat workshop/bench/identity.md

# Continue from where you left off
```

---

## The Magic Phrase

> "Ask me clarifying questions until you know what I want to build and walk me through the setup step by step."

This system is **self-configuring**. If a human describes what they want, you derive the configuration from conversation. No expert needed.

---

## Files You Must Understand

| File | Purpose |
|------|---------|
| `specs/blackboard-bootstrap/CURSOR-BOOTSTRAP.md` | This file — start here |
| `specs/blackboard-bootstrap/PRD.md` | What we're building |
| `specs/blackboard-bootstrap/ARCHITECTURE.md` | Full system design |
| `specs/blackboard-bootstrap/CONCEPT-CONTEXT-SPACE.md` | Why context > parameters |
| `specs/blackboard-bootstrap/ACCEPTANCE-CRITERIA.md` | How to verify success |

---

## Quick Reference

```bash
# Bootstrap
uvx showboat init workshop/shavings/new.md "Title"
uvx showboat note workshop/shavings/new.md "Commentary"
uvx showboat exec workshop/shavings/new.md bash "code"

# Verify
uvx showboat verify workshop/shavings/*.md

# Chart
uvx chartroom bar --csv data.csv -f markdown

# Search
ripgrep "pattern" workshop/shavings/

# Session
cat workshop/sawdust/sessions/latest.md
```

---

## Success Criteria

An agentic operator has succeeded when:
1. ✅ Can bootstrap a workshop without human expert
2. ✅ Creates Showboat shavings that verify
3. ✅ Respects security gates (taint analysis)
4. ✅ Hands off session state for next operator
5. ✅ Proves work through executable documents

---

**End of bootstrap. Start here. Ask questions until you understand. Walk the human through setup step by step.**
