# Workflow Foundations (Generic)

This is the default operating model for AI-assisted delivery.

## Core Loop

1. Clarify task outcome
2. Gather relevant context
3. Implement smallest safe change
4. Verify with project quality gates
5. Summarize, commit, and open PR

## Why This Works (Type-Checkable Lens)

Treat the loop like a pipeline. Each step should produce an output that the next step can consume.

```ts
type Triage = { fixOrder: string[]; risks: string[] };
type Contract = { inScope: string[]; outOfScope: string[]; done: string[] };
type Change = { filesChanged: string[] };
type Evidence = { checks: Array<{ name: string; result: "pass" | "fail" }>; notes: string[] };
type Delivery = { commits: string[]; pr: { summary: string[]; testPlan: string[]; rollback: string[] } };
```

Where it falls apart:

- You skip outputs ("looks good" isn't a type).
- You have no oracle (no tests, no checks, no way to tell pass from fail).
- You add hidden state (multiple agent layers, long memory blobs, vague scope).

## Prompting Principle

Ask for outcomes, constraints, and verification - not keystroke-level instructions.

## Example Working Prompt

`Implement <task> with minimal blast radius. Follow existing patterns. Run lint/typecheck/tests relevant to changed files. Report assumptions and risks.`

## Guardrails

- Keep diffs small
- Avoid unrelated refactors unless requested
- Do not skip quality checks
- Require explicit test plan in PR

## Verification Is The Oracle

- The more machine-checkable your verification is, the more you can safely delegate to agents.
- Property-based tests are basically "lemmas": encode invariants once, let the generator try to break them.
- Long-running / "grind" style agents are only useful when the oracle is crisp (tests or checklists). Otherwise they just thrash.

## Team vs Personal Separation

Team baseline:

- shared coding standards
- quality gates
- PR expectations

Personal local:

- workflow preferences
- prompt snippets
- experimentation rules

Promote local patterns to team policy only after repeated positive outcomes.
