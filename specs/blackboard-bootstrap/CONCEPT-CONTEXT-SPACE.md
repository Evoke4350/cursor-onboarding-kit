# Concept: Context Space Optimization

**The theoretical foundation for the Agentic Blackboard**

---

## The Problem with Parameter Space

### Why Fine-Tuning Fails

| Issue | What Happens |
|-------|--------------|
| **Cost** | GPU hours, compute budget, iteration cycles |
| **Irreversibility** | Once weights change, you can't go back |
| **Catastrophic forgetting** | New knowledge overwrites old |
| **Overfitting** | Model memorizes training data, loses generality |
| **Brittleness** | Small changes cascade unpredictably |

### Why GRPO Isn't Enough

Group Relative Policy Optimization (GRPO) tries to improve agents through RL:

1. Supervised Fine-Tuning (SFT) phase
2. Reinforcement Learning (RL) phase
3. Gradient-based weight updates

**The problem**: It still operates in parameter space. Even "training-free" variants still require some optimization.

From arXiv:2510.08191:
> "Methods like agentic reinforcement learning typically rely on costly parameter updates through SFT followed by RL with GRPO to alter the output distribution."

---

## The Alternative: Context Space

### Training-Free GRPO

The paper proposes a different approach:

> "LLMs can achieve a similar effect on the output distribution by learning experiential knowledge as a token prior, which is a far more lightweight approach."

**Key mechanism:**
1. Model introspects its own attempts
2. Extracts "semantic advantage" — what worked vs what didn't
3. Distills into natural language rules
4. Stores in experience library
5. Injects as token prior in future prompts

**No weight updates. No gradients. Just context.**

### Why Context Space Wins

| Dimension | Parameter Space | Context Space |
|-----------|----------------|---------------|
| **Cost** | Expensive (GPU) | Cheap (tokens) |
| **Reversibility** | No | Yes (git) |
| **Generality** | Destroys base | Preserves base |
| **Iteration speed** | Slow (training) | Fast (prompting) |
| **Debuggability** | Opaque weights | Readable prose |
| **Transfer** | Domain-specific | Universal |

---

## How Blackboard Implements This

### The Mapping

```
┌─────────────────────────────────────────────────────────────────┐
│                 TRAINING-FREE GRPO                              │
│                                                                 │
│   Introspect ──→ Extract ──→ Distill ──→ Store ──→ Inject      │
│       │            │           │          │          │         │
│       ▼            ▼           ▼          ▼          ▼         │
│   ┌───────┐   ┌───────┐   ┌───────┐  ┌───────┐  ┌───────┐     │
│   │Model  │   │Rules  │   │Natural│  │Exper- │  │Token  │     │
│   │tries  │   │about  │   │Lang   │  │ience  │  │Prior  │     │
│   │stuff  │   │worked │   │Rules  │  │Library│  │in API │     │
│   └───────┘   └───────┘   └───────┘  └───────┘  └───────┘     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ MAPS TO
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   AGENTIC BLACKBOARD                            │
│                                                                 │
│   /cut ──────→ /carve ────→ /chamfer ──→ shavings/ ──→ context │
│       │            │           │           │            │       │
│       ▼            ▼           ▼           ▼            ▼       │
│   ┌───────┐   ┌───────┐   ┌───────┐   ┌───────┐   ┌───────┐   │
│   │Extract│   │Find   │   │Update │   │Atomic │   │Inject │   │
│   │insight│   │connect│   │older  │   │notes  │   │in LLM │   │
│   │       │   │ions   │   │notes  │   │       │   │prompt │   │
│   └───────┘   └───────┘   └───────┘   └───────┘   └───────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Concrete Implementation

| GRPO Concept | Blackboard Tool | What It Does |
|--------------|-----------------|--------------|
| Introspect | `/cut` | Extract insight from attempt |
| Extract rules | `/carve` | Find connections, patterns |
| Distill | `/chamfer` | Refine older notes |
| Store | `shavings/` | Markdown with wiki links |
| Inject | Context files | CLAUDE.md, AGENTS.md, PROMPT.md |

### The Ralph Loop as Multi-Epoch Learning

From the paper:
> "Iteratively distilling high-quality experiential knowledge during multi-epoch learning on a minimal ground-truth data."

The Ralph loop is exactly this:

```bash
while :; do cat PROMPT.md | claude-code ; done
```

Each iteration:
1. Fresh context (no pollution from previous attempts)
2. Persistent state in files (experiential knowledge survives)
3. Natural language rules in progress.txt
4. Token prior injection via PROMPT.md

---

## Practical Implications

### For Cursor Hooks

The hooks we're building are **token prior injection points**:
- `before_file_read` → Check against experience (protected paths)
- `before_shell_execution` → Apply learned rules (exfil blocks)
- `before_mcp_execution` → Consult experience library

### For Shavings

Each shaving is a **natural language rule** distilled from experience:
```markdown
---
title: the evidence suggests that morning exercise reduces anxiety
description: mechanism through cortisol regulation
source: [[2024-03-health-tracking]]
topics: [health, productivity]
---

[The actual insight in prose...]
```

When injected as context, this becomes a **token prior** that guides model behavior.

### For the Workshop

The entire `workshop/` directory is the **experience library**:
- `bench/` → High-level patterns (MOCs, identity)
- `shavings/` → Atomic rules (individual insights)
- `sawdust/` → Working memory (sessions, queues)

---

## The Bottom Line

**Parameter space is unmanageable.**
- Even GRPO fails
- Expensive, irreversible, destroys generality

**Context space is the answer.**
- Cheap, reversible, preserves base model
- Model learns from itself
- Natural language rules, not weights

**The Agentic Blackboard IS this experience library.**

We're not fine-tuning. We're not doing RL. We're managing context space.

That's what we're fucking doing.

---

## References

- arXiv:2510.08191 — Training-Free Group Relative Policy Optimization
- arscontexta — 249 research claims on agent cognition
- Ralph Loop — Fresh context per iteration, persistent state in files
