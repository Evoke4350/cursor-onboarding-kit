# LLM Mechanics for Engineers (No-Fluff)

This is a compact technical appendix for instructors who want light jargon without academic overhead.

## Core Concepts Used in This Lab

- **Context window pressure**: too much preamble reduces useful working context.
- **Instruction hierarchy**: higher-priority constraints dominate weaker goals.
- **Attention salience**: explicit constraints are more likely to be followed than implied intent.
- **Action bias**: models tend to start coding unless constrained to analyze first.
- **Scope drift**: broad prompts expand edit surface unpredictably.
- **Schema shaping**: fixed output formats improve consistency.
- **Completion shaping**: explicit done criteria increases completion reliability.
- **Evaluator loop**: verification prompts convert generation into check mode.
- **Retrieval collision**: mixed tasks in one session produce blended context and noisy edits.
- **Variance control**: narrow prompts reduce response randomness.

## Workflow -> Mechanics Mapping

| Workflow | Primary mechanics |
|---|---|
| Discussion-first | action bias suppression, intent disambiguation |
| Contract definition | instruction hierarchy, salience |
| Scoped execution | variance control, scope drift prevention |
| Verification | evaluator loop, confidence calibration |
| Sub-agent role split | specialization, reduced mode collapse |
| Commit curation | semantic chunking for human review |
| Adversarial pass | negative-case surfacing |

## Trick -> Mechanics Mapping (20)

1. Discussion alignment -> action bias suppression  
2. Contract skeleton -> instruction hierarchy  
3. One ticket/session -> retrieval collision prevention  
4. Minimal blast radius -> scope drift reduction  
5. Verification-first -> evaluator loop activation  
6. Plan-then-build -> decomposition before generation  
7. Team vs personal split -> policy conflict reduction  
8. Local excludes -> context boundary control  
9. Positive booleans -> lower negation complexity  
10. Ternary guards -> explicit boolean semantics  
11. Role split -> specialization  
12. Parallel limits -> coordination overhead control  
13. Completion contract -> completion shaping  
14. Commit grouping -> cognitive load reduction  
15. PR schema -> output variance reduction  
16. Markdown lifecycle -> retrieval hygiene  
17. Experiments log -> closed-loop tuning  
18. Rule pruning -> preamble token reduction  
19. Safe ingest -> secret/context isolation  
20. Adversarial review -> robustness testing

## Quick Heuristics Instructors Can Repeat

- If output is broad, tighten scope before changing models.
- If output is plausible-but-wrong, switch to verification mode.
- If diffs are noisy, force file-level scope and minimal blast radius.
- If rules are ignored, shorten rule text and increase constraint clarity.
- If sessions degrade over time, reset context with a fresh contract.
