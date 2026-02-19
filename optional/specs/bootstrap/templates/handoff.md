# Handoff: <workstream or issue>

## Snapshot

- Owner: <name/team>
- Date: <YYYY-MM-DD>
- Status: <green/yellow/red>
- Confidence: <low/medium/high>

## Checkpoint (for recovery)

- **Phase:** [idle|triage|scope|implement|verify|deliver]
- **Position:** [file:line or where work stopped]
- **State file:** `{{MEMORY:state}}/checkpoint.yaml`
- **Session log:** `{{MEMORY:short}}/sessions/YYYYMMDD-HHMM.md`

## Current state

<What is done, what is partially done, and what is blocked. Keep it scannable.>

## Open items

| Item | Priority | Owner | Next action |
|---|---|---|---|
|  |  |  |  |

## Risks and blockers

| Risk/Blocker | Impact | Mitigation | Escalation needed |
|---|---|---|---|
|  |  |  |  |

## Known decisions

- Decision: <what was chosen>
- Rationale: <why>
- Alternatives rejected: <what/why>

## Next 24-48h plan

- [ ] <Task 1>
- [ ] <Task 2>

## First move for next owner

<Single next action to regain momentum in under 15 minutes.>

## Artifacts

- PRs: <links>
- RFCs: <links>
- Memory: `{{MEMORY:long}}/*.md`
