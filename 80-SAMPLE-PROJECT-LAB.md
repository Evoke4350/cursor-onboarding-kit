# Sample Project Lab (Hands-On Practice)

Use this lab to train new Cursor users without relying on client-specific code.

## Lab Goal

Practice a full delivery loop:

- implement a small UI change
- fix a subtle bug
- add tests
- prepare commit and PR write-up

## Instructor Package Option

If you want a ready-to-run instructor package, use:

- `LAB-rn-insurance-20-tricks/README.md`
- `LAB-rn-insurance-20-tricks/PROMPT-PACK-20-TRICKS.md`
- `LAB-rn-insurance-20-tricks/INSTRUCTOR-RUNBOOK.md`

## Recommended Stack

- React + TypeScript app (small starter)
- Jest + Testing Library
- ESLint + TypeScript strict mode

Any equivalent stack is fine if quality gates exist.

## Lab Backlog (Pick 2-3)

1. Rename negative booleans to positive form in one feature module.
2. Replace risky `&&` JSX render guards with explicit ternaries.
3. Add one integration-style test that catches a falsy-value edge case (`0`, `""`).
4. Draft a PR description with Summary/Test Plan/Risks.

## Lab Acceptance Criteria

- Lint/typecheck/tests pass
- No unrelated file changes
- Clear commit message and PR test plan
- Risks and assumptions called out

## Suggested Facilitation

Timebox: 60-90 minutes

1. 10 min: setup + context
2. 35 min: implementation + verification
3. 15 min: commit + PR draft
4. 15 min: group review of prompts and outcomes

## Debrief Questions

- Which prompts produced the best results?
- Where did the agent need tighter constraints?
- What should become a shared team rule?
- What should remain personal/local?
