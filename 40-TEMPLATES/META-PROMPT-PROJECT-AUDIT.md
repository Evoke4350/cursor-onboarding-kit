# Meta-Prompt: Agentic Blackboard Project Overview & Usage Guide Generator

Use this prompt to get any LLM to generate a complete project audit and usage guide.

---

## The Prompt

```
You are auditing the Agentic Blackboard project. Generate a comprehensive overview and usage guide.

## Part 1: Project Inventory

Scan the project and list:

### Layer 1: Operator Instructions
- Location: root directory, numbered 00-99*.md files
- Count all instruction files
- Identify the "must read" entry points
- List the key workflow patterns documented

### Layer 2: Workshop CLI
- Location: workshop-cli/
- List all implemented commands
- Check test coverage
- Identify what's wired vs stubbed

### Configuration Files
- AGENTS.md / CLAUDE.md (operator instructions)
- .cursor/hooks.json (security gates)
- .cursorignore (hard deny)
- workshop/.workshop/policy/*.yaml (taint policy)

## Part 2: Integration Status

For each component, assess:

| Component | Built | Tested | Integrated | Status |
|-----------|-------|--------|------------|--------|
| Taint tracking | ? | ? | ? | PRE/TRANS |
| 5 Cs pipeline | ? | ? | ? | PRE/TRANS |
| Cursor SQLite reader | ? | ? | ? | PRE/TRANS |
| Security gates | ? | ? | ? | PRE/TRANS |

Status definitions:
- PRE: Code exists but not wired into actual workflow
- TRANS: Fully integrated and working end-to-end

## Part 3: What Actually Works

List commands that:
1. Parse correctly (--help shows usage)
2. Execute without error
3. Produce expected output
4. Have passing tests

Be honest about gaps.

## Part 4: Step-by-Step Usage Guide

For a new operator starting fresh, provide:

### Day 0: Setup
- Prerequisites (what tools needed)
- Installation commands
- Verification steps

### Day 1: First Workshop
- How to initialize a workshop
- What files get created
- How to verify it worked

### Day 2: First Shaving
- How to cut a file
- What the output looks like
- How to verify it's valid

### Day 3: Security Check
- How to configure taint policy
- How to test the security gates
- How to verify protection works

### Ongoing: Workflow
- Daily usage patterns
- Session handoff protocol
- How to check project health

## Output Format

Keep output structured:
- Use tables for status
- Use code blocks for commands
- Be precise about what works vs what's aspirational
- No marketing language
- Single actionable next step at the end
```

---

## File Location

This meta-prompt lives in your operator prompt pack for project audits.
