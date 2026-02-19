# Agent Operator Pattern (Prompt Chain + Workflow Productization)

This note captures the working pattern used to build this onboarding kit.
It is not about reproducing identical output. It is about reproducing **reliable trajectory**.

## State-First Mindset

Before any prompt chain: **where is state?**

| Question | If No Answer | Result |
|----------|--------------|--------|
| Where's current work tracked? | In my head | Lost on interruption |
| Can I resume after crash? | Start over | Not a system |
| Why did I make that decision? | ¯\\_(ツ)_/¯ | Unexplainable |
| What changed since yesterday? | Check git manually | Slow context recovery |

**Every prompt chain should produce state artifacts, not just output.**

## Minimum Viable Chain (Default)

If you are new to this repo, start here and ignore the rest for now.

1. **Discussion**: no edits yet, triage likely bugs and prioritize.
2. **Contract**: write in-scope, out-of-scope, done condition.
3. **Scoped Fix**: implement one bug cluster with minimal blast radius.
4. **Verification**: produce explicit expected vs actual evidence.
5. **Delivery**: group commits by intent and draft PR summary.

Most teams get most of the value from this five-step chain.  
Everything below is optional depth and optimization.

## Fast Path (When You Don't Need Ceremony)

- If it's a tiny change and you already understand the bug: compress to **scope+fix**, then **verify+package**.
- If it's ambiguous: do the discussion step first. No heroics.
- If it touches multiple files: keep the contract step, or you will end up with an "AI diff salad."

## Category Theory Lens (Optional Nerd Stuff)

- **Objects**: ticket states (untriaged, scoped, changed, verified, packaged).
- **Morphisms**: prompts that move you between states (discussion, contract, fix, verify, delivery).
- **Composition**: chaining only works if each step emits a usable artifact (plan, contract, evidence).
- **Functors**: the same loop across tools/harnesses (Cursor, CLI, Copilot) if capabilities match.
- **Natural transformations**: prompt tweaks per tool that preserve the intent of the loop.
- **Products**: combining independent context streams (code × logs × UX) without mixing them into soup.
- **Limits**: "minimal blast radius that still satisfies contract + tests" (constraints intersect, change shrinks).
- **Adjunctions**: spec <-> implementation. Without a contract, the agent happily wanders in the wrong direction.
- **Monads**: the effectful loop (read files, edit code, run commands) with explicit "done" as the bind condition.

Where it falls apart is always the same: missing oracle (no tests), hidden state (memory soup), or untyped outputs ("looks good").

## What This Pattern Optimizes For

- high-throughput ideation via dictation/conversation
- constrained execution into useful artifacts
- reusable prompt chains with repeatable checkpoints
- quality via backpressure and evidence, not vibes

## Core Philosophy

1. Conversation is the capture layer.
2. Constraints are the control layer.
3. Completion criteria are the reliability layer.
4. Review artifacts are the transfer layer.

Or, in one line:

**Natural language in, bounded engineering out.**

---

## The Signature Pattern

Your recurring ask shape is:

`Read this context -> make a medium-sized change -> keep scope tight -> show evidence.`

With optional variants:

- “do not edit yet, discuss first”
- “turn this into a shareable rule/doc”
- “add FAQs for likely objections”
- “add adversarial take”
- “evaluate this and specify the next recommended change”

This pattern is strong because it combines:

- human-level intent speed
- machine-level synthesis speed
- explicit completion criteria

---

## Expanded Prompt Chain (Optional)

Use this when you need the full productization path.

## Step 0 - Intent seed (dictation)

Goal: capture raw intent quickly.

Template:

`This project needs <outcome>. Keep it <constraints>. Make it usable for <audience>.`

## Step 1 - Scope lock

Goal: prevent expansion drift.

Template:

`In scope: <x>. Out of scope: <y>. If you need to cross scope, ask first.`

## Step 2 - Persona packet

Goal: shape output for specific learner/operator archetypes.

Template:

`Assume audience includes: <persona A>, <persona B>. Explain for both.`

Example persona classes you used:

- senior engineer new to AI tooling
- consultant needing portable, client-agnostic assets
- instructor needing ready-to-run workshop content

## Step 3 - Completion contract

Goal: define “done” in measurable terms.

Template:

`Success means: <artifact list> + <quality checks> + <reviewability outcomes>.`

## Step 4 - Artifact fan-out

Goal: create modular docs, not one giant document.

Pattern:

- foundation docs
- advanced appendices
- templates
- lab package
- evidence maps
- adversarial critique

## Step 5 - Evaluate and reflect

Goal: stress-test usefulness.

Pattern:

- evaluate prompts/tricks against solution key
- identify high-leverage prompts
- record gaps and immediate changes

## Step 6 - Meta reflection

Goal: tune the formula after each structural change (for example explicit -> semantic prompting).

Pattern:

- adjust evaluation focus
- add new metrics (for example steering efficiency)
- update operating guidance

---

## Why This Works (LLM Mechanics, Practical)

- **Constraint salience**: explicit boundaries outrank implied intent.
- **Mode control**: “no edits yet” switches behavior from generation to analysis.
- **Completion shaping**: done criteria improve completion reliability.
- **Variance reduction**: output schemas and narrow asks improve repeatability.
- **Drift recovery**: steering prompts reduce waste from non-deterministic turns.

---

## Advanced Caution: "Subconscious Supervisor" Drift

This project tested a second "watcher" agent that tried to guide a primary agent across turns.
This felt like it should help, but often created alignment drift instead.

What went wrong:

- two instruction streams started competing
- goals drifted from concrete task output toward meta-guidance loops
- memory became noisy, and the worker agent lost local task clarity
- the system looked thoughtful but produced less reliable execution
- tool calls became less predictable (wrong tool choice, repeated exploration, occasional dead-end loops)
- project-level confusion increased ("what are we solving right now?" became ambiguous across turns)
- eval quality dropped because it became hard to isolate whether memory improved or degraded outcomes

Why this happens:

- what helps humans ("ambient coaching") can confuse language models
- LLMs optimize against active prompt/context, not implied shared intent
- extra supervisory narration can dilute the highest-salience task contract
- memory adds context mass, and not all context mass is useful context
- when memory is broad but task contract is narrow, alignment drift is statistically more likely

Memory-specific lesson:

- adding memory is not automatically a quality upgrade
- memory can be a force multiplier or a confusion multiplier depending on curation
- if you cannot measure impact, treat memory as an experiment, not infrastructure

Use this pattern only when:

- roles are strictly separated (research vs implementation vs review)
- each agent has a measurable task contract
- handoff artifacts are explicit and short

Recovery playbook when drift appears:

1. pause all secondary guidance
2. restate single objective, scope, and done criteria
3. reduce to one executing agent
4. reintroduce parallel agents only with explicit role contracts
5. strip non-essential memory and rerun a minimal baseline task

How to evaluate whether memory is helping:

1. pick 2-3 recurring task types (bugfix, refactor, doc synthesis)
2. run A/B passes (with memory vs minimal memory)
3. compare: task completion rate, tool-error rate, scope drift incidents, rework volume
4. keep only memory blocks that show repeatable benefit

Rule of thumb:

**More agent layers do not equal more intelligence.  
Clarity beats clever orchestration.**

Forward-looking exploration (not yet settled):

- git-managed memory block state for agents is promising, but still experimental in practice
- idea: version memory in small, reviewable blocks with explicit owners and rollback points
- open question: does the operational overhead outperform simpler prompt contracts?
- current stance: continue exploring, but do not assume positive engineering utility without evidence

Adjacent tooling experiments:

- exploring agent-first workflows across tools, including `jj` (Jujutsu), to improve change isolation
- potential upside: cleaner state transitions and easier rollback when agent runs branch or drift
- caution: new tooling can add cognitive load; evaluate outcomes, not novelty

---

## Advanced Caution: Pre/Trans Fallacy in System Building

When building agent systems, avoid confusing **pre-rational** (early developmental) stages with **trans-rational** (integrated/transcendent) stages.

**The fallacy:** A system that *looks* complete but isn't *actually* integrated.

Example mistake pattern:
- "We built the taint tracker" → but Cursor hooks aren't calling it
- "We have a 5 Cs pipeline" → but it creates plain markdown, not executable docs
- "We documented the pattern" → but nobody follows it in practice

**Diagnostic questions:**

1. **Is it wired?** Does component A actually call component B, or just exist near it?
2. **Is it tested end-to-end?** Can a new operator follow the docs and get working results?
3. **Is it used?** Do real sessions actually invoke this system?

**Pre-stage signals:**
- Code exists but no integration tests
- Docs describe ideal state, not current state
- Manual steps required between components
- "Works on my machine" without reproducibility

**Trans-stage signals:**
- Automatic handoff between components
- Errors caught at integration boundaries
- New operators succeed without expert help
- Evidence trail from intent to delivery

**Recovery pattern:**

When you discover pre/trans confusion:

1. Stop adding new features
2. Map what actually connects vs what should connect
3. Write integration tests for the gaps
4. Only add new capability after current layer works end-to-end

Rule of thumb:

**A working prototype beats an elegant architecture that isn't wired.**

---

## Personal Style Layer

Your high-performing style signals:

- direct, minimal prose
- rapid iterative asks
- explicit preference for practical over performative output
- willingness to tighten after each artifact
- insistence on teachability and transferability

What to preserve when others copy this:

- **the loop**, not the literal wording
- **the control points**, not the exact adjectives

---

## FAQ Pattern

Every mature module eventually got:

1. no-fluff answer
2. technical/jargon answer
3. failure mode
4. recovery prompt

This is a strong transfer design because it handles both:

- tactical implementers
- conceptual skeptics

---

## Predictable Output Recipe

Use this compact recipe to get predictable outcomes without robotic prompting:

1. Start semantic and conversational.
2. Add explicit scope boundaries.
3. Add completion contract.
4. Require evidence output.
5. Run adversarial pass.
6. Capture deltas in experiment log.

Deterministic enough for team use does **not** require perfectly deterministic model output.
It requires stable **workflow constraints**.

---

## Copy/Paste Blocks

## A) Discussion-first block

`No edits yet. Restate the problem, scope, risks, and done criteria in 5 bullets.`

## B) Scoped execution block

`Make this medium-sized change with minimal blast radius. Do not touch unrelated areas.`

## C) Completion block

`You are done only if: <criteria>. Return pass/fail checklist.`

## D) Drift recovery block

`Pause. You drifted scope. Re-ground in current files and return a revised plan only.`

## E) Reviewability block

`Now package this for humans: commit grouping, PR summary, test plan, risks/rollback.`

---

## Organizational Rollout Advice

When sharing this pattern internally:

- present it as an operator system, not a magic prompt
- keep a “minimum viable chain” for first-time users
- add advanced layers (subagents, model switching, adversarial reviews) gradually
- track outcomes with simple evidence, not vanity metrics

---

## Minimum Viable Chain (for newcomers)

1. Discuss-first
2. Scope lock
3. Medium-sized execution
4. Completion checklist
5. Human review package

If a team can execute this chain consistently, they are ready for advanced orchestration.

---

## Self-Reinforcement Loop (Practical, Guardrailed)

Goal:

- improve workflow quality over time, not chase perfect prompts

Loop:

1. run normal work (ship code/docs with standard guardrails)
2. collect artifacts (chat history, transcripts, commits, review notes)
3. analyze patterns (what correlated with good outcomes vs rework)
4. promote only proven deltas to local `AGENTS.local.md`
5. promote stable deltas to team `AGENTS.md` or rules
6. re-measure on next cycle

Optional tool support:

- DevSQL can correlate conversation patterns with commit/review outcomes:
  https://github.com/douglance/devsql
- treat query outputs as signals, not truth

Reflection prompts that work well:

- `Notice any frustration patterns in the last 7 sessions and map each one to a concrete mitigation.`
- `Find the 5 prompts that most often preceded clean commits; summarize common structure.`

Guardrails:

- cap loop cadence (for example weekly), not every session
- cap active experiments (1-2 at a time)
- require outcome evidence before policy promotion
- reject perfection loops; optimize for reliable delivery
