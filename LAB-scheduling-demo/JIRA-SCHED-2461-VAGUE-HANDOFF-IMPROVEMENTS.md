# SCHED-2461: Improve On-Call Handoff Experience (Need ASAP)

**Type:** Story  
**Priority:** Medium-High (but maybe urgent?)  
**Story Points:** 3 or 8 (not sure, depends)  
**Reporter:** PM-Core-Platform  
**Assignee:** Unassigned  
**Status:** Backlog

## Summary

Can we make handoff and scheduling "less scary" and more "enterprise ready" before the April customer pulse checks?

## Description

Right now things are technically working-ish, but multiple teams said the outputs feel confusing and sometimes alarming. We should smooth the rough edges and maybe align labels/colors/wording and escalation behavior so it looks intentional.

I do not want a huge rewrite. I do want fewer weird edge-case surprises.

## Requested Outcome (Loose)

- Coverage numbers should be easier to explain in reviews.
- Escalation should feel reliable.
- Timezone weirdness should not create "what the hell happened" moments.
- Keep existing exports stable if possible.

## Acceptance (Draft, Not Final)

1. Ops managers stop posting confusion screenshots in Slack for one week.
2. APAC team says timezone behavior is "better" (exact metric TBD).
3. No breaking changes for downstream CSV consumers.

## Design / Artifacts

- External Figma JSON export (fake external reference):  
  `https://figma-deliverables.example-internal.com/ops/sched2461/frame-118-export-v7.json`
- Local mirror of that payload for offline lab work:  
  `CONTEXT-VAGUE-SCHED-2461/04-figma-frame-118-export.json`

## Additional Context Pack (Progressive Disclosure)

Start with this ticket only, then reveal files below one at a time:

- `CONTEXT-VAGUE-SCHED-2461/01-slack-thread-ops-war-room.md`
- `CONTEXT-VAGUE-SCHED-2461/02-confluence-page-handoff-principles.md`
- `CONTEXT-VAGUE-SCHED-2461/03-robo-digest-oncall-risk.txt`
- `CONTEXT-VAGUE-SCHED-2461/04-figma-frame-118-export.json`
- `CONTEXT-VAGUE-SCHED-2461/05-jira-comment-dump-sched-2461.txt`
- `CONTEXT-VAGUE-SCHED-2461/06-ai-tool-briefing-alt-proposal.md`
- `CONTEXT-VAGUE-SCHED-2461/07-zendesk-escalation-sample.csv`
- `CONTEXT-VAGUE-SCHED-2461/08-runbook-snippet-escalation-policy.yaml`
- `CONTEXT-VAGUE-SCHED-2461/09-datadog-query-notes.sql`
- `CONTEXT-VAGUE-SCHED-2461/10-loom-transcript-snippet.md`

## Notes

This ticket is intentionally vague for training. The real goal is to simulate how scope changes as context arrives from mixed-quality sources.

