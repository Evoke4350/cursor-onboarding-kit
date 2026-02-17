# Generic Template Clusters

Use these as a starter set for internal consultancy work. They are intentionally broad and can be pruned per engagement.

Editorial goal: each output should read like a short working memo from an experienced engineer, not a form filled by a bot.

## Core delivery templates

- `TEMPLATE-briefing.md`
- `TEMPLATE-napkin.md`
- `TEMPLATE-runbook.md`
- `TEMPLATE-evaluation-guide.md`
- `TEMPLATE-handoff.md`
- `TEMPLATE-slide-deck.md`
- `TEMPLATE-principles.md`
- `TEMPLATE-tech-writer-rfc.md`
- `TEMPLATE-decision-record.md`
- `TEMPLATE-risk-register.md`
- `TEMPLATE-release-readout.md`

## AI workflow templates

- `TEMPLATE-ai-output-provenance.md`
- `TEMPLATE-workflow-chain.md`
- `TEMPLATE-persona-packet.md`

## Existing templates

- `TEMPLATE-pr.md`
- `TEMPLATE-rfc.md`
- `TEMPLATE-experiment.md`

## Suggested naming conventions

- `PR-<topic>-<ticket>.md`
- `RFC-<topic>-<YYYYMMDD>.md`
- `NAPKIN-<question>-<YYYYMMDD>.md`
- `RUNBOOK-<system>-<topic>.md`
- `BRIEFING-<topic>-<YYYYMMDD>.md`

## Metadata convention (optional)

Add this at top of template outputs when useful:

```yaml
---
title: "<document title>"
owner: "<name/team>"
status: "draft"
last_updated: "YYYY-MM-DD"
source_type: "human | ai-assisted | ai-generated"
model_hint: "<optional-model-or-agent-name>"
---
```

## Humanization checklist (quick)

- Open with the decision or tension, not process language.
- Replace vague filler with concrete nouns and verbs.
- Keep one audience in mind per document (IC, lead, or manager).
- Add one sentence on "why now" and one on "what changes if we act."
- Keep templates short enough that teams will actually use them.
