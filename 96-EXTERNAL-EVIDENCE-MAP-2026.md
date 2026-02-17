# External Evidence Map (Curated, Authoritative)

This map uses primary product docs, official changelogs/blogs, and established platform references.
It intentionally excludes user-forum bug reports as core evidence.

## Context and Retrieval Architecture

1. Dynamic context discovery (official)
- Source: https://cursor.com/blog/dynamic-context-discovery
- Why it matters: explains on-demand context loading and token-efficiency behavior.
- Kit mapping: `79-CURSOR-CONTEXT-FOCUS-30-WAYS.md`, `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

2. Semantic search docs
- Source: https://cursor.com/docs/context/semantic-search
- Why it matters: formal retrieval behavior for code understanding at scale.
- Kit mapping: `79-CURSOR-CONTEXT-FOCUS-30-WAYS.md`

3. Cloud agents docs
- Source: https://cursor.com/docs/cloud-agent
- Why it matters: asynchronous runs and remote execution model.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

## Workflow Surfaces and Reuse

4. Commands docs
- Source: https://cursor.com/docs/agent/chat/commands
- Why it matters: reusable command workflows for teams.
- Kit mapping: `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`

5. Skills docs
- Source: https://cursor.com/docs/context/skills
- Why it matters: procedure packaging via `SKILL.md`.
- Kit mapping: `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`

6. Subagents docs
- Source: https://cursor.com/docs/context/subagents
- Why it matters: context-isolated delegation and parallel execution.
- Kit mapping: `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`, `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`

## Security and Operations

7. Agent security docs
- Source: https://cursor.com/docs/agent/security
- Why it matters: approval/sandbox safety posture.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

8. Hooks docs
- Source: https://cursor.com/docs/agent/hooks
- Why it matters: policy/event automation layer for governance.
- Kit mapping: `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`

9. Third-party hooks docs
- Source: https://cursor.com/docs/agent/third-party-hooks
- Why it matters: policy/event automation through external hook handlers.
- Kit mapping: `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`

10. Permissions reference (CLI)
- Source: https://cursor.com/docs/cli/reference/permissions
- Why it matters: concrete allow/deny execution controls.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

## Reviews, History, and Sharing

11. Bugbot docs
- Source: https://cursor.com/docs/bugbot
- Why it matters: structured AI review workflow and integration.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

12. Bugbot out-of-beta (official)
- Source: https://cursor.com/blog/bugbot-out-of-beta
- Why it matters: production-scale signals and expected workflow role.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

13. Shared transcripts docs
- Source: https://cursor.com/docs/shared-transcripts
- Why it matters: team sharing and traceability of agent sessions.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

14. Chat history docs
- Source: https://cursor.com/docs/agent/chat/history
- Why it matters: session continuity and retrospective workflows.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

15. Chat export docs
- Source: https://docs.cursor.com/en/agent/chat/export
- Why it matters: artifact transfer and archival workflows.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

## Tooling and Runtime Context

16. Shell configuration docs
- Source: https://docs.cursor.com/configuration/shell
- Why it matters: profile/PATH loading and external CLI interoperability.
- Kit mapping: `83-CURSOR-SETTINGS-CONFIG-FIRST.md`, `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

17. Terminal docs
- Source: https://cursor.com/docs/agent/terminal
- Why it matters: terminal execution behavior and safety modes.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`

18. Terminal Cmd K docs
- Source: https://docs.cursor.com/cmdk/terminal-cmdk
- Why it matters: command generation and reproduction-loop acceleration.
- Kit mapping: `82-CURSOR-DEBUG-MODE-UI-AFFORDANCES.md`

19. Browser tool docs
- Source: https://cursor.com/docs/agent/browser
- Why it matters: UI/debug evidence capture workflows.
- Kit mapping: `79-CURSOR-CONTEXT-FOCUS-30-WAYS.md`, `82-CURSOR-DEBUG-MODE-UI-AFFORDANCES.md`

20. Changelog
- Source: https://www.cursor.com/changelog
- Why it matters: canonical source for product capability changes.
- Kit mapping: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`, `95-READING-LIST.md`
