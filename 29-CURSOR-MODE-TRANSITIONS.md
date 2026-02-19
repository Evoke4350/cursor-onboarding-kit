# Cursor Auto-Approved Mode Transitions

---

## Feature Location

Settings → Agents → "Auto-Approved Mode Transitions"

Effect: For any listed transition, the agent may switch from source mode to target mode without prompting for approval. Transitions not listed require explicit approval.

Persistence: Press **Enter** after editing the field; otherwise the setting may not persist.

---

## Modes

| Mode | Role | Capabilities |
|------|------|--------------|
| Agent | Implementation, refactors, multi-file edits | Autonomous exploration, edits, commands |
| Ask | Learning, exploration, questions | Read-only; no automatic changes |
| Plan | Complex features requiring a plan first | Plan creation, clarification, then execution |
| Debug | Tricky bugs, regressions | Hypotheses, instrumentation, reproduction, log analysis, targeted fix |

---

## Transitions

A transition is a directed edge: `Source → Target`. Canonical naming:

- `Ask -> Agent`
- `Plan -> Agent`
- `Debug -> Agent`
- `Agent -> Debug`
- `Ask -> Debug`

---

## Path Classification

### State Semantics

- **Reflective states:** Ask, Plan — understanding and design; no or deferred execution
- **Execution states:** Agent, Debug — making changes

### Forward vs Regression

| Path | Classification | Rationale |
|------|----------------|-----------|
| Ask → Agent | Forward | Understanding included, then implementation |
| Plan → Agent | Forward | Plan included, then implementation |
| Debug → Agent | Forward | Debugging done, return to implementation |
| Agent → Debug | Forward | Hit a bug, gather evidence before continuing |
| Ask → Debug | Forward | Exploration reveals bug, move to debugging |
| Agent → Ask | Regression | Leaves implementation for read-only |
| Agent → Plan | Regression | Leaves implementation for planning |
| Debug → Ask | Regression | Leaves debugging for read-only |
| Debug → Plan | Regression | Leaves debugging for planning |

---

## Recommended Policy

### Auto-Approve

- Ask → Agent
- Plan → Agent
- Debug → Agent
- Agent → Debug
- Ask → Debug (optional; stricter without it)

### Do Not Auto-Approve

- Agent → Ask
- Agent → Plan
- Debug → Ask
- Debug → Plan

Rationale: Auto-approving into execution and Debug ↔ Agent maintains flow. Not auto-approving out of execution into Ask or Plan prevents involuntary regression.

---

## Application

1. Open Settings → Agents → Auto-Approved Mode Transitions
2. Set entries to the auto-approve list above
3. Use canonical transition names
4. Press **Enter** to persist
5. Do not add transitions from {Agent, Debug} to {Ask, Plan}

---

## Workflow Effects

### Interruption Rate

Each "approve this transition?" prompt is a context switch. Auto-approve reduces these interruptions. When the user says "implement X," the mode can change without a separate approval step—the intent was already expressed.

### Flow

Fewer context switches. Mode changes become a continuation of the conversation instead of a separate decision point. Modes feel like tools in one workflow rather than separate guarded steps.

### Control

Auto-approving only forward transitions means:
- When the user asks for action, it proceeds
- When in Agent/Debug, the user is not pulled back to Ask/Plan without consent
- Regression requires explicit approval

### Unattended Runs

Without auto-approve, long runs stall on approval prompts. With the right transitions auto-approved, sequences like Plan → Agent → Debug → Agent can complete without user presence.

### Discoverability

The setting is easy to miss and doesn't persist without pressing Enter. Many users never configure it, or try it and think it doesn't work because they didn't save.

---

## Anti-Patterns

- **Agent-only:** No Ask or Plan; action without understanding or design
- **Ask-only / Plan-only:** Never leaving reflection; no execution
- **Unapproved regression:** Auto-approving Agent → Ask or Agent → Plan
