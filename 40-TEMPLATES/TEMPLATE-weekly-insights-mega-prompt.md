# Weekly Insights Mega-Prompt (One-Shot or Chain-Shot)

Use this once per week with evidence inputs.
Do not run this as a daily loop.

## Evidence Inputs (attach or summarize)

- chat history excerpts
- shared transcript excerpts
- commit history (week range)
- PR review notes/findings
- optional: DevSQL query output

## One-Shot Prompt

```
Act as an engineering workflow analyst.

Objective:
Generate a weekly reinforcement report that improves delivery reliability without over-optimizing for perfection.

Hard constraints:
- No medical or mental-health diagnosis.
- Treat psychographic and biological signals as self-reported workflow context only.
- Use evidence from attached artifacts; if evidence is weak, say so.
- Prefer 1-3 policy changes max for next week.
- Reject recommendations that increase process overhead without clear benefit.

Context to analyze:
1) Prompting behavior
2) Tool usage patterns
3) Scope drift and rework events
4) Friction signals (frustration, indecision, repeated retries)
5) Energy/attention signals (time-of-day, session length, break patterns if present)

Return exactly this structure:

## A) Evidence Summary (max 10 bullets)
- what happened this week
- strongest and weakest signals

## B) Friction Patterns
- Pattern
- Evidence
- Impact
- Confidence (low/medium/high)

## C) High-Yield Behaviors To Keep
- Behavior
- Evidence
- Why it worked

## D) Guardrail Updates (max 3)
- Proposed update text
- Target file (`AGENTS.local.md`, `AGENTS.md`, rule file, command file, skill file)
- Expected benefit
- Risk
- Rollback trigger

## E) Stop-Doing List (max 5)
- anti-pattern
- replacement behavior

## F) Next-Week Experiment Plan
- 1-2 experiments only
- success metric
- failure metric
- end date

## G) Draft Commands/Skills
- `/weekly-insights` command body
- skill outline for `.cursor/skills/weekly-insights/SKILL.md`

## H) Safety Check
- ways this report could overfit noise
- what evidence is still missing
- what should NOT be changed yet
```

## Chain-Shot Variant

Run these in order:

1) Evidence extraction pass  
`Extract only concrete signals from artifacts. No recommendations yet.`

2) Pattern clustering pass  
`Cluster signals into 3-6 recurring patterns.`

3) Policy drafting pass  
`Propose up to 3 guardrail updates with rollback triggers.`

4) Skeptic pass  
`Challenge each proposed update as if you were a critical reviewer.`

5) Final synthesis pass  
`Return the final report structure (A-H) with only evidence-backed updates.`

## Suggested DevSQL Prompt

```
Use DevSQL to find sessions where prompt count was high and commit count was low.
Then list prompts that most often preceded same-day commits.
Summarize differences in structure and intent between the two groups.
```

Reference: https://github.com/douglance/devsql
