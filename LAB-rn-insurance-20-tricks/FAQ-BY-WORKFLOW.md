# FAQ by Workflow

This file anticipates common instructor/student questions for each workflow step.

## Workflow 1: Discussion-First Alignment

**Q: Why not just ask for the fix directly?**  
A: Direct edits on ambiguous prompts cause drift and wider diffs.

**No-fluff technical truth**  
The model cannot infer your hidden intent. You must externalize constraints.

**Watch for**  
Premature implementation before scope agreement.

**Recovery prompt**  
`Stop coding. Restate objective, in-scope, out-of-scope, risks, and done criteria in 5 bullets.`

## Workflow 2: Contract Definition

**Q: Isn’t this overkill for small bugs?**  
A: A 30-second contract prevents 10-minute cleanup loops.

**No-fluff technical truth**  
Clear constraints tighten token attention around relevant actions.

**Watch for**  
Vague verbs: “improve,” “clean,” “optimize” with no measurable outcome.

**Recovery prompt**  
`Rewrite as a strict task contract: objective, in-scope, out-of-scope, constraints, done condition.`

## Workflow 3: Scoped Execution

**Q: Why enforce one-ticket-one-session?**  
A: Mixed objectives produce mixed diffs and weak reviewability.

**No-fluff technical truth**  
Narrow context reduces branching and accidental file edits.

**Watch for**  
Touched files outside scope.

**Recovery prompt**  
`Revert to scoped execution: edit only <file>. Report if additional files are required and why.`

## Workflow 4: Verification

**Q: The output looks right. Why still verify?**  
A: Plausible output is not equivalent to correct behavior.

**No-fluff technical truth**  
LLMs optimize likely text patterns, not runtime truth.

**Watch for**  
Assertions with no scenario table/checklist.

**Recovery prompt**  
`Provide pass/fail checks for 4 edge cases and map each to expected behavior.`

## Workflow 5: Sub-Agent Exploration

**Q: When do we use explorer/verifier subagents?**  
A: Use them when discovery and execution should be separated.

**No-fluff technical truth**  
Role separation reduces action bias and over-editing.

**Watch for**  
Explorer response includes edits.

**Recovery prompt**  
`No edits allowed. Return findings only: issue, file, severity, recommendation.`

## Workflow 6: Plan-Then-Build

**Q: Why split planning and implementation?**  
A: Planning exposes contradictions before costlier edit loops.

**No-fluff technical truth**  
You are front-loading reasoning into structured context.

**Watch for**  
Plan bypass (edits happen before plan approval).

**Recovery prompt**  
`Pause edits. Produce a 2-step plan with risks and wait for approval.`

## Workflow 7: Commit Narrative Curation

**Q: Isn’t rewriting history deceptive?**  
A: It is acceptable pre-PR when allowed by policy and when provenance is preserved elsewhere.

**No-fluff technical truth**  
Review quality improves when commits match problem-solving milestones.

**Watch for**  
Mixed-purpose commit grouping.

**Recovery prompt**  
`Propose 3-4 commits by intent (fix/refactor/test/docs) with included files.`

## Workflow 8: Adversarial Review

**Q: Why run a devil’s-advocate pass after successful demos?**  
A: Success in one session does not prove robustness at team scale.

**No-fluff technical truth**  
You need negative-case testing to detect fragile assumptions.

**Watch for**  
No explicit failure modes or stress tests.

**Recovery prompt**  
`List top 5 failure risks and one stress test each, with owner and signal.`
