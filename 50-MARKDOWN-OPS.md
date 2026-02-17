# Markdown Ops (Lightweight Management System)

Use this to prevent note/document sprawl.

## Suggested Structure

- `00-inbox/` - raw capture
- `10-active/` - currently used docs
- `20-shipped/` - merged/delivered artifacts
- `40-templates/` - reusable templates
- `90-archive/` - old/inactive docs

## Naming Convention

`<TYPE>-<slug>-<YYYYMMDD>.md`

Examples:

- `RFC-agent-governance-20260216.md`
- `PR-checkout-falsy-fix-20260216.md`
- `PLAN-cursor-onboarding-20260216.md`

## Frontmatter Standard

```md
---
title: <title>
type: <RFC|PR|PLAN|DEMO|NOTE|RUNBOOK>
status: <draft|active|shipped|archived>
owner: <name>
created: <YYYY-MM-DD>
updated: <YYYY-MM-DD>
tags: [cursor, ai-workflow]
---
```

## Lifecycle Rules

- New docs start in `00-inbox/`
- Move to `10-active/` when in active use
- Move to `20-shipped/` after delivery/merge
- Move to `90-archive/` after 30+ days inactivity

## Required Indexes

- `INDEX.md` - global map of important docs
- `ACTIVE.md` - this sprint's working set
- `EXPERIMENTS.md` - rule/prompt/model experiments and outcomes
