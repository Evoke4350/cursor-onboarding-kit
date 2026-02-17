# No-Fluff Cursor Operating Guide

If someone has 10 minutes, this is the first file they should read.

## 1) Non-Negotiables

- Define outcome before asking an agent to code.
- Keep scope tight; one objective per pass.
- Require evidence (`what changed`, `what ran`, `what failed`, `what remains`).
- Keep quality gates on (lint, typecheck, tests, CI, review).
- Keep personal preferences local until they prove repeat value.

`AGENTS.md` is a guardrail file, not optional prose.

---

## 2) 10-Minute Map (What / Why / Where)

### Operating basics

- **Core loop**: simple repeatable workflow from prompt to verification.  
  **Why**: reduces drift and rework.  
  **Read**: `10-WORKFLOW-FOUNDATIONS.md`
- **Prompt patterns**: reusable prompt shapes.  
  **Why**: faster starts, fewer ambiguous asks.  
  **Read**: `20-PROMPT-PATTERNS.md`
- **No-fluff Q&A for experienced engineers**: practical objections and answers.  
  **Why**: speeds adoption for senior ICs.  
  **Read**: `03-EXPERIENCED-ENGINEER-LENS-QA.md`

### Instruction files and local personalization

- **Instruction stack (`AGENTS`, `CLAUDE`, Cursor rules, Copilot instructions)**.  
  **Why**: shared team policy + tool compatibility across agent systems.  
  **Read**: `35-INSTRUCTION-FILES-ADVANCED.md`
- **Starter pack templates for instruction files**.  
  **Why**: copy/paste baseline for new repos.  
  **Read**: `40-TEMPLATES/INSTRUCTION-STARTER-PACK/README.md`
- **How context is picked up** (what loads automatically vs conditionally).  
  **Why**: prevents false assumptions about "always loaded" docs.  
  **Read**: `40-TEMPLATES/INSTRUCTION-STARTER-PACK/CONTEXT-PICKUP-GUIDE.md`
- **Local-only config (`*.local.md`, git excludes)**.  
  **Why**: personal tuning without repo noise.  
  **Read**: `60-PERSONALIZATION-LOCAL-ONLY-CONFIG.md`

### Templates and documentation operations

- **Template clusters** (briefing, handoff, runbook, eval, RFC, risk, release readout).  
  **Why**: consistent artifacts, easier handoff and review.  
  **Read**: `40-TEMPLATES/README-GENERIC-TEMPLATE-CLUSTERS.md`
- **Markdown ops** (lifecycle, naming, archiving).  
  **Why**: prevents blackboard sprawl and stale docs.  
  **Read**: `50-MARKDOWN-OPS.md`
- **Glossary and frontmatter guide**.  
  **Why**: shared language, less confusion in docs.  
  **Read**: `90-GLOSSARY-AND-FRONTMATTER.md`

### Cursor runtime and UI (when you need the controls)

- **Cursor Tab** (suggestions, popover, snooze, disable-for-markdown).  
  **Why**: fast local edits without chat overhead; tune noise mid-task.  
  **Read**: `07-CURSOR-TAB-KEY.md`
- **Advanced Cursor surfaces** (CLI, Web/Cloud Agent, Bugbot, Git review, status bar, layout presets, audio notifications).  
  **Why**: stacked-diff review, in-flight steering, layout modes, optional TTS/bell.  
  **Read**: `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`
- **Hooks, skills, commands, subagents**.  
  **Why**: repeatable workflows, guardrails, and parallel task shapes.  
  **Read**: `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`
- **Context focus** (30 ways to target what the model sees).  
  **Why**: less drift, better outputs.  
  **Read**: `79-CURSOR-CONTEXT-FOCUS-30-WAYS.md`
- **Config-first setup** (models, approval, MCP, shell).  
  **Why**: if you tune IDE before use.  
  **Read**: `83-CURSOR-SETTINGS-CONFIG-FIRST.md`
- **Debug and shortcuts**.  
  **Why**: run/debug affordances and keyboard habits.  
  **Read**: `82-CURSOR-DEBUG-MODE-UI-AFFORDANCES.md`, `81-CURSOR-SHORTCUTS-ADVANCED-FOLLOW-UP.md`

### Governance and review

- **Commit history curation** (AI-assisted change, stacked diff, review-ready narrative).  
  **Why**: readable history and rollback precision.  
  **Read**: `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`
- **Evidence map and adversarial take**.  
  **Why**: align controls with risk; stress-test before scaling.  
  **Read**: `96-EXTERNAL-EVIDENCE-MAP-2026.md`, `97-DEVILS-ADVOCATE-ADVERSARIAL-TAKE.md`

### Advanced layers (add when needed)

- **Model switching**.  
  **Why**: cost/perf trade-offs by task type.  
  **Read**: `30-MODEL-SWITCHING-ADVANCED.md`
- **Sub-agent orchestration and prompt contracts**.  
  **Why**: only useful when tasks are cross-domain/parallelizable.  
  **Read**: `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`, `99B-SUBAGENT-PROMPT-LIBRARY.md`
- **Operator pattern (real-world workflow + memory drift cautions)**.  
  **Why**: shows what worked, what failed, and how to recover.  
  **Read**: `05-AGENT-OPERATOR-PATTERN.md`
- **Other harness compatibility (optional)**.  
  **Why**: helps mixed-tool teams without bloating Cursor-first onboarding.  
  **Read**: `76-OTHER-AGENT-HARNESSES-ADVANCED.md`

**Full navigation**: see `INDEX.md` for adoption sequence and all doc links.

---

## 3) Two Working Tracks

### Track A: already fluent with AI tooling

1. Lock objective/scope/done criteria in one prompt.
2. Parallelize only independent workstreams.
3. Implement in small slices.
4. Run hard gates (`lint`, `typecheck`, tests, CI).
5. Package for humans (risk + rollback + test evidence).

### Track B: strong engineer, newer to AI workflow

1. Ask for one medium change.
2. Require risks/assumptions before edits.
3. Verify every pass.
4. Commit only with evidence.
5. Keep personal style in local files until proven.

Prompt skeleton:

```md
Implement <change> in <scope>.
Constraints: minimal blast radius, follow existing patterns.
Verification: run lint/typecheck/relevant tests.
Return: files changed, why, risk notes, rollback.
```

---

## 4) Practical Warnings

- More agent layers do not automatically improve outcomes.
- More memory does not automatically improve quality.
- If quality drops, simplify first: single agent, tight scope, explicit done criteria.
- Treat new orchestration/memory patterns as experiments until measured.

---

## 5) Trigger Phrase Library (Mode + Behavior Control)

Use these phrases to steer behavior quickly during live sessions.

### High-reliability triggers (explicit)

- `Let's discuss first.` -> analysis/discussion before edits
- `No edits yet. Restate scope, risks, and done criteria.` -> planning behavior
- `Switch to plan mode.` -> explicit planning mode request
- `Use the question tool and ask me one decision question.` -> structured question capture
- `Debug this with runtime evidence, not code reading only.` -> debug workflow request

### Planning-mode semantic triggers (usually work)

- `I need a plan before code.`
- `Map the approach and trade-offs first.`
- `Break this into phases with risks and rollback.`
- `Show me options A/B/C with recommendation first.`
- `Create a plan and wait for approval before implementing.`

### Debug-mode semantic triggers (usually work)

- `This is a regression; treat this as a debugging task.`
- `Reproduce first, then hypothesize, then instrument with logs.`
- `Use runtime evidence to find root cause.`
- `Intermittent failure: collect traces and narrow conditions.`
- `Do not refactor yet, isolate the failing path first.`

### Idiomatic/metaphor triggers (advanced)

You can use idioms and analogies to encode system structure quickly.
Models often latch onto these well when roles and boundaries are explicit.

Examples:

- `Use a city metaphor: mayor = orchestrator, rigs = executors, cargo = tasks, checkpoints = validation gates.`
- `Treat this as a logistics network: intake -> routing -> execution -> quality gate -> release.`
- `Give me one analogy for this architecture, then map each analogy role to a concrete file or function.`

Why it helps:

- compresses complex structure into memorable language
- improves coordination prompts for multi-agent workflows
- makes planning artifacts easier for humans to review

Guardrails:

- always map metaphor roles back to real code boundaries
- keep done criteria concrete (tests, outputs, files changed)
- if metaphor causes drift, drop it and revert to direct contracts

### Reliability note

- Explicit instructions are more reliable than implied intent.
- Semantic phrasing helps, but behavior can vary by model/version.
- If the agent drifts, re-issue a short explicit control line.

---

## 6) 7-Point Readiness Check

- [ ] Can define objective/scope/done clearly
- [ ] Uses verification commands by default
- [ ] Keeps backpressure gates enabled
- [ ] Knows when to use single-agent vs sub-agents
- [ ] Separates team policy from personal preferences
- [ ] Produces concise, evidence-based PR notes
- [ ] Can explain risk and rollback before merge

---

## 7) Junior First-Week Plan (Ship One PR)

Use this if you know TypeScript/React but this is your first team role with Cursor.

### Day 1

- Read `00-START-HERE.md` and this guide.
- Read `10-WORKFLOW-FOUNDATIONS.md`.
- Skim `07-CURSOR-TAB-KEY.md` (Tab vs chat split; use Tab for small local edits).
- Copy starter instruction files from `40-TEMPLATES/INSTRUCTION-STARTER-PACK/README.md`.

### Day 2

- Pick one low-risk ticket.
- Run discussion-first prompt: restate scope, non-goals, risks, done criteria.
- Make one medium change only.

### Day 3

- Run verification and collect evidence (`lint`, `typecheck`, relevant tests).
- Write PR notes from evidence, not memory.

### Day 4

- Get review feedback and apply targeted fixes.
- Keep scope locked; no side quests.

### Day 5

- Merge one clean PR with rollback notes.
- Write a short experiment note: what prompt pattern worked, what drifted, what to keep.

Rule of thumb:

- If confused, simplify.
- One ticket, one objective, one evidence-backed PR.

**Next**: Use `INDEX.md` for the full adoption sequence and to find any doc by topic.
