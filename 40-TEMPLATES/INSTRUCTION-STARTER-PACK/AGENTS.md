# AGENTS.md (Template)

## Project Context

Generic React Native insurance quote flow with three core files:

- `src/PolicyQuoteScreen.tsx`
- `src/eligibility.ts`
- `src/telemetry.ts`

## Non-Negotiables

- Ship the smallest useful change.
- Avoid unrelated refactors in bugfix work.
- Prefer positive boolean names (`isEnabled` over `isDisabled`).
- Use explicit render conditions for UI guards.
- Keep telemetry field semantics stable.

## Workflow

1. Clarify objective and scope.
2. Propose a plan for multi-file edits.
3. Implement minimal blast-radius edits.
4. Verify with lint/typecheck/tests or scenario checks.
5. Return risks, assumptions, and rollback path.

## Validation

- Run relevant checks before signoff.
- Provide evidence, not just confidence statements.
- Call out assumptions and unresolved ambiguity explicitly.
