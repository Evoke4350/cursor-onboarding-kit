# Cursor Settings (Config-First Engineer Guide)

If you prefer tuning the IDE before usage, this is your baseline map.

## 1) Models and routing

Set model defaults and provider keys first.

- pick default model strategy (fast vs deep)
- configure BYOK provider keys where needed
- keep team defaults documented, not implied

References: https://cursor.com/docs/settings/api-keys, https://cursor.com/docs/api

Config-first tip:

- the model/settings panel is where you enable provider keys, available models, and default routing posture for your team style

## 2) Agent execution posture

Choose risk posture before enabling automation.

- approval mode (ask/allowlist/auto patterns)
- sandbox/network boundaries
- command allow/deny policy

References: https://cursor.com/docs/agent/security, https://cursor.com/docs/cli/reference/permissions, https://cursor.com/docs/agent/terminal

## 3) MCP and integrations

Treat MCP like production integration, not a toy.

- enable only needed servers/tools
- require ownership and auditability
- restrict by workspace/team policy

Reference: https://cursor.com/docs/integrations

Empathy note:

- configuration-first engineers usually want this documented as a baseline profile
- keep a team "safe default" profile and a personal "experimental" profile

## 3.5) Shell profile and PATH loading

If tools are available in native terminal but missing in Cursor, check shell/profile loading first.

- verify integrated shell profile is correct
- verify `PATH` and exported environment variables in the active shell
- restart terminal/editor after profile changes

Reference: https://docs.cursor.com/configuration/shell

## 4) Commands, skills, hooks

These are your configuration primitives for workflow behavior.

- `.cursor/commands/*.md` = reusable entrypoints
- `.cursor/skills/**/SKILL.md` = reusable procedures
- `.cursor/hooks.json` = policy checks and event automation

References: https://cursor.com/docs/agent/chat/commands, https://cursor.com/docs/context/skills, https://cursor.com/docs/agent/hooks

## 5) Review and quality

Enable review surfaces early.

- Bugbot for PR backpressure
- repo-level guidance files for consistency

References: https://cursor.com/docs/bugbot, https://cursor.com/docs/integrations/github

## 6) Config-first rollout pattern

1. set safety posture
2. set model/provider posture
3. add two shared commands
4. add one shared skill
5. add one low-friction hook
6. review after one week and prune

## 7) Built-in voice and multimodal context

Cursor chat supports rich context attachment workflows (images/files/web context controls in composer).  
Use these for evidence capture, not prompt decoration.

Related references: https://docs.cursor.com/en/context/%40-symbols/overview, https://cursor.com/docs/agent/browser

Practical screen signals:

- mode selector (Agent/Ask/Plan styles)
- model selector
- web/image attachment controls
- stop/record action control during active runs
