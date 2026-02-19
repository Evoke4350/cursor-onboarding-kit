# AI Agents Need State

> Stateless prompts are useful, but durable systems need memory, checkpoints, and explicit state transitions.

Most agent demos collapse because they confuse **prompt quality** with **system design**.

Prompts matter. But in production, what matters more is whether the agent can:

1. recover from partial failure,
2. replay decisions safely, and
3. explain why it did what it did.

That requires state.

---

## Treat Actions as Transitions

An agent loop is easier to reason about if every tool call is a transition:

```
(state_n, input) -> decision -> tool_result -> state_n+1
```

Once you model this explicitly, retries become deterministic and observability becomes straightforward.

---

## Keep Memory Layered

Use short-term memory for active tasks, long-term memory for durable user context, and immutable logs for reconstruction.

When these are blended into one opaque prompt blob, debugging becomes guesswork.

---

## Build for Interruption

Real systems are interrupted by deployments, rate limits, and network failures.

If your agent cannot resume from checkpoints, it is not an autonomous system yet — it is a best-effort script.

---

## How Workshop Implements This

| State Layer | Location | Purpose |
|-------------|----------|---------|
| **Checkpoint** | `sawdust/state/` | Current task, position, recoverable state |
| **Short-term** | `sawdust/sessions/` | Session logs, ephemeral context |
| **Long-term** | `shavings/` | Durable knowledge, persists across sessions |
| **Identity** | `bench/` | Who you are, how you work (rarely changes) |
| **Immutable log** | `git history` | Every decision traceable, replayable |

### State Transitions in the 5 Cs

```
(state, file) -> cut -> shaving created -> state+shaving_ref
(state, query) -> carve -> results found -> state+connections
(state, shaving, context) -> chamfer -> updated -> state+version
(state) -> check -> validation -> state+health
```

### Checkpoint Format

```yaml
# sawdust/state/current-task.yaml
task_id: bd-42
phase: implement
position: src/auth.rs:147
started: 2026-02-18T16:00:00Z
context:
  - previous_file: src/auth.rs
  - test_status: failing
  - blocker: null
```

### Recovery

```bash
# After interruption, agent reads checkpoint
workshop recover

# Outputs:
# "Continuing from src/auth.rs:147 (implement phase)
#  Task: bd-42 | Started: 2 hours ago
#  Last action: Wrote auth function, tests failing"
```

---

## The State Machine Mindset

Every workshop command is a state transition:

| Command | From State | To State |
|---------|------------|----------|
| `workshop cut` | idle | capturing |
| `workshop carve` | idle | searching |
| `workshop chamfer` | idle | updating |
| `workshop check` | any | validating |
| `workshop recover` | interrupted | previous_state |

The state machine makes behavior **predictable**, **observable**, and **resumable**.
