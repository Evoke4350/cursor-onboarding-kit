# Spec 001: Section Breakdown
**Parent PRD:** specs/000-PRD.md
**Date:** 2026-02-17

All 28 sections with their bead IDs, doc counts, and priority order for execution.
Priority is based on relevance to the demo talk (engineers new to Cursor).

---

| Priority | Section | Bead ID | Doc Count | Rationale |
|---|---|---|---|---|
| 1 | get-started | cursor-onboarding-kit-1lm | 3 | First thing any new user sees |
| 2 | agent core (overview/modes/planning/terminal/tools/apply/review) | cursor-onboarding-kit-26z | 10 | Primary agent surface |
| 3 | models | cursor-onboarding-kit-b60 | 1 | Auto mode — active question |
| 4 | context/rules + mcp + indexing + ignore + memories | cursor-onboarding-kit-wru | 5 | How context works is foundational |
| 5 | context/@-symbols/* | cursor-onboarding-kit-c6d | 10 | @ reference is muscle memory |
| 6 | agent/chat/* | cursor-onboarding-kit-bs0 | 8 | Chat UI features |
| 7 | tab | cursor-onboarding-kit-lvl | 1 | Tab completion |
| 8 | inline-edit | cursor-onboarding-kit-mir | 2 | Cmd+K |
| 9 | background-agent | cursor-onboarding-kit-c1z | 2 | Async agents overview |
| 10 | background-agent/api/* | cursor-onboarding-kit-cvo | 10 | Programmatic API |
| 11 | cli core | cursor-onboarding-kit-ioe | 4 | CLI for terminal engineers |
| 12 | cli/reference/* | cursor-onboarding-kit-ytk | 6 | CLI reference |
| 13 | cli/cookbook/* | cursor-onboarding-kit-3x9 | 5 | CI/CD recipes |
| 14 | cli/github-actions + cli/mcp | cursor-onboarding-kit-cqt | 2 | CLI integrations |
| 15 | integrations | cursor-onboarding-kit-zqo | 4 | Git, GitHub, Linear, Slack |
| 16 | bugbot | cursor-onboarding-kit-d7v | 1 | AI PR review |
| 17 | guides/migration | cursor-onboarding-kit-2m9 | 2 | VS Code + JetBrains |
| 18 | guides/languages | cursor-onboarding-kit-r25 | 4 | Language-specific setups |
| 19 | guides/advanced | cursor-onboarding-kit-iui | 3 | Large codebases, data science |
| 20 | guides/tutorials | cursor-onboarding-kit-zuo | 3 | Web dev, diagrams, MCP server |
| 21 | guides/working-with-context | cursor-onboarding-kit-2yw | 1 | Context guide |
| 22 | tools | cursor-onboarding-kit-z6m | 2 | MCP server browser |
| 23 | configuration | cursor-onboarding-kit-1rc | 3 | Kbd, themes, shell |
| 24 | settings/api-keys | cursor-onboarding-kit-yvl | 1 | BYO keys |
| 25 | account | cursor-onboarding-kit-1hz | 7 | Billing, security |
| 26 | account/teams/* | cursor-onboarding-kit-zap | 7 | Enterprise/team admin |
| 27 | troubleshooting | cursor-onboarding-kit-vil | 3 | Support flows |
| 28 | welcome | cursor-onboarding-kit-j70 | 1 | Entry point |

---

## URL Pattern

All English docs are at:
```
https://docs.cursor.com/en/{path}.md
```

The `.md` suffix returns the raw markdown source — no HTML scraping needed. This is the exact URL to fetch for each page.

Example:
- `https://docs.cursor.com/en/agent/overview.md`
- `https://docs.cursor.com/en/models.md`
- `https://docs.cursor.com/en/cli/reference/parameters.md`

---

## Output File Structure

```
LAB-changelog-viewer/output/docs/
├── get-started/
│   ├── installation.md
│   ├── quickstart.md
│   └── concepts.md
├── agent/
│   ├── overview.md
│   ├── modes.md
│   ├── planning.md
│   ├── terminal.md
│   ├── tools.md
│   ├── apply.md
│   ├── review.md
│   └── chat/
│       ├── checkpoints.md
│       ├── compact.md
│       ├── commands.md
│       ├── duplicate.md
│       ├── export.md
│       ├── history.md
│       ├── summarization.md
│       └── tabs.md
├── background-agent/
│   ├── overview.md
│   ├── web-and-mobile.md
│   └── api/
│       ├── overview.md
│       ├── launch-an-agent.md
│       ├── agent-status.md
│       ├── agent-conversation.md
│       ├── add-followup.md
│       ├── delete-agent.md
│       ├── list-agents.md
│       ├── list-models.md
│       ├── list-repositories.md
│       ├── api-key-info.md
│       └── webhooks.md
├── cli/
│   ├── overview.md
│   ├── using.md
│   ├── headless.md
│   ├── shell-mode.md
│   ├── github-actions.md
│   ├── mcp.md
│   ├── installation.md
│   ├── reference/
│   │   ├── parameters.md
│   │   ├── permissions.md
│   │   ├── authentication.md
│   │   ├── configuration.md
│   │   ├── output-format.md
│   │   └── slash-commands.md
│   └── cookbook/
│       ├── code-review.md
│       ├── fix-ci.md
│       ├── secret-audit.md
│       ├── translate-keys.md
│       └── update-docs.md
├── context/
│   ├── rules.md
│   ├── mcp.md
│   ├── codebase-indexing.md
│   ├── ignore-files.md
│   ├── memories.md
│   └── @-symbols/
│       ├── overview.md
│       ├── @-code.md
│       ├── @-cursor-rules.md
│       ├── @-files-and-folders.md
│       ├── @-git.md
│       ├── @-link.md
│       ├── @-linter-errors.md
│       ├── @-past-chats.md
│       ├── @-recent-changes.md
│       ├── @-web.md
│       ├── pill-files.md
│       └── slash-commands.md
├── inline-edit/
│   ├── overview.md
│   └── terminal.md
├── tab/
│   └── overview.md
├── models.md
├── settings/
│   └── api-keys.md
├── configuration/
│   ├── kbd.md
│   ├── themes.md
│   └── shell.md
├── integrations/
│   ├── git.md
│   ├── github.md
│   ├── linear.md
│   └── slack.md
├── bugbot.md
├── account/
│   ├── agent-security.md
│   ├── billing.md
│   ├── pricing.md
│   ├── update-access.md
│   └── teams/
│       ├── admin-api.md
│       ├── ai-code-tracking-api.md
│       ├── analytics.md
│       ├── analyticsV2.md
│       ├── dashboard.md
│       ├── enterprise-settings.md
│       ├── members.md
│       ├── scim.md
│       ├── setup.md
│       └── sso.md
├── guides/
│   ├── migration/
│   │   ├── vscode.md
│   │   └── jetbrains.md
│   ├── languages/
│   │   ├── python.md
│   │   ├── javascript.md
│   │   ├── java.md
│   │   └── swift.md
│   ├── advanced/
│   │   ├── large-codebases.md
│   │   ├── datascience.md
│   │   └── working-with-documentation.md
│   ├── tutorials/
│   │   ├── web-development.md
│   │   ├── architectural-diagrams.md
│   │   └── building-mcp-server.md
│   └── working-with-context.md
├── tools/
│   ├── mcp.md
│   └── developers.md
├── troubleshooting/
│   ├── common-issues.md
│   ├── request-reporting.md
│   └── troubleshooting-guide.md
└── welcome.md
```
