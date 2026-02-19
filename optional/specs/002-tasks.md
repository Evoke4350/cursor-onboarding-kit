# Spec 002: Per-Document Task Beads
**Parent PRD:** specs/000-PRD.md
**Date:** 2026-02-17

Each row is one bead (task) to be created under its section epic.
Tasks are created lazily — section epics are ready first, tasks created when a section is started.

The bd create command pattern for each task is:

```bash
bd create "DOC: {path}" \
  -t task -p 2 \
  --description "Fetch https://docs.cursor.com/en/{path}.md → extract verbatim quotes → write LAB-changelog-viewer/output/docs/{path}.md following strict protocol (specs/000-PRD.md). Date: 2026-02-17." \
  --deps "parent:{section-epic-id}" \
  --json
```

---

## Section: get-started (bead: cursor-onboarding-kit-1lm)

| Path | Title | Description |
|---|---|---|
| get-started/installation | Installation | Get Cursor installed on your computer |
| get-started/quickstart | Quickstart | Get started with Cursor in 5 minutes |
| get-started/concepts | Concepts | Key features that make Cursor powerful |

## Section: agent core (bead: cursor-onboarding-kit-26z)

| Path | Title | Description |
|---|---|---|
| agent/overview | Overview | Autonomous coding tasks, terminal commands, code editing |
| agent/modes | Modes | Choose the right mode — autonomous to focused edits |
| agent/planning | Planning | Plans, todos, and queuing for complex tasks |
| agent/terminal | Terminal | Run terminal commands as part of agent operations |
| agent/tools | Tools | Tools available for searching, editing, running code |
| agent/apply | Apply | Accept or reject code suggestions using Apply |
| agent/review | Diffs & Review | Review and manage AI-generated code changes |

## Section: agent/chat (bead: cursor-onboarding-kit-bs0)

| Path | Title | Description |
|---|---|---|
| agent/chat/checkpoints | Checkpoints | Save and restore previous states |
| agent/chat/compact | Compact | Compact mode interface |
| agent/chat/commands | Commands | Reusable workflow commands |
| agent/chat/duplicate | Duplicate | Branch from any point in a conversation |
| agent/chat/export | Export | Export chats to markdown |
| agent/chat/history | History | View and manage chat conversations |
| agent/chat/summarization | Summarization | Context management for long conversations |
| agent/chat/tabs | Tabs | Run multiple Agent conversations simultaneously |

## Section: background-agent (bead: cursor-onboarding-kit-c1z)

| Path | Title | Description |
|---|---|---|
| background-agent | Background Agents | Async remote agents overview |
| background-agent/web-and-mobile | Web & Mobile | Run agents from any device |

## Section: background-agent/api (bead: cursor-onboarding-kit-cvo)

| Path | Title | Description |
|---|---|---|
| background-agent/api/overview | API Overview | Programmatic background agent management |
| background-agent/api/launch-an-agent | Launch Agent | Start a new background agent |
| background-agent/api/agent-status | Agent Status | Get status and results |
| background-agent/api/agent-conversation | Agent Conversation | Retrieve conversation history |
| background-agent/api/add-followup | Add Follow-up | Send additional instruction to running agent |
| background-agent/api/delete-agent | Delete Agent | Permanently delete agent |
| background-agent/api/list-agents | List Agents | Paginated list of all background agents |
| background-agent/api/list-models | List Models | Recommended models for background agents |
| background-agent/api/list-repositories | List Repositories | GitHub repos accessible to authenticated user |
| background-agent/api/api-key-info | API Key Info | Metadata about authentication API key |
| background-agent/api/webhooks | Webhooks | Real-time status notifications |

## Section: cli core (bead: cursor-onboarding-kit-ioe)

| Path | Title | Description |
|---|---|---|
| cli/overview | Cursor CLI | Get started with CLI |
| cli/using | Using Agent in CLI | Prompt, review, iterate |
| cli/headless | Headless CLI | Scripts for automated code analysis |
| cli/shell-mode | Shell Mode | Run shell commands without leaving conversation |
| cli/installation | Installation | Install and update Cursor CLI |

## Section: cli/reference (bead: cursor-onboarding-kit-ytk)

| Path | Title | Description |
|---|---|---|
| cli/reference/parameters | Parameters | Complete command reference |
| cli/reference/permissions | Permissions | Permission types for file/command access |
| cli/reference/authentication | Authentication | Browser flow or API key auth |
| cli/reference/configuration | Configuration | cli-config.json reference |
| cli/reference/output-format | Output Format | Text, JSON, stream-JSON schemas |
| cli/reference/slash-commands | Slash Commands | Quick actions in CLI sessions |

## Section: cli/cookbook (bead: cursor-onboarding-kit-3x9)

| Path | Title | Description |
|---|---|---|
| cli/cookbook/code-review | Code Review | GitHub Actions workflow for automated PR review |
| cli/cookbook/fix-ci | Fix CI Failures | Fix CI issues via Cursor CLI in GitHub Actions |
| cli/cookbook/secret-audit | Secret Audit | Audit secrets via Cursor CLI |
| cli/cookbook/translate-keys | Translate Keys | Translate keys via Cursor CLI |
| cli/cookbook/update-docs | Update Docs | Update docs via Cursor CLI |

## Section: cli integrations (bead: cursor-onboarding-kit-cqt)

| Path | Title | Description |
|---|---|---|
| cli/github-actions | GitHub Actions | Cursor CLI in CI/CD |
| cli/mcp | MCP | Use MCP servers with cursor-agent |

## Section: context/@-symbols (bead: cursor-onboarding-kit-c6d)

| Path | Title | Description |
|---|---|---|
| context/@-symbols/overview | Overview | Reference code, files, docs using @ |
| context/@-symbols/@-code | @Code | Reference specific code snippets |
| context/@-symbols/@-cursor-rules | @Cursor Rules | Apply project-specific rules |
| context/@-symbols/@-files-and-folders | @Files & Folders | Reference files and folders |
| context/@-symbols/@-git | @Git | Reference Git changes |
| context/@-symbols/@-link | @Link | Include web content by URL |
| context/@-symbols/@-linter-errors | @Linter Errors | Access linting errors |
| context/@-symbols/@-past-chats | @Past Chats | Include chat history summaries |
| context/@-symbols/@-recent-changes | @Recent Changes | Include recently modified code |
| context/@-symbols/@-web | @Web | Search the web |
| context/@-symbols/pill-files | #Files | Select files using # prefix |
| context/@-symbols/slash-commands | /command | Add files and control context |

## Section: context system (bead: cursor-onboarding-kit-wru)

| Path | Title | Description |
|---|---|---|
| context/rules | Rules | Reusable, scoped agent behavior instructions |
| context/mcp | MCP | Connect external tools via Model Context Protocol |
| context/codebase-indexing | Codebase Indexing | How Cursor learns your codebase |
| context/ignore-files | Ignore Files | .cursorignore and .cursorindexingignore |
| context/memories | Memories | Persistent memory across sessions |

## Section: inline-edit (bead: cursor-onboarding-kit-mir)

| Path | Title | Description |
|---|---|---|
| inline-edit/overview | Inline Edit | Cmd+K for editing and questions |
| inline-edit/terminal | Terminal | Generate terminal commands with Cmd+K |

## Section: tab (bead: cursor-onboarding-kit-lvl)

| Path | Title | Description |
|---|---|---|
| tab/overview | Tab | Autocomplete with multi-line edits, cross-file suggestions |

## Section: models (bead: cursor-onboarding-kit-b60)

| Path | Title | Description |
|---|---|---|
| models | Models | Auto mode, Max Mode, context windows |

## Section: settings (bead: cursor-onboarding-kit-yvl)

| Path | Title | Description |
|---|---|---|
| settings/api-keys | API Keys | Bring your own LLM provider |

## Section: configuration (bead: cursor-onboarding-kit-1rc)

| Path | Title | Description |
|---|---|---|
| configuration/kbd | Keyboard Shortcuts | Keybindings in Cursor |
| configuration/themes | Themes | Customize appearance |
| configuration/shell | Shell Commands | Install and use shell commands |

## Section: integrations (bead: cursor-onboarding-kit-zqo)

| Path | Title | Description |
|---|---|---|
| integrations/git | Git | AI commit messages, merge conflict resolution |
| integrations/github | GitHub | Official Cursor GitHub app for background agents |
| integrations/linear | Linear | Background Agents from Linear |
| integrations/slack | Slack | Background Agents from Slack |

## Section: bugbot (bead: cursor-onboarding-kit-d7v)

| Path | Title | Description |
|---|---|---|
| bugbot | Bugbot | AI code review for pull requests |

## Section: account (bead: cursor-onboarding-kit-1hz)

| Path | Title | Description |
|---|---|---|
| account/agent-security | Agent Security | Security considerations for Cursor Agent |
| account/billing | Billing | Subscriptions, refunds, invoices |
| account/pricing | Pricing | Plans and pricing |
| account/update-access | Update Access | Update frequency control |

## Section: account/teams (bead: cursor-onboarding-kit-zap)

| Path | Title | Description |
|---|---|---|
| account/teams/setup | Team Setup | Create and set up a Cursor team |
| account/teams/dashboard | Dashboard | Billing, usage, team settings |
| account/teams/enterprise-settings | Enterprise Settings | Centrally manage org settings |
| account/teams/members | Members & Roles | Manage team members |
| account/teams/admin-api | Admin API | Team metrics and spending via API |
| account/teams/ai-code-tracking-api | AI Code Tracking API | AI-generated code analytics |
| account/teams/analytics | Analytics | Team usage and activity metrics |
| account/teams/analyticsV2 | Analytics V2 | Advanced analytics |
| account/teams/scim | SCIM | Automated user provisioning |
| account/teams/sso | SSO | Single sign-on setup |

## Section: guides/migration (bead: cursor-onboarding-kit-2m9)

| Path | Title | Description |
|---|---|---|
| guides/migration/vscode | VS Code | Import VS Code settings and extensions |
| guides/migration/jetbrains | JetBrains | Migrate from JetBrains IDEs |

## Section: guides/languages (bead: cursor-onboarding-kit-r25)

| Path | Title | Description |
|---|---|---|
| guides/languages/python | Python | Python dev with extensions and linting |
| guides/languages/javascript | JavaScript & TypeScript | JS/TS with framework support |
| guides/languages/java | Java | JDK, extensions, build tools |
| guides/languages/swift | iOS & macOS (Swift) | Cursor with Xcode for Swift |

## Section: guides/advanced (bead: cursor-onboarding-kit-iui)

| Path | Title | Description |
|---|---|---|
| guides/advanced/large-codebases | Large Codebases | Working with large codebases |
| guides/advanced/datascience | Data Science | Python, R, SQL workflows |
| guides/advanced/working-with-documentation | Working with Documentation | Leverage docs via prompting |

## Section: guides/tutorials (bead: cursor-onboarding-kit-zuo)

| Path | Title | Description |
|---|---|---|
| guides/tutorials/web-development | Web Development | Cursor for web dev |
| guides/tutorials/architectural-diagrams | Architectural Diagrams | Mermaid diagrams |
| guides/tutorials/building-mcp-server | Building an MCP Server | MCP server with PostgreSQL |

## Section: guides/working-with-context (bead: cursor-onboarding-kit-2yw)

| Path | Title | Description |
|---|---|---|
| guides/working-with-context | Working with Context | Context workflow guide |

## Section: tools (bead: cursor-onboarding-kit-z6m)

| Path | Title | Description |
|---|---|---|
| tools/mcp | MCP Servers | Explore and install MCP servers |
| tools/developers | Developers | Generate install links for Tools & MCP |

## Section: troubleshooting (bead: cursor-onboarding-kit-vil)

| Path | Title | Description |
|---|---|---|
| troubleshooting/common-issues | Common Issues | FAQ and common problems |
| troubleshooting/request-reporting | Getting a Request ID | Find request IDs for support |
| troubleshooting/troubleshooting-guide | Troubleshooting Guide | Steps to fix and report bugs |

## Section: welcome (bead: cursor-onboarding-kit-j70)

| Path | Title | Description |
|---|---|---|
| welcome | Welcome | Learn about Cursor and how to get started |
