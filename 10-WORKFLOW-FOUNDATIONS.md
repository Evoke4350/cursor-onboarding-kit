# Workflow Foundations (Generic)

This is the default operating model for AI-assisted delivery.

## Core Loop

1. Clarify task outcome
2. Gather relevant context
3. Implement smallest safe change
4. Verify with project quality gates
5. Summarize, commit, and open PR

## Prompting Principle

Ask for outcomes, constraints, and verification - not keystroke-level instructions.

## Example Working Prompt

`Implement <task> with minimal blast radius. Follow existing patterns. Run lint/typecheck/tests relevant to changed files. Report assumptions and risks.`

## Guardrails

- Keep diffs small
- Avoid unrelated refactors unless requested
- Do not skip quality checks
- Require explicit test plan in PR

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
