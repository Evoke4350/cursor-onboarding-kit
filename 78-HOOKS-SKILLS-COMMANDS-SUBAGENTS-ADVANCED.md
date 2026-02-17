# Hooks, Skills, Commands, and Subagents (Senior Guide)

If you operationalize these four surfaces, Cursor becomes a workflow platform, not just an editor.

## One-line mental model

- Hooks = guardrails and telemetry.
- Commands = repeatable entry points.
- Skills = reusable procedures.
- Subagents = parallel specialists with isolated context.

## Scope and storage model

Use this as your default:

- **Global local-only**: `~/.cursor/**` for personal experiments and machine-specific behavior.
- **Project committed**: `.cursor/**` for team-shared workflows.
- **No built-in per-directory hook config**: use path checks inside hooks to scope behavior by subtree.

References: [Hooks](https://cursor.com/docs/agent/hooks), [Commands](https://cursor.com/docs/agent/chat/commands), [Skills](https://cursor.com/docs/context/skills), [Subagents](https://cursor.com/docs/context/subagents)

## Hooks: how to use them without pain

Use hooks for enforcement and capture, not long business logic.

Good hook use:

- block destructive shell commands in sensitive repos
- enforce minimal commit hygiene before risky actions
- log agent actions for audit/replay
- tag sessions with repo/task metadata

Bad hook use:

- heavy network calls in pre-action hooks
- brittle scripts that fail closed on routine tasks
- duplicate checks that already exist in CI

Project pattern:

1. start with global local hooks for experimentation
2. graduate stable hooks into project `.cursor/hooks.json`
3. keep hook scripts in `scripts/hooks/` and code-review them

References: [Hooks](https://cursor.com/docs/agent/hooks), [Third-party hooks](https://cursor.com/docs/agent/third-party-hooks), [Hooks partners blog](https://cursor.com/blog/hooks-partners)

## Commands: shared workflows in one file

Commands are the easiest way to standardize team behavior quickly.

Patterns that work:

- `/weekly-insights` for evidence-based reflection
- `/ship-pr` for commit + PR checklist flow
- `/risk-review` for findings-first review output

Commit command files when:

- behavior should be shared by the team
- output schema needs consistency

Keep command files local-only when:

- they encode personal shortcuts
- they depend on machine-local tools/secrets

Reference: [Commands](https://cursor.com/docs/agent/chat/commands)

## Skills: procedural knowledge, not policy dumps

A good skill is narrow, testable, and reusable.

Good skill examples:

- process PR comments into patch + verification plan
- parse large JSON into summarized risk report
- run migration checklist for one framework

Skill design rules:

- one purpose per skill
- clear trigger description
- concrete steps
- strict output shape
- explicit stop condition

Reference: [Skills](https://cursor.com/docs/context/skills)

## Subagents: parallelize the right things

Subagents are best for decomposition, not delegation theater.

Use subagents for:

- broad codebase exploration
- parallel hypothesis checks
- isolated draft generation with strict merge criteria

Avoid subagents for:

- tiny edits
- tasks with unresolved requirements
- anything where handoff quality is weak

Reference: [Subagents](https://cursor.com/docs/context/subagents), [Cursor 2.4 changelog](https://cursor.com/changelog/2-4)

## Repo pattern: what gets committed

Commit:

- `.cursor/commands/*.md`
- `.cursor/skills/**/SKILL.md`
- stable `.cursor/hooks.json` and hook scripts

Do not commit:

- personal/global config in `~/.cursor/**`
- machine-specific secrets and tokens
- unstable experiments without owner and rollback

## Suggested rollout

Week 1:

- add 2 commands, 1 skill, 0-1 hook

Week 2:

- add one subagent workflow for a recurring heavy task

Week 3:

- keep what reduced rework, remove what increased friction
