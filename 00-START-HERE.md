# Start Here: 45-Minute Cursor Onboarding

Use this agenda to train engineers who are new to Cursor.

## Learning Goals

- Understand where guidance should live (team vs personal)
- Execute a complete coding loop with verification
- Use prompts that reduce iteration churn
- Know when to introduce model switching

## 45-Minute Agenda

## 0-5 min: Mental Model

Explain the stack:

1. Team baseline (`AGENTS.md` or equivalent)
2. Local/personal overrides (`AGENTS.local.md` or local equivalent)
3. Scoped rules (`.cursor/rules/*.mdc`)
4. Working notes (blackboard/markdown workspace)

## 5-15 min: Show Rule Layering

Demo one simple coding preference rule and one safety rule.

Outcome: attendees understand that not all preferences belong in team policy.

## 15-30 min: Live Ticket Workflow

Run a real small task:

1. Ask agent to inspect context
2. Implement change
3. Run lint/tests/typecheck
4. Summarize risks/assumptions

## 30-38 min: Commit + PR Quality

Show how to ask for:

- clean staging (only related files)
- commit message in repo style
- PR summary + test plan + risk notes

## 38-45 min: Team Adoption + Next Steps

- Establish minimum workflow standard
- Introduce markdown templates
- Flag model switching as phase-2 capability
- Show local-only personalization via `.git/info/exclude`
- Assign one sample lab from `80-SAMPLE-PROJECT-LAB.md`

## Definition Of "Good First Week"

- 2-3 merged PRs using this workflow
- no drop in quality gates
- reduced review noise
- at least one documented prompt/rule experiment
