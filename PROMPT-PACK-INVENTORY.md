# Prompt Pack Inventory - Agent OS Bootstrap

**Purpose:** Exhaustive inventory of files to include in agent-os for bootstrapping new projects.

---

## Directory Structure (Proposed Renaming)

Based on "AI Agents Need State" concepts - checkpoint, working memory, long-term memory, identity:

```
my-project/
├── AGENTS.md                    # Project instructions (team policy)
├── AGENTS.local.md              # Local overrides (gitignored)
├── CLAUDE.md → AGENTS.md        # Symlink (Claude Code compat)
├── CLAUDE.local.md → AGENTS.local.md  # Symlink (local overrides)
│
├── identity/                    # Former: bench/
│   └── identity.md              # Who/what/why of this project
│
├── memory/                      # Former: shavings/ (long-term)
│   ├── decisions/               # Decision records (ADR-style)
│   ├── patterns/                # Discovered patterns
│   └── insights/                # Captured learnings
│
├── session/                     # Former: sawdust/ (short-term)
│   ├── logs/                    # Session logs (YYYYMMDD-HHMM.md)
│   └── scratch/                 # Ephemeral working notes
│
├── checkpoint/                  # Former: sawdust/state/
│   ├── current.yaml             # Recoverable state
│   └── history/                 # Checkpoint snapshots
│
├── .cursor/
│   └── rules/                   # Cursor rules (.mdc files)
│
├── .github/
│   ├── copilot-instructions.md  # GitHub Copilot instructions
│   └── instructions/            # Per-domain instructions
│
└── .agent/
    ├── vocabulary.yaml          # Semantic command mapping
    ├── derivation.yaml          # Path delimiters
    └── config.yaml              # Agent configuration
```

### Naming Rationale

| Old Name | New Name | Why |
|----------|----------|-----|
| `bench/` | `identity/` | Clearer purpose - project identity |
| `shavings/` | `memory/` | Generic, works everywhere |
| `sawdust/` | `session/` | Session-scoped, ephemeral |
| `sawdust/state/` | `checkpoint/` | Recovery checkpoint, standard term |

---

## File Inventory

### Core Instruction Files

| File | Purpose | Required |
|------|---------|----------|
| `AGENTS.md` | Team policy, project rules, workflow | ✅ Yes |
| `AGENTS.local.md` | Personal overrides (gitignored) | ✅ Template |
| `CLAUDE.md` | Symlink to AGENTS.md | ✅ Yes |
| `CLAUDE.local.md` | Symlink to AGENTS.local.md | ✅ Template |

### Cursor Rules (`.cursor/rules/`)

| File | Purpose | When to Use |
|------|---------|-------------|
| `00-project-basics.mdc` | Core project constraints | Always (alwaysApply: true) |
| `93-personal-constraints.mdc` | Personal preferences | Per-project customization |
| `97-positive-boolean-naming.mdc` | Naming conventions | Code files |

### GitHub Instructions (`.github/`)

| File | Purpose |
|------|---------|
| `copilot-instructions.md` | GitHub Copilot integration |
| `instructions/*.instructions.md` | Domain-specific rules (e.g., telemetry, mobile-ui) |

### Agent Configuration (`.agent/`)

| File | Schema | Purpose |
|------|--------|---------|
| `vocabulary.yaml` | See below | Maps semantic IDs → commands |
| `derivation.yaml` | See below | Maps semantic delimiters → paths |
| `config.yaml` | Simple key-value | Agent-specific settings |

### Identity (`identity/`)

| File | Schema | Purpose |
|------|--------|---------|
| `identity.md` | Markdown | Project purpose, context, conventions |

### Memory (`memory/`)

| Subdirectory | Format | Purpose |
|--------------|--------|---------|
| `decisions/` | `YYYYMMDD-title.md` | Decision records (ADR-style) |
| `patterns/` | `pattern-name.md` | Discovered codebase patterns |
| `insights/` | `YYYYMMDD-topic.md` | Captured learnings |

### Session (`session/`)

| Subdirectory | Format | Purpose |
|--------------|--------|---------|
| `logs/` | `YYYYMMDD-HHMM.md` | Immutable session logs |
| `scratch/` | Any | Ephemeral working notes |

### Checkpoint (`checkpoint/`)

| File | Schema | Purpose |
|------|--------|---------|
| `current.yaml` | See below | Recoverable agent state |
| `history/*.yaml` | Timestamped snapshots | Checkpoint history |

---

## Schemas

### checkpoint/current.yaml

```yaml
# Checkpoint - Current recoverable state
# Purpose: Enable resumption after interruption

task:
  id: beads-issue-id | null
  title: "Current task description"
  type: bug | feature | refactor | docs | null
  priority: 0-4 | null

phase: idle | triage | scope | implement | verify | deliver | blocked
phase_entered: "2026-02-18T17:00:00Z"

position:
  file: "path/to/file.ts" | null
  line: 42 | null
  context: ["related file 1", "related file 2"]
  url: "https://..." | null

decisions:
  - timestamp: "2026-02-18T17:00:00Z"
    question: "What was decided"
    choice: "What was chosen"
    rationale: "Why"

blockers:
  - description: "What's blocking"
    since: "2026-02-18T17:00:00Z"
    unblocks_on: "condition" | null

session:
  id: "20260218-1700"
  started: "2026-02-18T17:00:00Z"
  last_activity: "2026-02-18T17:30:00Z"
  log: "session/logs/20260218-1700.md"

transitions:
  - { from: "idle", to: "triage", at: "17:00", reason: "started work" }
```

### derivation.yaml

```yaml
# Derivation file - Maps semantic delimiters to paths
# Used by templates with {{MEMORY:*}} syntax

project:
  name: "My Project"
  slug: "my-project"
  type: software | research | writing | other
  created: "2026-02-18"

memory:
  identity: "identity/identity.md"
  long: "memory/"
  short: "session/"
  checkpoint: "checkpoint/"
  config: ".agent/"

workflow:
  capture: "memory capture"
  search: "memory search"
  update: "memory update"
  validate: "memory check"

# Semantic delimiter resolution:
# {{PROJECT_NAME}} → My Project
# {{MEMORY:identity}} → identity/identity.md
# {{MEMORY:long}} → memory/
# {{MEMORY:short}} → session/
# {{MEMORY:checkpoint}} → checkpoint/
# {{MEMORY:config}} → .agent/
```

### session/logs/YYYYMMDD-HHMM.md

```markdown
---
session_id: 20260218-1700
started: 2026-02-18T17:00:00Z
ended: 2026-02-18T18:00:00Z | null
agent: claude-opus-4-6
status: active | completed | interrupted
---

## Initial State

- **Task:** [What was being worked on]
- **Position:** [File:line or description]
- **Phase:** [Current phase]

## State Transitions

| Time | From | To | Trigger | Notes |
|------|------|-----|---------|-------|
| 17:00 | idle | triage | [prompt] | [details] |

## Decisions Made

### Decision 1: [Title]
- **Question:** [What was being decided]
- **Options:** [What was considered]
- **Choice:** [What was chosen]
- **Rationale:** [Why]

## Artifacts Created

- `memory/insights/20260218-topic.md` - [description]

## Final State

- **Task:** [Final status]
- **Position:** [Where work ended]
- **Phase:** [Final phase]
- **Blockers:** [Any remaining]

## For Next Session

1. Read `checkpoint/current.yaml`
2. Pick up at [position]
3. Continue with [next step]
```

---

## Templates (from 40-TEMPLATES/)

### Session Templates

| Template | Purpose | Location After Bootstrap |
|----------|---------|--------------------------|
| `TEMPLATE-session-log.md` | Session logging | session/logs/ |
| `TEMPLATE-handoff.md` | Session handoff | session/logs/ |

### Decision Templates

| Template | Purpose | Location After Bootstrap |
|----------|---------|--------------------------|
| `TEMPLATE-decision-record.md` | ADR-style decisions | memory/decisions/ |
| `TEMPLATE-rfc.md` | RFC documents | docs/ |

### Planning Templates

| Template | Purpose | Location After Bootstrap |
|----------|---------|--------------------------|
| `TEMPLATE-principles.md` | Project principles | identity/ |
| `TEMPLATE-runbook.md` | Operational runbooks | docs/runbooks/ |

### Meta-Prompts

| Template | Purpose |
|----------|---------|
| `META-PROMPT-PROJECT-AUDIT.md` | Audit current project |
| `META-PROMPT-AGENTIC-BLACKBOARD-ONE-PAGER.md` | Explain blackboard concept |

---

## Starter Pack Files (from 40-TEMPLATES/INSTRUCTION-STARTER-PACK/)

| File | Purpose | Include in Bootstrap? |
|------|---------|----------------------|
| `AGENTS.md` | Template project instructions | ✅ As template |
| `AGENTS.local.md` | Template local overrides | ✅ As template |
| `CLAUDE.md` | Symlink template | ✅ Copy |
| `CLAUDE.local.md` | Symlink template | ✅ Copy |
| `AGENTS.template.automatic-session-review.md` | Session review pattern | ✅ Optional |
| `AGENTS.template.learning-example.md` | Learning capture pattern | ✅ Optional |
| `CONTEXT-PICKUP-GUIDE.md` | How to recover context | ✅ Optional |
| `GIT-LOCAL-EXCLUDE.template` | gitignore patterns | ✅ As template |
| `SETUP-COMPAT-INSTRUCTIONS.sh` | Setup script | ✅ Optional |
| `YOLO-TROUBLESHOOT-BONUS.sh` | Troubleshooting | ❌ Skip |
| `.cursor/rules/*.mdc` | Cursor rules | ✅ Copy |
| `.github/copilot-instructions.md` | Copilot compat | ✅ Optional |
| `.github/instructions/*.md` | Domain rules | ✅ Optional |

---

## Bootstrap Process

When running `agent-os init`:

1. **Create directory structure**
   ```
   identity/ memory/ session/ checkpoint/ .agent/ .cursor/rules/ .github/
   ```

2. **Copy core files**
   - AGENTS.md → AGENTS.md
   - AGENTS.local.md → AGENTS.local.md (gitignored)
   - Create symlinks: CLAUDE.md → AGENTS.md, CLAUDE.local.md → AGENTS.local.md

3. **Generate project-specific files**
   - derivation.yaml (from prompts)
   - vocabulary.yaml (from template)
   - identity/identity.md (from template)

4. **Initialize checkpoint**
   - checkpoint/current.yaml (empty state)

5. **Optional: Initialize beads**
   - `bd init --prefix <slug>`

---

## Semantic Delimiter Quick Reference

| Delimiter | Resolves To |
|-----------|-------------|
| `{{PROJECT_NAME}}` | "My Project" |
| `{{PROJECT_SLUG}}` | "my-project" |
| `{{PROJECT_TYPE}}` | software |
| `{{MEMORY:identity}}` | identity/identity.md |
| `{{MEMORY:long}}` | memory/ |
| `{{MEMORY:short}}` | session/ |
| `{{MEMORY:checkpoint}}` | checkpoint/ |
| `{{MEMORY:config}}` | .agent/ |
| `{{WORKFLOW_PREFIX}}` | "memory" (or custom) |

---

## Vocabulary Quick Reference

| Semantic ID | Purpose | Default Command |
|-------------|---------|-----------------|
| LR-L2-init | Create structure | `memory init` |
| LR-L2-capture | Save insight | `memory capture` |
| LR-L3-search | Find knowledge | `memory search` |
| LR-L3-validate | Check correctness | `memory check` |
| UL-L2-recover | Resume from checkpoint | `read:checkpoint/current.yaml` |
| UL-L4-prioritize | Choose next work | `bd ready` |
| LL-L3-handoff | Transfer context | Template: handoff.md |
| LL-L2-onboard | Get started | `read:identity/identity.md` |

---

## Files to Exclude from Bootstrap

- `VERSION` - Repo-specific
- `RELEASING.md` - Repo-specific
- `CONTRIBUTING.md` - Repo-specific
- `CLA/` - Repo-specific
- `PROVENANCE/` - Repo-specific
- `LICENSE` - Repo-specific
- `LAB-*/` - Examples only
- `99-*/` - Advanced/reference only
- `optional/` - Deep integration only

---

## Summary: Minimal Bootstrap

```
agent-os init my-project

Creates:
├── AGENTS.md                    # From template
├── AGENTS.local.md              # From template (gitignored)
├── CLAUDE.md → AGENTS.md        # Symlink
├── CLAUDE.local.md → AGENTS.local.md  # Symlink
├── identity/identity.md         # From template
├── memory/                      # Empty
├── session/logs/                # Empty
├── checkpoint/current.yaml      # Empty state
├── .agent/
│   ├── vocabulary.yaml          # From template
│   └── derivation.yaml          # Generated
├── .cursor/rules/               # From starter pack
└── .github/                     # Optional, from starter pack
```
