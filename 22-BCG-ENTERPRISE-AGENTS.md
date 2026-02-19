# BCG Enterprise Agent Design Patterns (2025)

**Reference:** [Building Effective Enterprise Agents (BCG, November 2025)](https://www.bcg.com/assets/2025/building-effective-enterprise-agents.pdf)

BCG's AI Platforms Group published a comprehensive framework for building production-grade enterprise agents. This document captures the key patterns relevant to the Cursor Onboarding Kit.

---

## Agent Design Cards (ADCs)

Agent Design Cards are BCG's standardized blueprint for documenting agent requirements. An effective ADC should:

1. **Define purpose** - Clearly describe what the agent is designed to achieve
2. **Clarify boundaries** - Specify the agent's role, scope, and points of human oversight
3. **Detail inputs and outputs** - Make data sources, dependencies, and deliverables explicit
4. **Describe capabilities** - Outline tools and capabilities needed for the agent's success
5. **Anticipate failure** - Define fallback behavior, escalation paths, and guardrails

### Example Agent Design Card

```
Agent Goal: Reduce processing time for loan applications

Metrics:
  - 30% reduction in manual exception handling time

Skills, Tools & Capabilities:
  - Document parsing and field validation
  - Cross-system data reconciliation (CRM, Credit Bureau)
  - Policy-based reasoning for exception routing

Agent Trigger: System-led

Input(s) & Output(s):
  - Inputs: Loan application data, validation rules from policy database
  - Outputs: Audit log of actions and corrections performed, exceptions

Fallback:
  - Notify loan officer via workflow system for manual intervention

Priority: 1
```

---

## Agent Suitability Framework

Not every problem needs an agent. Use this framework to decide:

| Risk/Governance → | Low | High |
|-------------------|-----|------|
| **High Complexity** | Agent-led with Human Oversight | Human-led with Agent Support |
| **Low Complexity** | Agent-led (full autonomy) | Traditional Automation |

**Key insight:** If clear rules and basic automation deliver the desired outcome, avoid building agents for agents' sake.

---

## Agent Maturity Horizons

| Horizon | Type | Description | Example |
|---------|------|-------------|---------|
| 0 | Constrained agents | Predefined rules, single repetitive task | Simple Q&A chatbot |
| 1 | Single agents | Multi-step tasks in set environment, plans and acts alone | Onboarding assistant |
| 2 | Deep agents | Orchestrator splits tasks for specialist agents | Employee onboarding (sets up email, access, etc.) |
| 3 | Role-based agents | Team of agents collaborate, distinct roles, handoffs | Marketing campaign agents |
| 4 | Agent mesh | Network of autonomous agents that self-organize | Supply chain optimization |

**Recommendation:** Build toward Horizon 2 (deep agents) today. Fully autonomous mesh agents require mature reasoning and evaluation systems.

---

## Human Oversight Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| **Agent-assisted** | Agent provides output to normal user workflow | ChatGPT |
| **Human-in-the-loop** | Agent makes decision, awaits human approval | Claude Code |
| **Human-on-the-loop** | User observes outputs, can intervene if issues flagged | Crew AI |
| **Human-out-of-the-loop** | Agent acts without explicit human oversight | Standalone support agents |

---

## Design Principles

### Start Simple, Iterate with Evals

1. Begin with a single observe-reason-act loop
2. Introduce sub-flows only when complexity causes brittleness
3. Add specialized agents only when domain-specific tasks require them

### Outcome-First Design

Start with business outcomes ("What are we trying to achieve?"), then decompose:

```
Outcome: 30% faster loan approvals
  → Dependencies: document verification, exception handling, fewer manual handoffs
  → Agent opportunities: automated resolutions, remediation suggestions
```

### Context Engineering

Prevent context pollution with these strategies:

| Strategy | Description |
|----------|-------------|
| **Compression** | Summarize context as window nears limit |
| **Pruning** | Remove old or irrelevant content |
| **Ranking** | Ensure most relevant information is visible |
| **Isolation** | Split task/context across sub-agents |
| **Notes** | Let agents take structured notes during sessions |

---

## Memory Architecture

| Type | Description | Duration |
|------|-------------|----------|
| **Short-term (STM)** | Context window: instructions, knowledge, tools | Single session |
| **Semantic (LTM)** | Abstract, factual, domain-specific knowledge | Persistent |
| **Procedural (LTM)** | How to perform tasks or skills | Persistent |
| **Episodic (LTM)** | Past events as example behaviors | Persistent |

---

## Platform Decision Framework

| Platform Type | Best For | Trade-offs |
|---------------|----------|------------|
| **Standalone Agentic Solutions** | Fast, narrow capability for one team | Limited extensibility |
| **Embedded Agentic Platforms** | In-suite agents leveraging native data/workflows | Vendor lock-in |
| **Agent Builder Platforms** | Governed low/no-code builder for broad use | Engineering lift |
| **Custom-Built Platforms** | Differentiating use cases with bespoke logic | Highest operational burden |

### Decision Criteria

1. **Differentiation** - Is this critical to competitive differentiation?
2. **Execution** - Do we have capabilities to build?
3. **Gravity factors** - Data proximity, system integration, governance needs

---

## Failure Modes

| Category | Examples | Mitigations |
|----------|----------|-------------|
| **Identity/Auth** | Agent impersonated, unintended actions | Unique identifiers, granular permissions, audit trails |
| **Data supply-chain** | Prompt injection, harmful content | Input validation, XPIA protection, monitor data flows |
| **Orchestration** | Tool failures, agent deadlocks | Control flow guardrails, scoped environments |
| **Reasoning** | Hallucinations, task drift | Monitor reasoning patterns, granular roles |
| **Operations** | Resource overuse, cost explosion | Rate limits, timeouts, isolation |

---

## Key Takeaways

1. **Design for outcomes, not outputs** - Anchor on measurable business outcomes
2. **Start simple and iterate** - Single observe-reason-act loop first
3. **Build on shared foundations** - Standardize runtimes, gateways, guardrails
4. **Choose the right platform** - Based on data gravity, governance, differentiation
5. **Engineer trust by default** - Identity, access control, monitoring, evaluation

---

## Mapping to Cursor Onboarding Kit

| BCG Concept | Cursor Onboarding Kit Equivalent |
|-------------|----------------------------------|
| Agent Design Cards | Issue templates in `40-TEMPLATES/` |
| Human oversight patterns | Review workflows in `AGENTS.md` |
| Context engineering | Prompt patterns in `20-PROMPT-PATTERNS.md` |
| Memory architecture | State machine in `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md` |
| Failure modes | Guardrails in `AGENTS.md` (Before You Stop, Landing the Plane) |
