# Epilogue: Frontier Sub-Agent Orchestration

This appendix covers advanced orchestration patterns for using sub-agents in Cursor to solve large, high-coupling tasks.

Use this only after your baseline workflow is stable.

## When Sub-Agents Are Worth It

Use sub-agents when:

- task spans multiple domains (UI + API + tests + infra)
- exploration can be parallelized (independent codebase regions)
- work includes long-running or specialized flows (browser testing, heavy shell validation)
- one agent would otherwise lose context over many steps

Avoid sub-agents when:

- task is narrow and local (single-file edits, small bugfix)
- coordination overhead exceeds expected gains
- acceptance criteria are vague

## Agent Roles Pattern (Practical)

Use a simple 3-role model:

1. **Orchestrator (you/main agent):** owns goal, plan, acceptance criteria, final integration
2. **Explorer(s):** gather context in parallel and return scoped findings
3. **Executor(s):** implement or validate narrow slices with explicit boundaries

Rule: orchestrator retains architectural ownership and merge decisions.

## How To Decompose Large Tasks

Break by independent concerns:

- code discovery / impact analysis
- implementation slices
- tests and verification
- docs/PR prep

Bad split: "fix everything in checkout flow."  
Good split:

- Agent A: map checkout render-state paths and risky falsy guards
- Agent B: map analytics/event payload contracts
- Agent C: enumerate related tests and current coverage gaps

## Reward Structure For Agent Success

Treat "reward" as measurable completion criteria.

Each sub-agent task should include:

- **Goal:** one concrete outcome
- **Boundary:** files/areas in scope; explicit out-of-scope
- **Evidence required:** commands run, outputs summarized, artifacts created
- **Quality gates:** lint/type/test checks relevant to scope
- **Done definition:** binary, auditable completion condition

Example reward contract:

- "Return exactly: impacted files, root cause, proposed fix, and verification commands with pass/fail result."

This reduces vague "looks done" outcomes.

## Prompt Template For Sub-Agent Tasks

Use this structure:

1. Objective
2. Scope and constraints
3. Required output format
4. Verification expectations
5. Stop conditions / escalation path

Example:

`Investigate <domain>. Scope: <paths>. Do not edit files. Return: (1) findings, (2) risks, (3) recommended edit plan, (4) exact commands to verify.`

## Parallelism Heuristics

- Start with 2-3 agents max for most engineering tasks.
- Increase only when domains are truly independent.
- Keep one owner per concern to avoid overlap churn.
- Prefer short-lived agents with clear handoff output.

## Managing Sub-Agent Quality

- Require structured outputs (bullet schema/checklist)
- Ask for evidence, not assertions
- Compare outputs for contradictions before editing
- Resolve ambiguities in orchestrator layer before execution

## Cursor-Specific Management Notes

From practical Cursor usage:

- Use `explore` agents for broad codebase discovery.
- Use `shell` agents for terminal-heavy flows.
- Use `browser-use` agents for multi-step web interactions/testing.
- Use `generalPurpose` for mixed reasoning tasks.
- Keep delegated prompts explicit because sub-agents do not inherit intent unless provided.
- For parallel work, launch multiple sub-agents in one batch only when tasks are independent.

## Orchestration Workflow (Large Refactor)

1. Orchestrator defines target behavior and non-goals.
2. Launch exploration sub-agents in parallel.
3. Consolidate findings into one implementation plan.
4. Launch narrow execution tasks (one concern per sub-agent).
5. Run centralized verification in orchestrator.
6. Prepare commit and PR narrative from integrated evidence.

## Anti-Patterns

- Delegating full problem statement without boundaries
- Running too many agents on overlapping files
- Accepting "done" without verifiable outputs
- Letting sub-agents set architecture implicitly
- Mixing exploration and execution in the same unconstrained task

## Advanced Use Cases

- Cross-cutting refactors (naming conventions, API contract normalization)
- Migration prep (inventory + risk table + phased execution)
- Reliability hardening (test-gap mapping + failure mode analysis)
- Documentation synchronization after complex code changes

## Suggested KPI Set (For Teams)

- time-to-first-correct-PR
- rework rate after review
- defect escapes on orchestrated changes
- % tasks completed without escalation
- parallel task utilization vs merge-conflict overhead

## Final Guidance

Sub-agent orchestration is a force multiplier only when task decomposition and success criteria are explicit.

Think in terms of:

- small accountable units
- evidence-backed handoffs
- centralized integration and verification

That is the frontier practice that scales safely.
