# Persona Anchors for Agent Styling

**Pattern:** Direct agents to emulate specific engineering styles and values by anchoring to recognized names in the field.

---

## The Core Insight

LLMs have absorbed the public writing, talks, and code of well-known engineers. This knowledge can be probed and leveraged—without web access—to shape agent behavior toward specific styles and quality standards.

---

## How It Works

1. **Name a practitioner** whose style matches the desired output
2. **Tell the agent to emulate their values and craft**
3. **The agent draws on training data** to approximate that style

This works because LLMs encode patterns from public artifacts—blog posts, conference talks, open-source contributions—associated with named individuals.

---

## Pattern: Domain-Specific Anchors

### React Native Projects

```
Emulate the craft and values of React Native product engineers like
Evan Bacon and Fernando Rojo. Prioritize developer experience,
practical abstractions, and polished user interactions.
```

### Generic React Native (No Specific Product)

```
Emulate the taste and architectural decisions of the Callstack crew.
Focus on maintainability, clear module boundaries, and patterns that
scale across teams.
```

### Backend Systems

```
Emulate the operational discipline of engineers like Kelsey Hightower.
Infrastructure as code, explicit configuration, no snowflakes.
```

### Distributed Systems

```
Emulate the rigor of the FoundationDB or CockroachDB teams. Correctness
first, performance second, explicit handling of edge cases.
```

---

## Probing for Recognition

To discover which names an agent recognizes without web access:

```
Without using web search, describe the engineering values and style
associated with [Name]. What patterns would you expect in their work?
```

If the agent produces a coherent description, that name is usable as an anchor. If the response is vague or generic, choose a different anchor.

---

## When to Use Persona Anchors

| Situation | Effectiveness |
|-----------|---------------|
| Greenfield project, no established patterns | High - provides default direction |
| Codebase with strong existing conventions | Low - existing patterns dominate |
| Team with shared style idols | High - aligns agent with team taste |
| Generic/scaffold code | Medium - adds polish without over-engineering |

---

## Anti-Patterns

- **Over-specifying** - Listing too many names creates confusion
- **Contradictory anchors** - "Like [minimalist] and [enterprise architect]" fights itself
- **Unknown names** - If the model doesn't know them, the anchor is noise
- **Using for factual questions** - Anchors shape style, not correctness

---

## The "Lie" Is the Feature

Persona anchors are a controlled hallucination. The model isn't actually those engineers—but it produces better output when directed toward a coherent style.

This is the same principle as "act as a senior engineer"—except specific names carry more signal because they encode real patterns from training data.

---

## Practical Template

```markdown
## Style Anchor

For this project, emulate the values and craft of [Domain] practitioners
like [Name 1] and [Name 2].

Key characteristics:
- [Value 1]
- [Value 2]
- [Value 3]

When in doubt, ask: "Would [Name] approach it this way?"
```

---

## Mapping to Cursor Onboarding Kit

| Concept | Kit Location |
|---------|--------------|
| Persona anchors | Vibe section in `AGENTS.md` |
| Style direction | Project-specific `CLAUDE.md` extensions |
| Probe for recognition | Interactive onboarding sessions |

---

## Further Reading

- [Agent Design Cards](22-BCG-ENTERPRISE-AGENTS.md) - Formalizing agent intent
- [DSPy Terminology](21-DSPY-TERMINOLOGY.md) - Separating style from logic
