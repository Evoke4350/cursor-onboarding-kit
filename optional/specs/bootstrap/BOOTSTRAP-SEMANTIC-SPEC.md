# Bootstrap Semantic Delimiter Specification

**Purpose:** Define a universal pattern for bootstrapping agent-aware projects
**Status:** Draft

---

## The Problem

Every project that wants agent memory, state management, and workflow needs similar structure. But the naming varies:

| Project | Memory | State | Identity |
|---------|--------|-------|----------|
| workshop | shavings/ | sawdust/ | bench/identity.md |
| agent-os | standards/ | .agent/ | product/mission.md |
| arscontexta | notes/ | ops/ | derivation.md |
| generic | ??? | ??? | ??? |

We need **semantic delimiters** that bootstrap scripts can recognize and project-specific names can fill in.

---

## Semantic Delimiter Pattern

### Core Delimiters (Required)

| Delimiter | Meaning | Example Resolution |
|-----------|---------|-------------------|
| `{{PROJECT_NAME}}` | Human-readable project name | "Store UI" |
| `{{PROJECT_SLUG}}` | URL-safe identifier | "store-ui" |
| `{{PROJECT_TYPE}}` | Category | software, research, writing |
| `{{AGENT_FILE}}` | Where agent reads instructions | AGENTS.md, CLAUDE.md |

### Memory Delimiters (Semantic)

| Delimiter | Meaning | Filled By |
|-----------|---------|-----------|
| `{{MEMORY:identity}}` | Who/what this project is | User answers |
| `{{MEMORY:long}}` | Durable knowledge storage | `shavings/`, `notes/`, `insights/` |
| `{{MEMORY:short}}` | Ephemeral session data | `sawdust/`, `.session/`, `scratch/` |
| `{{MEMORY:state}}` | Checkpoints for recovery | `.state/`, `state/`, `sawdust/state/` |
| `{{MEMORY:config}}` | Project configuration | `.workshop/`, `.agent/`, `config/` |

### Workflow Delimiters

| Delimiter | Meaning |
|-----------|---------|
| `{{WORKFLOW:capture}}` | Command to capture insight |
| `{{WORKFLOW:search}}` | Command to find connections |
| `{{WORKFLOW:update}}` | Command to update existing |
| `{{WORKFLOW:validate}}` | Command to validate structure |

---

## Delimiter Resolution

A `derivation.yaml` file maps semantics to project-specific names:

```yaml
# derivation.yaml - created by bootstrap
project:
  name: "Store UI"
  slug: "store-ui"
  type: software

memory:
  identity: "bench/identity.md"      # {{MEMORY:identity}}
  long: "shavings/"                   # {{MEMORY:long}}
  short: "sawdust/"                   # {{MEMORY:short}}
  state: "sawdust/state/"             # {{MEMORY:state}}
  config: ".workshop/"                # {{MEMORY:config}}

workflow:
  capture: "workshop cut"             # {{WORKFLOW:capture}}
  search: "workshop carve"            # {{WORKFLOW:search}}
  update: "workshop chamfer"          # {{WORKFLOW:update}}
  validate: "workshop check"          # {{WORKFLOW:validate}}
```

---

## Generic Checkpoint Format

```yaml
# {{MEMORY:state}}/checkpoint.yaml
# Generic format - no project-specific terminology

task:
  id: null                           # Task identifier (any format)
  title: ""                          # Human readable
  type: bugfix|feature|refactor|docs|research
  priority: P0|P1|P2

phase: idle|triage|scope|implement|verify|deliver
phase_entered: null

position:
  file: null
  line: null
  context: []

decisions:
  - at: null
    question: ""
    choice: ""
    alternatives: []

session:
  started: null
  last_activity: null
  log: "{{MEMORY:short}}/sessions/{session_id}.md"

# State machine
transitions:
  - from: null
    to: null
    at: null
    trigger: ""
```

---

## Bootstrap Flow (Gum Script)

```bash
#!/usr/bin/env bash
# bootstrap.sh - Universal agent project bootstrap

set -e

# Detect if derivation.yaml exists
if [ -f "derivation.yaml" ]; then
  echo "Project already bootstrapped."
  echo "Edit derivation.yaml to reconfigure."
  exit 0
fi

# Phase 1: Gather minimal info
PROJECT_NAME=$(gum input --placeholder "Project name (e.g., Store UI)")
PROJECT_SLUG=$(echo "$PROJECT_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -cd '[:alnum:]-')
PROJECT_TYPE=$(gum choose --header "What type of project?" "software" "research" "writing")

# Phase 2: Choose naming scheme
NAMING_SCHEME=$(gum choose --header "Memory naming style?" \
  "workshop (bench/shavings/sawdust)" \
  "standard (identity/notes/sessions)" \
  "minimal (.identity/.memory/.state)")

case $NAMING_SCHEME in
  workshop*)
    IDENTITY_DIR="bench"
    MEMORY_DIR="shavings"
    SHORT_DIR="sawdust"
    STATE_DIR="sawdust/state"
    CONFIG_DIR=".workshop"
    ;;
  standard*)
    IDENTITY_DIR="."
    MEMORY_DIR="notes"
    SHORT_DIR="sessions"
    STATE_DIR=".state"
    CONFIG_DIR="config"
    ;;
  minimal*)
    IDENTITY_DIR="."
    MEMORY_DIR=".memory"
    SHORT_DIR=".scratch"
    STATE_DIR=".state"
    CONFIG_DIR=".agent"
    ;;
esac

# Phase 3: Generate derivation.yaml
cat > derivation.yaml << EOF
project:
  name: "$PROJECT_NAME"
  slug: "$PROJECT_SLUG"
  type: $PROJECT_TYPE

memory:
  identity: "$IDENTITY_DIR/identity.md"
  long: "$MEMORY_DIR/"
  short: "$SHORT_DIR/"
  state: "$STATE_DIR/"
  config: "$CONFIG_DIR/"

workflow:
  capture: "workshop cut"
  search: "workshop carve"
  update: "workshop chamfer"
  validate: "workshop check"
EOF

# Phase 4: Create structure
mkdir -p "$MEMORY_DIR" "$SHORT_DIR/sessions" "$STATE_DIR" "$CONFIG_DIR"

# Phase 5: Generate identity from template
if [ ! -f "$IDENTITY_DIR/identity.md" ]; then
  mkdir -p "$IDENTITY_DIR"
  cat > "$IDENTITY_DIR/identity.md" << EOF
# {{PROJECT_NAME}}

**Type:** {{PROJECT_TYPE}}
**Created:** $(date +%Y-%m-%d)

## Purpose

[What this project does]

## Context

[Background a new agent needs]

## Key Files

[Important entry points]
EOF
fi

# Phase 6: Create checkpoint template
cat > "$STATE_DIR/checkpoint.yaml" << 'EOF'
task:
  id: null
  title: ""
  type: null
  priority: null

phase: idle
phase_entered: null

position:
  file: null
  line: null
  context: []

decisions: []
session:
  started: null
  last_activity: null

transitions: []
EOF

# Phase 7: Create AGENTS.md symlink if doesn't exist
if [ ! -f "AGENTS.md" ] && [ -f "../cursor-onboarding-kit/AGENTS.md" ]; then
  gum confirm "Link AGENTS.md from cursor-onboarding-kit?"
  ln -s "../cursor-onboarding-kit/AGENTS.md" AGENTS.md
fi

echo ""
echo "✓ Bootstrapped $PROJECT_NAME"
echo ""
echo "Structure:"
echo "  $IDENTITY_DIR/identity.md  - Who/what"
echo "  $MEMORY_DIR/               - Durable memory"
echo "  $SHORT_DIR/                - Session data"
echo "  $STATE_DIR/                - Checkpoints"
echo "  $CONFIG_DIR/               - Configuration"
echo ""
echo "Next: Edit $IDENTITY_DIR/identity.md with project details"
```

---

## Template Resolution

Templates use delimiters that get resolved at copy time:

```markdown
# {{PROJECT_NAME}} - Handoff

## Checkpoint (for recovery)

- **Phase:** [current phase]
- **Position:** [where work stopped]
- **State file:** {{MEMORY:state}}/checkpoint.yaml
- **Session log:** {{MEMORY:short}}/sessions/{session}.md

## Current state

[What's done, what's blocked]
```

Bootstrap resolves:
- `{{PROJECT_NAME}}` → "Store UI"
- `{{MEMORY:state}}` → "sawdust/state" (workshop) or ".state" (minimal)
- `{{MEMORY:short}}` → "sawdust" (workshop) or ".scratch" (minimal)

---

## Integration Points

### With Workshop CLI

```bash
# workshop reads derivation.yaml for naming
workshop init . --from-derivation

# or bootstrap creates it
./bootstrap.sh && workshop init . --from-derivation
```

### With Agent Instructions

AGENTS.md uses delimiters that resolve:

```markdown
## State Recovery

Read {{MEMORY:state}}/checkpoint.yaml to resume.
Log sessions to {{MEMORY:short}}/sessions/.
```

### With Existing Projects

For projects that already have structure:

```bash
# Map existing structure to semantics
./bootstrap.sh --map-existing
# Creates derivation.yaml pointing to existing directories
```

---

## Semantic vs Syntactic

| Level | What | Example |
|-------|------|---------|
| **Semantic** | What it means | `{{MEMORY:long}}` |
| **Derivation** | Project-specific | `shavings/` or `notes/` |
| **Syntactic** | Actual path | `shavings/20260218-insight.md` |

Bootstrap operates at semantic level. Derivation maps to syntactic.

---

## Files to Create

| File | Purpose |
|------|---------|
| `derivation.yaml` | Project-specific name mapping |
| `bootstrap.sh` | Gum-based interactive setup |
| `templates/*.md` | Delimiter-ready templates |
| `{{MEMORY:state}}/checkpoint.yaml` | Generic checkpoint schema |

---

## Success Criteria

1. Running `./bootstrap.sh` in an empty directory creates full structure
2. Templates resolve correctly for any naming scheme
3. Checkpoint format is identical across projects
4. AGENTS.md can reference `{{MEMORY:*}}` and work everywhere
5. No hardcoded workshop terminology in generic templates
