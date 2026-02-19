# AGENTS.md Effectiveness: What the Research Says

**Reference:** [Evaluating AGENTS.md: Are Repository-Level Context Files Helpful for Coding Agents?](https://arxiv.org/pdf/2602.11988) (Gloaguen et al., ETH Zurich / LogicStar.ai, February 2026)

---

## The Surprising Finding

Context files like `AGENTS.md` and `CLAUDE.md` are widely recommended. Over 60,000 repositories include them. But rigorous evaluation reveals:

| Context Type | Success Rate Change | Cost Change |
|--------------|---------------------|-------------|
| **None** | Baseline | Baseline |
| **LLM-generated** | **-3%** | **+20%** |
| **Developer-written** | +4% | +19% |

**LLM-generated context files make agents worse and more expensive.**

---

## Why Context Files Underperform

### 1. Redundant Documentation

When researchers removed all existing documentation (READMEs, docs folders), LLM-generated context files suddenly became useful (+2.7% improvement). This suggests:

> Context files are mostly redundant with what's already in the repository.

### 2. No Effective Overview

One recommended use of context files is providing a codebase overview. But agents with context files don't find relevant files faster—they often take more steps because they:

1. Issue multiple commands to find the context file
2. Read it multiple times despite it being in context
3. Explore more broadly without better targeting

### 3. Unnecessary Requirements Make Tasks Harder

Context files add instructions. Agents follow them. But additional requirements—even well-intentioned ones—increase cognitive load and reasoning tokens (14-22% more reasoning with context files).

**More instructions ≠ better outcomes.**

---

## What Context Files Do Well

### Agents Follow Instructions

If a tool is mentioned in the context file, agents use it:
- `uv`: 1.6 uses/instance when mentioned vs. <0.01 when not
- Repository-specific tools: 2.5 uses/instance when mentioned vs. <0.05 when not

This isn't an instruction-following problem. Agents comply—they're just not being helped by what they're told.

### More Exploration, More Testing

Context files increase:
- Test execution frequency
- File traversal (grep, read, glob)
- Repository-specific tool usage

This is the "thoroughness" that drives up costs without improving outcomes.

---

## Practical Recommendations

### When to Skip AGENTS.md

- Well-documented repositories with README, docs, examples
- Popular repositories with strong conventions (models already know them)
- Simple tasks that don't require context

### When AGENTS.md Helps

- Niche repositories with no documentation
- Custom tooling that differs from standard conventions
- Team-specific patterns that aren't discoverable

### What to Include (If You Write One)

Based on the research, context files should contain **only minimal requirements**:

```
# Build & Test
- Run tests: `pytest tests/`
- Lint: `ruff check .`

# Conventions
- Use `uv` for dependency management
- Follow existing module patterns
```

**Not:**
- Long codebase overviews
- Redundant information from READMEs
- Style guides the model already knows

---

## The "Surprising Behavior" Pattern

When agents encounter something unexpected, that's signal—not noise.

> When agents fail, fix the code, not the prompt. Surprising behavior reveals architectural friction.

Instead of adding more instructions to `AGENTS.md`, consider:

1. **Is the codebase structure confusing?** Rename, reorganize, add comments
2. **Are conventions unclear?** Add type hints, improve names, add docstrings
3. **Is the task underspecified?** Improve the issue description, not the context file

---

## Agent Psychology: The Step-3 Trick

Counterintuitive but effective: if an agent struggles with step 2, tell it to do step 3. The agent often completes step 2 in the process.

This works because:
- Agents reason forward from instructions
- Changing the target reframes the problem
- "Lie" is the feature—controlled misdirection for better outcomes

---

## Token Economics

Context files consume tokens in every request. For a 600-word `AGENTS.md`:

- ~800 tokens of context per request
- Multiplied by every step in every task
- Compounds quickly in long-running sessions

**Question:** Is that token budget better spent on task-specific context (the actual code being modified) or on generic repository context?

The research suggests: task-specific context wins.

---

## Mapping to Cursor Onboarding Kit

| Research Finding | Kit Implication |
|------------------|-----------------|
| LLM-generated files hurt | Don't auto-generate `CLAUDE.md` |
| Minimal requirements help | Keep `AGENTS.md` short |
| Redundancy is waste | Don't repeat README content |
| Surprises reveal friction | Update code, not prompts |

---

## Further Reading

- [AgentBench Harness](https://github.com/eth-sri/agentbench) - Benchmark for evaluating context files
- [BCG Enterprise Agents](22-BCG-ENTERPRISE-AGENTS.md) - Agent Design Cards
- [Formal Verification](23-FORMAL-VERIFICATION-AGENTS.md) - Property-based testing with agents
