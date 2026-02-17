# Glossary + Frontmatter Guide

## Glossary

- **Agent**: AI system that can inspect files, edit code, and run commands.
- **Rule (`.mdc`)**: Cursor project rule file in `.cursor/rules/` with YAML frontmatter.
- **Frontmatter**: metadata block at the top of markdown files, wrapped in `---`.
- **`alwaysApply`**: Cursor rule toggle for always-on vs conditional activation.
- **`globs`**: file patterns controlling where conditional rules apply.
- **`AGENTS.md`**: agent instruction file used by multiple agent workflows.
- **`AGENTS.local.md`**: local-only personal instruction file (exclude from git).
- **Blackboard**: local markdown workspace for notes, RFCs, prompts, and experiments.
- **ROVO content ingest**: pulling Jira/Confluence/Figma context into local markdown safely.

## What Frontmatter Is

Frontmatter is structured metadata at the top of markdown. Tools read this to classify, filter, and scope files.

Basic example:

```md
---
title: Cursor onboarding notes
type: NOTE
status: active
owner: <name>
created: 2026-02-16
updated: 2026-02-16
tags: [cursor, onboarding]
---
```

## Frontmatter In This Kit

You use frontmatter in:

- Cursor rules (`.cursor/rules/*.mdc`)
- Blackboard markdown documents (optional but recommended)

## Minimal Safe Frontmatter Standard

Use these keys for knowledge docs:

- `title`
- `type`
- `status`
- `owner`
- `created`
- `updated`
- `tags`

## Common Mistakes

- Missing closing `---`
- Invalid YAML syntax
- Inconsistent `type/status` values
- Overloading frontmatter with unnecessary keys

## Read More

- YAML frontmatter in GitHub docs  
  https://docs.github.com/en/contributing/writing-for-github-docs/using-yaml-frontmatter
- VitePress frontmatter guide  
  https://vitepress.dev/guide/frontmatter
- Hugo front matter docs  
  https://gohugo.io/content-management/front-matter/
