# Solution Key (Deterministic)

Use this key to evaluate whether each prompt produced the intended outcome.

## Canonical End-State (after implementation prompts)

## `src/eligibility.ts`

Expected intent:

- Replace negative naming with positive equivalents (for readability)
- Correct age boundary: 18 should be eligible (reject only `< 18`)
- Correct serious-condition logic (should not auto-pass)
- Correct high-mileage + claims logic (should not auto-pass to low risk)
- Keep function signatures stable

## `src/PolicyQuoteScreen.tsx`

Expected intent:

- Eligibility display should reflect actual eligibility semantics
- Premium banner condition should be explicit boolean logic
- Remove logic inversion between `isApplicantIneligible` and displayed label
- Keep UI structure mostly unchanged (minimal blast radius)

## `src/telemetry.ts`

Expected intent:

- `is_eligible` should not invert source value
- `monthly_premium` should remain numeric (no `"n/a"` fallback)
- Event payload keys remain stable and semantically correct

---

## Prompt-by-Prompt Expected Outcomes

## 1) Discussion alignment
Expected: no code edits; 3 bug clusters + staged plan.
Fallback if drift:
`Stop coding. Return only bug clusters and execution sequence.`

## 2) Contract baseline
Expected: explicit objective/in-scope/out-of-scope for eligibility only.
Fallback:
`Rewrite into strict contract with done condition.`

## 3) One-ticket execution
Expected: edits only in `src/eligibility.ts`.
Fallback:
`Restrict edits to src/eligibility.ts and explain why if impossible.`

## 4) Minimal blast radius
Expected: small edit set in `PolicyQuoteScreen.tsx`; no broad refactor.
Fallback:
`Undo broad changes; apply smallest possible fix for eligibility label logic.`

## 5) Verification table
Expected: pass/fail table with at least 4 scenarios.
Fallback:
`Provide scenario table with expected vs actual and pass/fail.`

## 6) Plan-then-build
Expected: plan first; edits only after approval.
Fallback:
`Pause edits and provide two-step plan with risks.`

## 7) Team-vs-personal style discipline
Expected: no style churn unrelated to bug fixes.
Fallback:
`Keep only functional bug-fix changes; drop preference-only edits.`

## 8) Local context discipline
Expected: no references outside lab files.
Fallback:
`Ground response only in src/* files and list them explicitly.`

## 9) Positive booleans
Expected: rename map and clearer conditions; no double negatives.
Fallback:
`Replace negative boolean semantics with positive naming and update conditionals.`

## 10) Ternary guard
Expected: explicit boolean condition for banner rendering.
Fallback:
`Use explicit boolean/ternary; avoid truthy coercion on numeric values.`

## 11) Explorer role
Expected: mismatch findings only; no edits.
Fallback:
`No edits. Return telemetry field mismatch list with severity.`

## 12) Parallel suitability
Expected: yes/no decision with dependency rationale.
Fallback:
`Return binary parallel recommendation and two concrete coupling points.`

## 13) Completion contract execution
Expected: telemetry fixes + completion checklist pass/fail.
Fallback:
`Apply telemetry fixes and return checklist for is_eligible/monthly_premium/field names.`

## 14) Commit grouping
Expected: 3-4 coherent commit groups by intent.
Fallback:
`Propose commit groups: fix/refactor/test/docs with file lists.`

## 15) PR standardization
Expected: Summary/Test plan/Risks with concise evidence.
Fallback:
`Reformat into Summary, Test plan, Risks/Rollback only.`

## 16) Markdown lifecycle
Expected: valid frontmatter with lifecycle fields.
Fallback:
`Provide frontmatter with type/status/owner/updated and 3 bullets.`

## 17) Experiment log
Expected: hypothesis -> result -> decision.
Fallback:
`Write experiment with keep/modify/drop outcome and follow-up.`

## 18) Rule pruning design
Expected: baseline/prune/rerun/compare methodology.
Fallback:
`Return 4-step pruning experiment with measurable before/after criteria.`

## 19) Safe external ingest
Expected: env-var-based checklist + generic safe example.
Fallback:
`Provide secret-safe ingest checklist and generic curl using env vars.`

## 20) Adversarial review
Expected: risk register with stress tests and owner.
Fallback:
`List top 5 risks with one stress test and owner per risk.`

---

## Quick Instructor Pass Criteria

- Output is scoped to requested files
- Completion criteria are satisfied explicitly
- No hidden or implied cross-scope edits
- Verification evidence is present where requested
- Language remains clear and mono-semantic
