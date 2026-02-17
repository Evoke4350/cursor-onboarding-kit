# Instructor Runbook: RN Insurance 20-Trick Lab

This runbook is optimized for a single operator. Optional subagent variants are included for advanced sessions.

## Session Goals

- Show that prompt structure controls outcomes
- Demonstrate discussion -> constraints -> execution -> verification
- Produce review-ready artifacts from the same workflow

## Timeboxes

- Fast demo: 30 minutes (top 8 prompts)
- Standard demo: 60 minutes (prompts 1-15)
- Full demo: 90 minutes (all 20 prompts + adversarial pass)

## Suggested Sequence

1. Prompt 1 (discussion-only alignment)
2. Prompt 2 (contract definition)
3. Prompt 3 (scoped implementation)
4. Prompt 4 + 10 (UI correctness + explicit rendering guard)
5. Prompt 13 (reward contract on telemetry)
6. Prompt 5 (verification table)
7. Prompt 14 + 15 (commit and PR narrative quality)
8. Prompt 20 (adversarial risk review)

## Demo Beats ("wow" moments)

## Beat A - Intent clarity beats verbosity
Run Prompt 1 then Prompt 2 back-to-back.
Show how the quality of the next implementation request improves when constraints are explicit.

## Beat B - Same bug, better prompt, better diff
Compare a broad ask vs Prompt 4 (minimal blast radius).
Highlight reduced diff noise and faster reviewability.

## Beat C - Reward shaping
Run Prompt 13 and require checklist output.
Show that binary done criteria produce less ambiguous completions.

## Beat D - Human-readable delivery
Run Prompt 14 and Prompt 15.
Show how AI-heavy work still becomes review-friendly through narrative curation.

## Operating Script (Your voice profile)

Use this baseline phrase pattern:

`Read these files, make this medium-sized change, keep scope tight, show proof.`

Optional style add-on:

`We are discussing first, no edits yet.`

## Troubleshooting Matrix

| Symptom | Likely cause | Instructor fix |
|---|---|---|
| Model edits too much | scope too broad | re-run with Prompt 3/4 structure |
| Wrong logic but confident output | no verification gate | run Prompt 5 immediately |
| Drifts into unrelated files | mixed session goals | enforce one-ticket-one-session |
| Overly verbose answer | missing output schema | re-run with strict output format |
| Ignores constraints | weak salience | move constraints to top and shorten wording |

## Optional Subagent Variant

Use only in advanced mode:

- Explorer: Prompt 11 (findings only)
- Executor: Prompt 13 (apply telemetry fixes)
- Verifier: Prompt 5 (pass/fail table)

Keep orchestrator (instructor) as final integration owner.

## Instructor Checklist

- [ ] Use discussion-first at least once
- [ ] Use reward contract at least once
- [ ] Show verification evidence before claiming done
- [ ] Show commit/PR narrative workflow
- [ ] Run adversarial pass before closing session
