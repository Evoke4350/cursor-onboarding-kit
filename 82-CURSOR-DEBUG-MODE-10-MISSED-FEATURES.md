# Cursor Debug Mode + 10 Commonly Missed Features
# Cursor Debug + Two UI Screens + 10 Missed Features

## Screen A: Run and Debug panel

What it is:

- Cursor/VS Code **Run and Debug** view.
- Appears when no debug target is active yet.

What to do:

- click `Run and Debug` for quick launch
- or create `.vscode/launch.json` for repeatable profiles

Reference: https://cursor.com/for/debugging

## Screen B: Chat composer with voice/context controls

What it is:

- the chat input bar in Agent mode
- bottom controls for mode/model/context attachments

Visible controls in your screenshot:

- `Agent` mode selector
- model selector (`gpt-5.3-codex` shown)
- globe icon (web context/search surface)
- image icon (attach screenshot/image)
- white circular button with black square (active stop/record state)

Voice note:

- voice-capable controls can appear in this composer flow depending on settings/version/features
- treat voice input as optional acceleration, not a replacement for explicit evidence context

Practical use:

- paste images/PDFs/files to decompose artifacts
- attach visual + textual evidence together for better debugging

References: https://cursor.com/docs/agent/chat/commands, https://cursor.com/docs/en/context/%40-symbols/overview

## Why Cursor debug mode feels “persistent”

- debugger state persists across stepping sessions (breakpoints/watch/call stack)
- terminal/runtime linkage keeps reproduction loops tight
- agent-assisted debugging can iterate until evidence confirms root cause

References: https://cursor.com/for/debugging, https://cursor.com/docs/agent/terminal

---

## 10 missed features (scorecard)

Scoring:

- 5 points: find the feature
- 5 points: explain it with two useful talking points

### 1) `launch.json` profiles — 10/10

- keep per-runtime configs instead of ad hoc launches
- encode env/args once and reuse across team

### 2) Compound debug launches — 10/10

- launch API + worker + frontend together
- ideal for integration bugs with cross-process timing

### 3) Conditional breakpoints — 10/10

- break only on bad state signatures
- cuts noise from high-frequency loops

### 4) Logpoints — 10/10

- inspect runtime state without stopping process
- safer for intermittent race windows

### 5) Debug console evaluation — 10/10

- evaluate expressions at paused frames
- confirm hypothesis before code edits

### 6) Terminal Cmd K for repro commands — 10/10

- generate exact repro/test commands fast
- run immediately with command-submit shortcut

Reference: https://docs.cursor.com/cmdk/terminal-cmdk

### 7) Agent terminal safety modes — 10/10

- choose sandbox/approval posture per risk
- avoid blanket auto-run in sensitive repos

Reference: https://cursor.com/docs/agent/terminal

### 8) Browser tool evidence capture — 10/10

- capture DOM/screenshot/network signals
- feed hard evidence back into fix prompts

Reference: https://cursor.com/docs/agent/browser

### 9) `@` context precision — 10/10

- target files/git/history instead of prompt bloat
- improves relevance and reduces drift

Reference: https://docs.cursor.com/en/context/%40-symbols/overview

### 10) Reusable debug workflows via commands/skills — 10/10

- put recurring debug prompts in `.cursor/commands`
- put procedure logic in `.cursor/skills/**/SKILL.md`

References: https://cursor.com/docs/agent/chat/commands, https://cursor.com/docs/context/skills
## What this screen is

This is the **Run and Debug** view (VS Code-compatible panel inside Cursor).  
It appears when no runnable debug target is active yet.

What each element means:

- **Run and Debug button**: starts a debug session for the current file/runtime.
- **Open a file which can be debugged or run**: prompt to choose a supported entrypoint.
- **create a launch.json file**: define explicit debug profiles in `.vscode/launch.json`.

Reference: [Debugging with Cursor](https://cursor.com/for/debugging)

## Why Cursor debug mode feels “persistent”

The persistence you notice is usually a combination of:

- a long-lived debugger session (breakpoints, watch values, call stack state)
- integrated terminal/runtime linkage (debug target runs with terminal/runtime context)
- agent-assisted debugging loops that can keep gathering evidence until root cause is clear

References: [Run/Debug setup](https://cursor.com/for/debugging), [Terminal Cmd K](https://docs.cursor.com/cmdk/terminal-cmdk), [Agent terminal](https://cursor.com/docs/agent/terminal)

---

## 10 missed features (100-point scorecard)

Scoring model:

- **5 points** for finding the feature
- **5 points** for explaining it with two useful talking points

### 1) `launch.json` profiles and compounds — **10/10**

Talking points:

- You can define multiple debug targets (app, tests, worker) and switch fast.
- Compound configs can launch multiple processes together for integration debugging.

Read more: [Debugging with Cursor](https://cursor.com/for/debugging)

### 2) Attach-to-process debugging — **10/10**

Talking points:

- You can attach a debugger to already-running processes instead of relaunching.
- This is high leverage for flaky state bugs and long startup services.

Read more: [Run/Debug workflows](https://cursor.com/for/debugging)

### 3) Conditional breakpoints and logpoints — **10/10**

Talking points:

- Break only when a condition matches (cuts noise fast).
- Logpoints trace state without pausing execution.

Read more: [Debugging capabilities](https://cursor.com/for/debugging)

### 4) Debug Console expression eval — **10/10**

Talking points:

- Evaluate expressions at paused frames without code edits.
- Validate hypotheses before changing source.

Read more: [Debugging with Cursor](https://cursor.com/for/debugging)

### 5) Watch + Call Stack discipline — **10/10**

Talking points:

- Watch values over time to isolate mutation bugs.
- Walk stack frames to locate bad assumptions, not symptoms.

Read more: [Debugging with Cursor](https://cursor.com/for/debugging)

### 6) Terminal Cmd K as debug command copilot — **10/10**

Talking points:

- Generate exact reproduction commands from natural language.
- Run immediately with `Cmd/Ctrl+Enter` for faster test loops.

Read more: [Terminal Cmd K](https://docs.cursor.com/cmdk/terminal-cmdk)

### 7) Agent terminal safety modes — **10/10**

Talking points:

- Sandbox/approval modes let you debug aggressively with controlled blast radius.
- Allowlist and policy settings reduce repetitive approvals on known-safe commands.

Read more: [Agent terminal](https://cursor.com/docs/agent/terminal), [Permissions](https://cursor.com/docs/cli/reference/permissions)

### 8) Browser tool capture for UI debugging — **10/10**

Talking points:

- Capture visual state, console, and network context directly from browser runs.
- Feed that evidence to the model for DOM/CSS/JS root-cause analysis.

Read more: [Browser tool](https://cursor.com/docs/agent/browser)

### 9) Context precision with `@` symbols — **10/10**

Talking points:

- `@files`, `@git`, `@past chats`, `@web` let you steer context instead of overprompting.
- Better context selection beats bigger prompts for bug isolation quality.

Read more: [@ symbols overview](https://docs.cursor.com/en/context/%40-symbols/overview), [@git](https://docs.cursor.com/context/@-symbols/@-git)

### 10) Reusable debug workflows via commands + skills — **10/10**

Talking points:

- Put recurring debug protocols in `.cursor/commands/*.md`.
- Put repeatable procedures in `.cursor/skills/**/SKILL.md` so teams stop reinventing prompts.

Read more: [Commands](https://cursor.com/docs/agent/chat/commands), [Skills](https://cursor.com/docs/context/skills)

---

## Bottom line

Most teams underuse Cursor debugging because they treat it like an AI chat, not an execution system.  
Execution evidence + scoped context + reusable workflows is where reliability comes from.
