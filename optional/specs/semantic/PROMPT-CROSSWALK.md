# Prompt Crosswalk: Semantic Equivalents Across Systems

**Purpose:** Map same semantic operations across different project vocabularies
**Use:** Translate prompts between workshop, agent-os, arscontexta, and custom systems

---

## Core Operations Crosswalk

| Semantic ID | Workshop | Agent OS | arscontexta | Generic |
|-------------|----------|----------|-------------|---------|
| **LR-L2-init** | `init` | `setup` | `derive` | `bootstrap` |
| **LR-L2-capture** | `cut` | `discover-standards` | `reduce` | `capture` |
| **LR-L3-search** | `carve` | `inject-standards` | `reflect` | `search` |
| **LR-L2-update** | `chamfer` | (edit directly) | `reweave` | `update` |
| **LR-L3-validate** | `check` | `index-standards` | `verify` | `validate` |
| **LR-L4-discover** | (manual) | `discover-standards` | `reduce` | `discover` |
| **LR-L4-plan** | (manual) | `shape-spec` | `seed` | `plan` |

---

## Quadrant Coverage by System

### Workshop CLI

| Quadrant | Commands | Coverage |
|----------|----------|----------|
| UL | (none - manual) | ❌ Missing |
| UR | cut, carve, chamfer | ⚠️ Partial |
| LL | (none - manual) | ❌ Missing |
| LR | init, check, taint, status | ✅ Good |

**Gap Analysis:** Workshop lacks UL (reflection) and LL (communication) commands.

### Agent OS

| Quadrant | Commands | Coverage |
|----------|----------|----------|
| UL | (none) | ❌ Missing |
| UR | (via Claude) | ⚠️ Implicit |
| LL | (none - manual) | ❌ Missing |
| LR | discover, inject, index, shape, plan | ✅ Excellent |

**Gap Analysis:** Agent OS is LR-heavy, assumes UR/LL happen elsewhere.

### arscontexta

| Quadrant | Commands | Coverage |
|----------|----------|----------|
| UL | reflect, remember, rethink | ✅ Excellent |
| UR | reduce, reweave | ✅ Good |
| LL | (via handoff protocol) | ⚠️ Partial |
| LR | seed, verify, pipeline | ✅ Excellent |

**Gap Analysis:** Most complete coverage, but LL is implicit.

---

## Level Coverage by System

| Level | Workshop | Agent OS | arscontexta |
|-------|----------|----------|-------------|
| L1 (Execution) | ✅ | ❌ | ❌ |
| L2 (Task) | ✅ | ✅ | ✅ |
| L3 (Feature) | ⚠️ | ✅ | ✅ |
| L4 (Epic) | ⚠️ | ✅ | ✅ |
| L5 (Product) | ❌ | ✅ | ❌ |
| L6 (System) | ❌ | ❌ | ❌ |
| L7 (Meta) | ❌ | ❌ | ✅ (rethink) |

---

## Complete Semantic Command Set

To have full AQAL coverage, a system needs:

### UL Commands (Self-Management)

| Semantic ID | Purpose | Implementation |
|-------------|---------|----------------|
| `reflect` | Learn from completed work | Analyze session, extract insights |
| `recover` | Resume from interruption | Read checkpoint, restore state |
| `prioritize` | Choose next work | Query ready queue, apply policy |
| `decide` | Make a choice with logging | Ask, record, proceed |
| `retrospect` | Improve methodology | Analyze patterns, suggest changes |

### UR Commands (Action)

| Semantic ID | Purpose | Implementation |
|-------------|---------|----------------|
| `capture` | Save insight to memory | Extract from source, write to long-term |
| `search` | Find existing knowledge | Query memory, return connections |
| `update` | Modify existing knowledge | Append context, preserve history |
| `implement` | Build code/functionality | Write code, tests |
| `fix` | Repair broken code | Debug, patch, verify |

### LL Commands (Communication)

| Semantic ID | Purpose | Implementation |
|-------------|---------|----------------|
| `explain` | Clarify for others | Generate documentation, comments |
| `handoff` | Transfer context | Write checkpoint, session log |
| `onboard` | Get started | Read identity, show next steps |
| `align` | Ensure shared understanding | Verify assumptions, clarify scope |
| `review` | Check others' work | Validate, critique, approve |

### LR Commands (Structure)

| Semantic ID | Purpose | Implementation |
|-------------|---------|----------------|
| `init` | Create structure | Generate directories, templates |
| `validate` | Check correctness | Run checks, report issues |
| `configure` | Adjust settings | Modify config files |
| `secure` | Protect sensitive data | Apply taint rules, enforce policy |
| `index` | Make findable | Update search index, catalog |

---

## Vocabulary Derivation File

```yaml
# vocabulary.yaml - Per-project command mapping

# How semantic operations are expressed in this project
commands:
  # LR Quadrant (Structure)
  init: "workshop init"
  capture: "workshop cut"
  search: "workshop carve"
  update: "workshop chamfer"
  validate: "workshop check"

  # UR Quadrant (Action) - typically native to agent
  implement: "native"
  fix: "native"
  refactor: "native"

  # LL Quadrant (Communication) - templates
  handoff: "template:40-TEMPLATES/TEMPLATE-handoff.md"
  explain: "prompt:Explain this for a new team member"

  # UL Quadrant (Self) - checkpoints
  recover: "read:{{MEMORY:state}}/checkpoint.yaml"
  reflect: "prompt:What did I learn from the last session?"

# Aliases for common workflows
workflows:
  bugfix:
    - reflect
    - search
    - fix
    - validate
    - handoff

  feature:
    - plan
    - implement
    - capture
    - validate
    - handoff

  refactor:
    - search
    - refactor
    - update
    - validate
```

---

## Template: Semantic Address Resolution

```bash
# resolve-prompt.sh - Translate semantic ID to actual command

SEMANTIC_ID=$1  # e.g., "LR-L2-capture"

# Read vocabulary mapping
source vocabulary.yaml

case $SEMANTIC_ID in
  LR-L2-init)      echo "$commands_init" ;;
  LR-L2-capture)   echo "$commands_capture" ;;
  LR-L3-search)    echo "$commands_search" ;;
  LR-L2-update)    echo "$commands_update" ;;
  LR-L3-validate)  echo "$commands_validate" ;;
  LL-L3-handoff)   echo "cat 40-TEMPLATES/TEMPLATE-handoff.md" ;;
  UL-L2-recover)   echo "cat {{MEMORY:state}}/checkpoint.yaml" ;;
  *)               echo "Unknown semantic ID: $SEMANTIC_ID" ;;
esac
```

---

## Usage Example

```bash
# Instead of remembering "workshop cut", use semantic address
./resolve-prompt.sh LR-L2-capture
# Output: workshop cut

# Or in a generic project
# Output: note add
# Output: capture
# etc.
```

This enables writing prompts that work across any system:

```markdown
# Generic prompt template
{{LR-L2-capture}} the insight from {{file}}
{{LR-L3-search}} for related work
{{LR-L2-update}} any connected items
{{LR-L3-validate}} the structure
```

Resolves to workshop:
```markdown
workshop cut the insight from {{file}}
workshop carve for related work
workshop chamfer any connected items
workshop check the structure
```

Or to standard naming:
```markdown
note add the insight from {{file}}
note find for related work
note edit any connected items
note verify the structure
```
