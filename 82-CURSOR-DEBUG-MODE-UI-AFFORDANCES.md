# Cursor Debug Mode: UI Affordances

This guide explains the interface in human-computer interaction terms: signifier, affordance, feedback, and mode.

## Run and Debug panel

Primary affordances:

- **Run and Debug button**: primary action affordance; starts an execution session.
- **Open a file** link: navigation affordance; points to valid executable/debuggable entrypoints.
- **`launch.json` prompt**: configuration affordance; converts one-off runs into stable, repeatable profiles.

Feedback model:

- no active target -> setup guidance is shown
- active debug session -> runtime state is exposed (breakpoints, stack, variables, console)

Reference: https://cursor.com/for/debugging

## Chat composer panel (Agent)

Primary affordances:

- **Mode selector**: interaction-mode affordance (changes agent behavior contract).
- **Model selector**: capability/cost/latency affordance (changes reasoning profile).
- **Globe control**: external context affordance (web-sourced evidence).
- **Image/file control**: multimodal context affordance (visual/document inputs).
- **Run/Stop control**: execution-state affordance (start/interrupt current agent action).

Feedback model:

- active run state changes available controls
- attached context changes what the model can ground on

References: https://cursor.com/docs/agent/chat/commands, https://docs.cursor.com/en/context/%40-symbols/overview

## Why debug mode feels persistent

- state continuity: breakpoints, stack, and watch state survive step-by-step investigation
- environment continuity: integrated terminal keeps reproduction context close to code edits
- evidence continuity: agent loops can keep collecting runtime proof until root cause is validated

References: https://cursor.com/for/debugging, https://cursor.com/docs/agent/terminal

## Operational capabilities to use deliberately

1. `launch.json` profiles for repeatable runs
2. compound launches for multi-process systems
3. conditional breakpoints and logpoints for noisy paths
4. debug console eval for fast hypothesis tests
5. terminal cmd-k for reproduction command generation
6. sandbox/approval policies for safe debug automation
7. browser tool capture for UI evidence
8. `@` context targeting for precision grounding
9. command files for repeatable debug prompts
10. skills for repeatable debug procedures

References:

- https://docs.cursor.com/cmdk/terminal-cmdk
- https://cursor.com/docs/agent/browser
- https://cursor.com/docs/cli/reference/permissions
- https://cursor.com/docs/agent/chat/commands
- https://cursor.com/docs/context/skills
