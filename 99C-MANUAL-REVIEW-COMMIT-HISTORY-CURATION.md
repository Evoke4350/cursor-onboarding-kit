# Advanced Appendix: Manual Review And Commit History Curation

This appendix captures an opinionated but practical position for AI-heavy engineering workflows:

Even when code is largely AI-developed, commit history should be curated for human review and operational traceability.

## Position Statement

Organizations can increasingly assume a meaningful portion of code is AI-assisted.  
That does not reduce accountability; it increases the need for disciplined workflow.

Responsibility model:

- **ICs (individual contributors):** own correctness, commit quality, and evidence quality.
- **Managers/tech leads:** own oversight, consistency, and policy adoption.
- **Organization:** defines acceptable risk, audit expectations, and tool governance.

## Why Curate Commit History

AI can produce large volumes of valid code quickly. Raw history is often not review-friendly.

Curated history improves:

- reviewer comprehension
- rollback precision
- incident forensics
- onboarding and knowledge transfer

## Core Principle

The commit timeline should reflect the problem-solving trajectory in human-readable steps, not the accidental order of agent edits.

In practice: align commits to logical milestones derived from prompt intent and solution stages.

## Recommended SOP

1. **Stabilize working tree** under full backpressure (hooks, lint, typecheck, tests, CI).
2. **Group changes by intent** (fix, refactor, test, docs, migration step).
3. **Rewrite local branch history** into coherent units before review.
4. **Use explicit commit messages** describing why the step exists.
5. **Validate again** after curation to confirm no behavior drift.
6. **Publish PR with narrative** that matches commit sequence.

## Backpressure Framing

Do not waste backpressure.

Backpressure includes:

- instruction files (for example `AGENTS.md`)
- local hooks / pre-commit checks
- CI/CD gates
- reviewer feedback loops

These controls convert AI speed into reliable delivery.

## Governance Boundaries (Org-Dependent)

Some organizations allow routine local history rewriting before PR.  
Others restrict rewriting due to compliance/audit controls.

Adopt policy by tier:

- **Allowed:** curate branch commits pre-PR for readability.
- **Restricted:** keep linear append-only history; rely on PR narrative quality.
- **Strictly controlled:** require signed-off workflow with documented exceptions.

Always follow repository and compliance rules for branch rewriting and force-push behavior.

## Agent-Operator Discretion Model

Operational quality emerges from four layers:

1. **SOP** (repeatable process)
2. **Principles** (decision standards)
3. **Policy** (organizational constraints)
4. **Operator discretion** (judgment under ambiguity)

AI improves throughput; operator discretion preserves safety and intent.

## Conversation-Driven Curation Pattern

A useful operating pattern is to discuss commit history intentionally with the agent:

- define the intended narrative arc
- map changed files to milestones
- ask for candidate commit grouping
- refine until history is reviewer-friendly

This is especially valuable for large AI-generated diffs.

## Example Milestone Commit Shape

- `fix(checkout): normalize falsy guards for numeric inputs`
- `refactor(state): rename negative booleans to positive intent`
- `test(checkout): add regression coverage for empty/zero edge cases`
- `docs(pr): capture risk and rollback notes`

## Review Checklist For Curated Histories

- [ ] Each commit has one clear purpose
- [ ] Commit order tells a coherent story
- [ ] No hidden unrelated changes
- [ ] Tests and validation map to behavior changes
- [ ] PR summary matches commit trajectory

## Final Guidance

As teams normalize AI-assisted development, workflow quality becomes the control plane.

Readable commit history is not cosmetic.  
It is part of production safety, review efficiency, and organizational trust.
