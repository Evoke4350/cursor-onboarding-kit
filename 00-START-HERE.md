# Start Here: Five-Step Habit

You do not need to read this whole repo. Most teams get most of the value from one chain run well.

If you're panicking because you feel like you need to understand everything: you don't. You need one win.

## Lifeline (Pick One File)

Forget the repo for now. Pick one file that's breaking your flow and paste this into your agent chat:

```text
No repo tour. No docs.
Read only this file.

1) What is it supposed to do?
2) What is it doing instead?
3) What's the smallest safe fix?

Then stop and ask me one question if you need to.
```

Ship the smallest fix with evidence. Then decide if you want the deeper loop.

## Minimum Viable Chain (Use This First)

Run one real ticket with these five prompts:

1. Discussion
`No edits yet. Restate the problem, likely risks, and fix order in five bullets.`
2. Scope
`Write a strict contract: in-scope, out-of-scope, and measurable done condition.`
3. Execute
`Implement only this contract with minimal blast radius. Ask before crossing scope.`
4. Verify
`Show a scenario table with expected vs actual and pass/fail. Call out uncertainty.`
5. Package
`Propose commit groups by intent and draft a PR summary, test plan, and rollback note.`

If this loop is clean, stop there and ship. Deeper docs are optional.

## Fast Path (Senior Engineer Edition)

If this is a tiny change and you already understand the bug:

1. Scope + execute in one prompt.
`Fix <bug> with minimal blast radius. No refactors. If you need to touch another file, ask first.`
2. Verify + package in one prompt.
`Run the relevant checks and give me: what changed, what ran, results, risks, rollback, and a PR summary.`

If the problem is ambiguous, cross-cutting, or multi-file: use the full five-step chain.

## 45-Minute Team Onboarding (Optional)

Use this when training a group:

1. 0-5 min: mental model (team rules vs local rules vs scoped rules)
2. 5-15 min: one live "discussion then scope" pass
3. 15-30 min: one scoped fix + verification table
4. 30-38 min: commit grouping + PR draft
5. 38-45 min: adoption defaults for week one

## Demo

- Lab bugs are intentionally planted for teaching.
- Real production bugs are messier.
- The chain still holds because it was designed for ambiguity, not toy perfection.

## Definition of a Good First Week

- 2-3 merged PRs using the five-step chain
- No drop in quality gates
- Lower review noise (smaller diffs, clearer PRs)
- At least one prompt or rule promoted after proving value
