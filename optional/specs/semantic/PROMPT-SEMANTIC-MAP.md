# Semantic Prompt Map (AQAL-Inspired)

**Purpose:** Map all prompts to semantic dimensions for universal applicability
**Based on:** Ken Wilber's AQAL (All Quadrants, All Levels, All Lines, All States, All Types)

---

## The AQAL Framework (Adapted for Prompts)

Wilber's insight: Reality can be mapped across multiple dimensions. Any phenomenon exists in:
- **Quadrants** (perspectives: I, We, It, Its)
- **Levels** (depth/complexity)
- **Lines** (types of intelligence/capability)
- **States** (temporary conditions)
- **Types** (stylistic variations)

We apply this to **prompts** to create a universal semantic map.

---

## Quadrants: Four Prompt Perspectives

| Quadrant | Perspective | Agent Dimension | Example Prompts |
|----------|-------------|-----------------|-----------------|
| **UL** (Interior-Individual) | "I" | Self-awareness, intention | Reflect, remember, prioritize |
| **UR** (Exterior-Individual) | "It" | Actions, outputs | Cut, carve, implement, fix |
| **LL** (Interior-Collective) | "We" | Shared understanding | Align, handoff, explain, onboard |
| **LR** (Exterior-Collective) | "Its" | Systems, structure | Init, configure, validate, secure |

### Quadrant Mapping

```yaml
# How prompts map to quadrants

UL: # Interior-Individual (subjective)
  - reflect: "What did I learn?"
  - remember: "What do I know?"
  - prioritize: "What matters most?"
  - decide: "What should I choose?"
  - recover: "Where was I?"

UR: # Exterior-Individual (objective action)
  - cut: "Extract this insight"
  - carve: "Find connections"
  - chamfer: "Update existing"
  - implement: "Build this"
  - fix: "Repair this"

LL: # Interior-Collective (shared meaning)
  - align: "Are we on same page?"
  - handoff: "Continue from here"
  - explain: "Help me understand"
  - onboard: "Get started"
  - review: "Check my work"

LR: # Exterior-Collective (system structure)
  - init: "Set up structure"
  - configure: "Adjust settings"
  - validate: "Check correctness"
  - secure: "Protect this"
  - index: "Make findable"
```

---

## Levels: Depth of Operation

Prompts operate at different levels of abstraction:

| Level | Name | Scope | Example |
|-------|------|-------|---------|
| **L1** | Execution | Single action | "Fix this typo" |
| **L2** | Task | Multiple actions | "Implement auth" |
| **L3** | Feature | Multiple tasks | "Add login system" |
| **L4** | Epic | Multiple features | "Build user management" |
| **L5** | Product | Multiple epics | "Create SaaS platform" |
| **L6** | System | Multiple products | "Design architecture" |
| **L7** | Meta | System of systems | "Improve how we work" |

### Level Indicators in Prompts

```
L1: "this file", "this line", "this function"
L2: "this feature", "this component", "this module"
L3: "the system", "the application", "the service"
L4: "the product", "the platform", "the suite"
L5: "the organization", "the ecosystem"
L6: "the architecture", "the paradigm"
L7: "the methodology", "the approach", "how we..."
```

---

## Lines: Types of Intelligence

Different prompts invoke different capabilities:

| Line | Capability | Prompts |
|------|------------|---------|
| **Cognitive** | Analysis, logic | Analyze, compare, evaluate |
| **Technical** | Code, systems | Implement, refactor, debug |
| **Creative** | Generation | Design, draft, propose |
| **Interpersonal** | Communication | Explain, handoff, align |
| **Self-awareness** | Reflection | Reflect, recover, learn |
| **Strategic** | Planning | Plan, scope, prioritize |
| **Ethical** | Values | Review, validate, secure |

### Line Detection

```yaml
# Semantic markers for each line

cognitive:
  markers: ["analyze", "compare", "evaluate", "assess", "determine"]
  output: "reasoning, comparison, judgment"

technical:
  markers: ["implement", "build", "fix", "refactor", "debug", "optimize"]
  output: "code, configuration, systems"

creative:
  markers: ["design", "create", "propose", "generate", "draft"]
  output: "new artifacts, options, possibilities"

interpersonal:
  markers: ["explain", "document", "handoff", "align", "communicate"]
  output: "shared understanding, continuity"

self_awareness:
  markers: ["reflect", "remember", "learn", "recover", "check"]
  output: "insight, continuity, improvement"

strategic:
  markers: ["plan", "scope", "prioritize", "roadmap", "sequence"]
  output: "direction, ordering, boundaries"

ethical:
  markers: ["validate", "secure", "review", "protect", "ensure"]
  output: "safety, correctness, alignment"
```

---

## States: Temporary Conditions

Prompts assume or create different states:

| State | Condition | Prompts That Need It |
|-------|-----------|---------------------|
| **Fresh** | New session, clean context | Init, onboard |
| **Active** | Mid-work, context loaded | Implement, fix, cut |
| **Blocked** | Waiting for something | Escalate, defer |
| **Complete** | Work done | Validate, deliver |
| **Interrupted** | Work stopped mid-way | Recover, resume |
| **Degraded** | Context pollution | Reset, summarize |

### State Transitions

```
Fresh → Active (on task start)
Active → Blocked (on blocker)
Active → Complete (on finish)
Active → Interrupted (on crash/stop)
Interrupted → Active (on recover)
Degraded → Fresh (on context reset)
```

---

## Types: Stylistic Variations

Same semantic prompt, different expression:

| Type | Style | Example |
|------|-------|---------|
| **Direct** | Imperative | "Fix this bug" |
| **Exploratory** | Open-ended | "What's causing this?" |
| **Guided** | Step-by-step | "First analyze, then fix" |
| **Constrained** | Limited scope | "Fix only this file" |
| **Collaborative** | Discussion | "Let's figure this out" |

### Type Selection

```yaml
# When to use each type

direct:
  when: "Clear task, known solution"
  efficiency: high
  creativity: low

exploratory:
  when: "Unclear problem, need discovery"
  efficiency: medium
  creativity: medium

guided:
  when: "Complex task, risk of drift"
  efficiency: medium
  creativity: low

constrained:
  when: "Risk of scope creep"
  efficiency: high
  creativity: low

collaborative:
  when: "Shared understanding needed"
  efficiency: low
  creativity: high
```

---

## Semantic Prompt Address

Every prompt has a semantic address:

```
[QUADRANT]-[LEVEL]-[LINE]-[STATE]-[TYPE]

Examples:
- UR-L2-technical-active-direct: "Fix the auth bug in login.rs"
- LL-L3-interpersonal-fresh-guided: "Help me understand the codebase"
- UL-L7-self_awareness-complete-exploratory: "What did we learn from this sprint?"
- LR-L4-strategic-fresh-constrained: "Plan the next quarter, focus on revenue"
```

---

## Master Prompt Map

### Memory/Knowledge Prompts (LR Quadrant)

| Semantic ID | Prompt | Function |
|-------------|--------|----------|
| LR-L2-cognitive-fresh-direct | `init` | Create structure |
| LR-L2-cognitive-active-direct | `capture` | Save insight |
| LR-L3-cognitive-active-direct | `search` | Find connections |
| LR-L2-technical-active-direct | `update` | Modify existing |
| LR-L3-ethical-active-direct | `validate` | Check correctness |
| LR-L4-cognitive-active-exploratory | `discover` | Extract patterns |

### Workflow Prompts (UR Quadrant)

| Semantic ID | Prompt | Function |
|-------------|--------|----------|
| UR-L2-technical-active-direct | `implement` | Build code |
| UR-L2-technical-active-direct | `fix` | Repair code |
| UR-L3-technical-active-guided | `refactor` | Improve structure |
| UR-L2-cognitive-active-exploratory | `debug` | Find root cause |
| UR-L4-creative-fresh-exploratory | `design` | Plan architecture |

### Communication Prompts (LL Quadrant)

| Semantic ID | Prompt | Function |
|-------------|--------|----------|
| LL-L2-interpersonal-active-direct | `explain` | Clarify for others |
| LL-L3-interpersonal-active-direct | `handoff` | Transfer context |
| LL-L2-interpersonal-fresh-guided | `onboard` | Get started |
| LL-L4-interpersonal-active-collaborative | `align` | Shared understanding |
| LL-L3-ethical-active-direct | `review` | Check others' work |

### Self-Management Prompts (UL Quadrant)

| Semantic ID | Prompt | Function |
|-------------|--------|----------|
| UL-L3-self_awareness-active-exploratory | `reflect` | Learn from work |
| UL-L2-self_awareness-interrupted-direct | `recover` | Resume from checkpoint |
| UL-L4-strategic-active-direct | `prioritize` | Choose next work |
| UL-L5-self_awareness-complete-exploratory | `retrospect` | Improve process |
| UL-L2-cognitive-active-direct | `decide` | Make a choice |

---

## Application: Derivation for Any Project

```yaml
# semantic-map.yaml - Generated per project

project:
  name: "{{PROJECT_NAME}}"
  vocabulary:
    capture: "{{WORKFLOW:capture}}"    # workshop: "cut", standard: "note"
    search: "{{WORKFLOW:search}}"      # workshop: "carve", standard: "find"
    update: "{{WORKFLOW:update}}"      # workshop: "chamfer", standard: "edit"
    validate: "{{WORKFLOW:validate}}"  # workshop: "check", standard: "verify"

quadrants:
  UL: [reflect, recover, prioritize, decide]
  UR: [capture, search, update, implement, fix]
  LL: [explain, handoff, onboard, align, review]
  LR: [init, validate, configure, secure, index]

# For store-ui project with minimal naming:
# capture → "Add to .memory/"
# search → "Search .memory/"
# etc.
```

---

## Benefits of Semantic Mapping

1. **Portability** - Same concept, different vocabulary
2. **Discoverability** - Find prompts by what they DO, not what they're called
3. **Composability** - Combine prompts by combining semantic addresses
4. **Translation** - Map workshop → agent-os → arscontexta → custom
5. **Validation** - Check if all quadrants/levels are covered

---

## Integration with Bootstrap

The bootstrap script creates `semantic-map.yaml` alongside `derivation.yaml`:

```bash
./bootstrap.sh
# Creates:
#   derivation.yaml    - Path mappings ({{MEMORY:*}})
#   semantic-map.yaml  - Prompt mappings (quadrants, levels, lines)
```
