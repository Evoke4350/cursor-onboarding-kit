# Cursor Tips and Tricks Scorecard (20 Tricks)

This is the "show me the tricks" version of the kit.

## Trick 1: Discussion-Only Alignment First (multi-step)

Before code edits, ask for:

1. restated problem
2. objective + non-goals
3. risks
4. proposed done criteria

Why it works: converts fuzzy intent into executable constraints.

## Trick 2: Contract Prompt Skeleton

Always include:

- Objective
- In-scope / Out-of-scope
- Constraints
- Evidence required
- Done condition

## Trick 3: One Ticket, One Session

Do not mix unrelated tasks in one agent session.

Why it works: cleaner diffs, cleaner review, lower context drift.

## Trick 4: Ask for "Minimal Blast Radius"

Add this phrase to implementation prompts.

Why it works: reduces incidental edits and noisy commits.

## Trick 5: Verification-First Closeout

Require lint/typecheck/tests and command-backed result summary before signoff.

## Trick 5A: Property-Based Tests as Oracles (optional)

Turn "this should never happen" into executable properties, then let the generator hunt edge cases.

Why it works: you get a real oracle (pass/fail) that a long-running agent can grind against.

## Trick 6: Plan-Then-Build for Multi-File Work (multi-step)

1. run planning pass
2. approve plan
3. execute in slices
4. central verification

Why it works: prevents "agent did something else" behavior.

## Trick 7: Split Team vs Personal Rules

- Team policy in committed files
- Personal preferences local-only

Why it works: avoids policy churn and git noise.

## Trick 8: Local Exclude for Personal Files

Use `.git/info/exclude` for personal config:

- `AGENTS.local.md`
- `CLAUDE.local.md`
- local/private cursor folders

## Trick 9: Positive Boolean Naming Rule

Prefer `isEnabled` over `!isDisabled`.

Why it works: fewer negation bugs and cleaner conditionals.

## Trick 10: Explicit Ternary Render Guards

Prefer `condition ? <Node /> : null` over truthy/falsy guards on non-booleans.

Why it works: avoids `0`/`""` render edge-case bugs.

## Trick 11: Sub-Agent Roles by Job

Use:

- explorer for discovery
- executor for implementation
- verifier for checks

## Trick 12: Parallelize Only Independent Domains

Limit default parallelism to 2-3 agents unless slices are truly independent.

## Trick 13: Reward Contract for Sub-Agents

Define reward as:

- measurable output
- evidence
- binary done condition

Not "write code quickly."

## Trick 14: Candidate Commit Grouping by Intent (multi-step)

1. map changed files to milestones
2. group by intent (fix/refactor/test/docs)
3. curate readable history
4. re-verify

Why it works: human-reviewable commit trajectory for AI-heavy branches.

## Trick 15: PR Output Standardization

Always generate:

- 3-bullet summary
- test checklist
- risk/rollback notes

## Trick 16: Markdown Lifecycle to Kill Sprawl

Use capture -> active -> shipped -> archive folder flow.

## Trick 17: Keep an Experiments Log

Track prompt/rule/model experiments with keep/modify/drop decisions.

## Trick 18: Rule Pruning Cycle (multi-step)

1. identify ignored/conflicting rules
2. delete low-signal rules
3. rerun real ticket
4. compare quality and speed

Why it works: reduces token tax and improves clarity.

## Trick 19: Use External Context Ingest Safely

Fetch Jira/Confluence/Figma with env vars + local private folders; sanitize before sharing.

## Trick 20: Run Adversarial Review Before Scale-Up

Apply devil's-advocate checklist before broad rollout.

Why it works: catches blind spots before organizational lock-in.

## BONUS Trick 21: Remind the model it has REPL access

Tell the model to use a Python REPL (or shell tooling) when scanning large file sets or doing repetitive analysis.

Use it for: indexing files, counting patterns, clustering duplicates, and producing summary tables before deep reads.

Why it works: models can default to naive read/search loops; a quick reminder unlocks higher-leverage tool use and better signal-to-noise.

## Unintuitive Trick 0: Models can cheat, stop them in their tracks

Tell the model which files are read-only before it starts. Example: "Do not edit tests in this pass."

Use it for: large existing test suites, high-risk modules, or any task where safety beats speed.

Why it works: if boundaries are vague, outputs often prioritize apparent task completion over constraints you intended to preserve.

## Unintuitive Trick 00A: Make it justify every deletion

Require a one-line reason for every deleted line (or deleted block) before changes are applied.

Use it for: refactors, cleanup passes, and "remove dead code" tasks.

Why it works: deletion is easy to overdo; forcing justification reduces accidental logic loss.

## Unintuitive Trick 00B: Force a failure-first read

Ask the model to first list three ways the change could break in production before it writes code.

Use it for: checkout, billing, auth, telemetry, and migration work.

Why it works: front-loading failure modes tends to produce more risk-aware outputs before code generation begins.

## Unintuitive Trick 00C: Ban generic confidence language

Disallow phrases like "looks good" or "should work." Require evidence statements only.

Use it for: final summaries, code reviews, and handoff notes.

Why it works: confidence language hides uncertainty; evidence language exposes assumptions and test gaps.

---

## Dictation One-Liners (Memorize These)

Use these as short operator prompts you can say out loud.

1. "Read these files, define in-scope and out-of-scope, then make one medium safe change."
2. "Do not edit tests in this pass; fix behavior in source only."
3. "Before coding, list three likely failure modes in production."
4. "For every deletion, give a one-line justification first."
5. "No generic confidence language; give evidence only."
6. "Use Python REPL for bulk scan, then read only targeted slices."
7. "If uncertain, ask one clarifying question before writing code."
8. "Keep blast radius minimal; no unrelated refactors."
9. "Use explicit boolean logic; avoid truthy/falsy guards for numbers and strings."
10. "Preserve telemetry semantics; call out contract impact explicitly."
11. "Return results as: changes made, evidence, risks, rollback."
12. "Treat AGENTS as canonical; mirror compatibility through bridge files."

### Fast Variants (Ultra-Short)

- "Scope lock, then code."
- "Failure modes first."
- "Evidence over confidence."
- "No test edits this pass."
- "Minimal blast radius."
- "REPL first, deep read second."

### Mode Trigger Variants (Memorize)

- "Let's discuss first, no edits."
- "Switch to plan mode and propose options."
- "Plan first, wait for my approval."
- "Treat this as debugging, reproduce before edits."
- "Use runtime evidence, not just static reading."
- "Use the question tool and ask me one decision question."

Why it works: in this kit's workflows, explicit mode and behavior language is more reliable than implicit intent.

### Idiom Trigger Variants (Advanced)

- "Use a city metaphor: mayor, crews, cargo, checkpoints; map each to concrete code roles."
- "Explain this as a logistics pipeline, then convert it back into implementation steps."
- "Give one analogy for the architecture, then list exact files/functions each part maps to."

Why it works: analogies can improve structure and memory, but only if mapped back to concrete boundaries.

---

## Where These Tricks Came From

These are distilled from:

- `02-NO-FLUFF-OPERATING-GUIDE.md`
- `03-EXPERIENCED-ENGINEER-LENS-QA.md`
- `10-WORKFLOW-FOUNDATIONS.md`
- `50-MARKDOWN-OPS.md`
- `60-PERSONALIZATION-LOCAL-ONLY-CONFIG.md`
- `75-GITHUB-COPILOT-CONFIG-ADVANCED.md`
- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`
- `99B-SUBAGENT-PROMPT-LIBRARY.md`
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`
- `97-DEVILS-ADVOCATE-ADVERSARIAL-TAKE.md`
