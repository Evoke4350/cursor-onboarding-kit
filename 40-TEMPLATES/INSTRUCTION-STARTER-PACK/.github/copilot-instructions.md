# Repository Custom Instructions (Template)

This repository contains a small React Native insurance quote flow.

## How To Use This File

- Keep this file short and durable.
- Put path-specific details in `.github/instructions/*.instructions.md`.
- Keep canonical workflow/policy in `AGENTS.md`.

## High-Signal Defaults

- Keep edits scoped to request intent.
- Avoid unrelated refactors.
- Preserve behavior unless the task explicitly changes behavior.
- Prefer positive boolean naming.
- Avoid truthy/falsy UI guards for non-boolean values.

## Validation Expectations

- Run relevant lint/type/test checks when available.
- If checks are unavailable in this environment, provide scenario-based validation evidence.

## File Orientation

Primary behavior is usually in:

- `src/PolicyQuoteScreen.tsx` (screen behavior)
- `src/eligibility.ts` (business logic)
- `src/telemetry.ts` (event payload semantics)
