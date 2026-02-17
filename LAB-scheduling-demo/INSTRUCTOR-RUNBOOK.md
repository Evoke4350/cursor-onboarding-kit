# Instructor Runbook: Team Scheduling Demo

Single-operator demo optimized for a live client audience.

## Session Goals

- Show that prompt structure controls outcome quality
- Demonstrate discussion → constraints → execution → verification
- Land the core claim: agents generate hypotheses; verification is the truth machine
- Progressive disclosure: start simple, build to multi-file coordination
- Produce review-ready artifacts from the same workflow

## Timeboxes

| Format | Duration | Prompts | Best for |
|--------|----------|---------|----------|
| Fast demo | 25 min | 1-5 (Act 1 + top of Act 2) | Executive overview |
| Standard | 45 min | 1-10 (Acts 1-2 + start of Act 3) | Engineering team intro |
| Full | 70 min | 1-13 (all 3 acts + adversarial) | Deep workshop |

## Suggested Sequence

### Act 1: "Let's Talk First" (5-8 min)

| Step | Prompt | Concept | Time |
|------|--------|---------|------|
| 1 | Anti-pattern: naive ask | Contrast setup | 1 min |
| 2 | Prompt 1: Discussion-first triage | Discussion-first | 2 min |
| 3 | Prompt 2: Contract definition | Completion contract | 2 min |

### Act 2: "Surgical Fixes" (10-15 min)

| Step | Prompt | Concept | Time |
|------|--------|---------|------|
| 4 | Prompt 3: Scoped rotation fix | Scoped execution | 3 min |
| 5 | Prompt 4: Multi-file boolean rename | Minimal blast radius, refactor | 4 min |
| 6 | Prompt 5: Verification table | Evidence-based completion | 3 min |
| 7 | Prompt 6: Explorer audit of notifications | Explorer mode (no edits) | 3 min |

### Act 3: "Coordination and Delivery" (10-15 min)

| Step | Prompt | Concept | Time |
|------|--------|---------|------|
| 8 | Prompt 7: Parallel suitability check | Parallel coordination | 1 min |
| 9 | Prompt 8: Plan-then-build (timezone bug) | Plan mode, hard bug | 4 min |
| 10 | Prompt 9: Fix scheduleReport bugs | Scoped execution | 3 min |
| 11 | Prompt 10: Commit curation | Commit narrative | 2 min |
| 12 | Prompt 11: PR draft | PR discipline | 2 min |
| 13 | Prompt 12: Adversarial review | Devil's advocate | 2 min |
| 14 | Prompt 13: Cleanup and reflection | Meta-reflection | 1 min |

## Demo Beats ("Wow" Moments)

### Beat A — Intent clarity beats verbosity (Act 1, Step 1→2)
Run the naive "fix all bugs" prompt first. Let the model produce an unfocused multi-file diff. Then run Prompt 1 (discussion-first). The audience sees: same codebase, dramatically different starting quality.

**Instructor line:** "Same AI, same code. The only difference is how I asked."

### Beat B — Multi-file refactor under control (Act 2, Step 5)
Run Prompt 4 (rename isNotAvailable → isAvailable across 4 files). The model traces every usage, inverts every conditional, preserves behavior. Show the diff is clean and behavior-preserving.

**Instructor line:** "This is a refactor humans hate doing by hand. The AI handles the tedium; the constraint keeps it safe."

### Beat C — Plan before you cut (Act 3, Step 9)
For the timezone offset bug (hardest one), show the model producing a plan, then waiting for approval. Approve step 1 only. Show the incremental, controlled execution.

**Instructor line:** "This is the hard bug. I don't trust myself to eyeball timezone math — and I shouldn't trust the AI to freestyle it either. Plan mode gives us a shared checkpoint."

### Beat D — Adversarial review catches what humans skip (Act 3, Step 13)
Run Prompt 12. The model produces concrete production failure risks with stress tests and owners — not generic risk poetry.

**Instructor line:** "Most teams skip this. The AI won't get tired or embarrassed to ask hard questions."

### Beat E — Oracles beat vibes (Optional, 3 min)
If someone asks "why does any of this matter?", show the punchline: the agent makes hypotheses, and the oracles decide what's real.

```bash
cd LAB-scheduling-demo
npm install
npm run test:e2e   # passes (smoke)
npm run test:spec  # fails (Team Coverage 200%)
npm run test:pbt   # fails (shrunk counterexample)
```

Optional upgrade: Bombadil is property-based testing for web UIs (generative action sequences + temporal-logic properties). See `VERIFICATION.md`.

**Instructor line:** "Same AI. Same code. Different outcome because the oracle is sharp."

## Opening Script

> "I just inherited this scheduling module from a departing engineer. It 'mostly works' but QA flagged some edge cases. Let me show you how to use AI to triage and fix this — and more importantly, how the structure of what you ask determines whether you get a clean PR or a mess."

## Anti-Pattern Demo (Step 1)

Open all 5 source files. Type into chat:

> "Fix all the bugs in this scheduling code."

Let the model run. Point out:
- It touches all files at once
- No prioritization
- No scope boundary
- The diff would be unreviewable
- This is what most people do their first week with AI tools

Then say: "Now let me show you how to actually do this." → Move to Prompt 1.

## Troubleshooting Matrix

| Symptom | Likely cause | Fix |
|---------|-------------|-----|
| Model edits when told not to | Scope too broad, "no edits" not salient | Move "no edits yet" to first line of prompt |
| Wrong fix for timezone bug | Missing plan step | Re-run with Prompt 8 (plan-then-build) |
| Model rewrites entire file | No blast-radius constraint | Add "minimal blast radius, keep existing structure" |
| Verification table too shallow | No edge cases specified | Add specific scenarios to prompt |
| Overly verbose PR draft | No length constraint | Add "summary under 5 bullets, test plan under 5 items" |
| Model conflates clusters | Mixed objectives in single prompt | Split into one-cluster prompts |

## Audience Q&A Prep

**"Doesn't this take longer than just fixing it?"**
"For one bug, maybe. For 12 bugs across 5 files that need to land as a reviewable PR? The structure pays for itself in the second file."

**"What if the model gets it wrong?"**
"That's exactly why we use verification tables and executable checks. The workflow assumes the model will be wrong sometimes — the oracles catch it."

**"Can junior engineers use this?"**
"The structured prompts are training wheels. Juniors get better outcomes immediately. Over time they internalize the patterns and write their own prompts."

**"Is this Cursor-specific?"**
"The prompt patterns work in any AI coding tool. Cursor gives you the best workflow around them — inline chat, multi-file context, agent mode. But the disciplines are portable."

## Instructor Checklist

- [ ] Used discussion-first at least once
- [ ] Showed anti-pattern vs structured prompt contrast
- [ ] Used completion contract at least once
- [ ] Showed multi-file coordination
- [ ] Produced verification evidence before claiming done
- [ ] Ran adversarial pass
