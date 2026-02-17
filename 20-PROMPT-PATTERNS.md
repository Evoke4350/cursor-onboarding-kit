# Prompt Patterns (Copy/Paste)

Use these as starting points. Replace placeholders.

## 1) Small Feature

`Implement <feature> in <area>. Keep blast radius minimal and follow existing conventions. Add/update tests for changed behavior. Run lint/typecheck/relevant tests and report results.`

## 2) Bug Fix

`Investigate and fix <bug>. Reproduce first, identify root cause, then implement minimal fix. Add regression coverage. Report root cause, fix strategy, and residual risk.`

## 3) Refactor (Safe)

`Refactor <component/module> for readability/maintainability without behavior changes. Keep external contracts stable. Run verification and call out any risky assumptions.`

## 4) Commit Preparation

`Prepare commit for current work: stage only related files, draft commit message in repo style, commit, then show final git status.`

## 5) PR Draft

`Draft PR description with: Summary (3 bullets), Test Plan (checklist), Risks/Rollback notes. Keep concise and concrete.`

## 6) Review Mode

`Review this diff for bugs, regressions, missing tests, and risk hotspots. Prioritize findings by severity and include concrete fixes.`

## Prompt Tips

- Be explicit about verification commands.
- Ask for assumptions and tradeoffs.
- Ask for "minimal blast radius" to reduce incidental edits.
- For reviews, ask for findings first, summary second.

## 7) Prompt Casting (Advanced)

`Cast this task as <role> for <goal>. Use <constraints>. Return <output shape>.`

Examples:

- `Cast this as a production incident responder. Goal: restore checkout flow fast. Constraints: no schema changes, no broad refactor. Output: root cause, patch, rollback, tests.`
- `Cast this as a senior reviewer. Goal: catch regressions. Constraints: findings only, severity ordered, include exact fix suggestion per finding.`

Use prompt casting to force decision posture, not style.
