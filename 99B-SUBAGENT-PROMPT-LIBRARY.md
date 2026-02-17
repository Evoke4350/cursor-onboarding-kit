# Sub-Agent Prompt Library (Generic, Copy/Paste)

Use this library to improve reliability when delegating work to sub-agents.

## Core Principle

Prompt quality is mostly about constraint quality.

Good prompts define:

- objective
- scope
- evidence required
- verification
- done condition

## Conversation-To-Execution Bridge

You do not need a perfect prompt on first attempt. Use a short discussion loop to refine intent.

Recommended loop:

1. Explain the problem in plain language.
2. Ask agent to restate objective, risks, and assumptions.
3. Correct misunderstandings.
4. Convert to a strict task contract.
5. Delegate execution.

This is how policy architecting becomes implementation quickly and reliably.

The objective is not "write code fast."  
It is "satisfy constraints with verifiable evidence."

## Universal Task Contract (Use In Every Sub-Agent Prompt)

```md
Objective:
<single concrete outcome>

Scope:
In-scope: <paths/domains>
Out-of-scope: <explicit exclusions>

Constraints:
- Follow existing patterns
- Minimize blast radius
- Do not perform unrelated refactors

Evidence Required:
- Files touched/found
- Key findings/decisions
- Verification commands + outcomes

Done When:
<binary completion criteria>
```

## Prompt 1: Parallel Exploration

```md
Investigate <topic> and return findings only (no edits).

Objective:
Map current behavior and risk areas for <topic>.

Scope:
In-scope: <paths>
Out-of-scope: implementation changes

Return format:
1) Key files/components
2) Current behavior
3) Risks/edge cases
4) Recommended implementation slices
5) Verification commands to run after implementation

Done when:
All five sections are complete and scoped to listed paths.
```

## Prompt 1B: Discussion-Only Alignment (No Edits)

```md
Do not edit code yet. We are aligning on policy and constraints first.

Return:
1) Problem restatement in your own words
2) Proposed objective and non-goals
3) Key risks/failure modes
4) Suggested task decomposition
5) Proposed done criteria and verification steps

Then wait for confirmation before implementation.
```

## Prompt 2: Narrow Implementation Slice

```md
Implement <specific slice> with minimal blast radius.

Objective:
Deliver <specific behavior change>.

Scope:
In-scope: <specific files/dirs>
Out-of-scope: unrelated cleanup and broad refactors

Constraints:
- Preserve public contracts unless explicitly requested
- Add/adjust tests for changed behavior

Evidence Required:
- Files changed and why
- Test updates
- Lint/type/test outputs (pass/fail)

Done when:
Behavior is implemented, tests updated, and checks pass.
```

## Prompt 3: Verification Agent

```md
Validate this change independently.

Objective:
Detect regressions and missing coverage for <feature/fix>.

Scope:
Changed files + directly related tests only.

Return format:
1) Validation commands executed
2) Results
3) Regressions found
4) Missing tests
5) Risk level (low/med/high) with rationale

Done when:
All sections are complete with command-backed evidence.
```

## Prompt 4: Large Refactor Orchestrator

```md
Plan and execute <large refactor> using staged sub-tasks.

Stage 1 (exploration):
- identify impacted domains
- produce dependency/risk map

Stage 2 (execution):
- split into independent slices
- execute one slice per sub-task

Stage 3 (integration):
- reconcile overlaps
- run centralized verification

Constraints:
- keep behavior stable unless explicitly in scope
- stop and escalate if unexpected coupling is discovered

Done when:
All slices merged, checks pass, and migration notes are documented.
```

## Prompt 5: PR Packaging Agent

```md
Prepare release-ready PR notes for this branch.

Return:
- Summary (3 bullets max)
- Test plan checklist
- Risks and rollback strategy
- Follow-up items (if any)

Constraints:
- No hype language
- No unverifiable claims

Done when:
Output is concise, accurate, and directly traceable to changes.
```

## Generic Trigger Design (For Any Team)

Treat triggers as routing hints, not magic.

Map by intent class:

- `build/fix/implement` -> implementation flow
- `debug/investigate` -> root-cause flow
- `review/audit` -> findings-first review flow
- `refactor/migrate` -> staged orchestration flow
- `test/qa/verify` -> verification flow

Guidelines:

- Keep trigger taxonomy small (5-8 intents).
- Prefer explicit intent labels over many synonyms.
- Log false positives/negatives in experiments doc.

## Anti-Patterns

- "Fix this" with no scope
- Multi-objective prompts in one task
- Missing done condition
- No evidence requirements
- Allowing sub-agents to redefine architecture silently

## Minimal Quality Check

- Objective is clear.
- Scope is clear.
- Constraints are explicit.
- Evidence requirements are explicit.
- Done condition is explicit.

If two or more are weak, rewrite before delegating.
