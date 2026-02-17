# Cursor Debug Mode: 10 Missed Features

## Screen quick read

### Run and Debug screen

- This is the Run/Debug panel.
- It appears when no debug target is configured or active.
- `Run and Debug` starts a session.
- `launch.json` gives repeatable debug profiles.

Reference: https://cursor.com/for/debugging

### Chat composer screen

- This is the Agent chat composer.
- Mode selector chooses behavior surface.
- Model selector chooses active model.
- Globe/image controls add web or media context.
- The round control is the active run/stop input control.

References: https://cursor.com/docs/agent/chat/commands, https://docs.cursor.com/en/context/%40-symbols/overview

## Why debug mode feels persistent

- Debug sessions keep runtime state (breakpoints, watch values, call stack).
- The integrated terminal keeps reproduction loops tight.
- Agent debugging loops can keep collecting evidence until root cause is clear.

References: https://cursor.com/for/debugging, https://cursor.com/docs/agent/terminal

---

## 10 commonly missed features

### 1) `launch.json` profiles

- Define stable runtime/debug configs once.
- Reuse across teammates and environments.

### 2) Compound launches

- Start multiple processes together.
- Useful for integration and distributed debugging.

### 3) Conditional breakpoints

- Pause only on suspicious states.
- Reduces noise in hot paths.

### 4) Logpoints

- Trace state without pausing execution.
- Good for timing-sensitive bugs.

### 5) Debug Console evaluation

- Evaluate expressions at paused frames.
- Test hypotheses before editing code.

### 6) Terminal Cmd K for repro commands

- Generate repro/test commands from natural language.
- Tightens reproduce-fix-verify loops.

Reference: https://docs.cursor.com/cmdk/terminal-cmdk

### 7) Agent terminal safety modes

- Tune sandbox and approval posture by risk level.
- Keep destructive operations gated in sensitive repos.

References: https://cursor.com/docs/agent/terminal, https://cursor.com/docs/cli/reference/permissions

### 8) Browser tool evidence capture

- Capture visual, console, and network evidence.
- Feed concrete evidence into fix prompts.

Reference: https://cursor.com/docs/agent/browser

### 9) `@` context targeting

- Target files/git/history precisely.
- Improves relevance and reduces drift.

Reference: https://docs.cursor.com/en/context/%40-symbols/overview

### 10) Commands + skills for reusable debugging workflows

- Put repeatable prompts in `.cursor/commands`.
- Put repeatable procedures in `.cursor/skills/**/SKILL.md`.

References: https://cursor.com/docs/agent/chat/commands, https://cursor.com/docs/context/skills
