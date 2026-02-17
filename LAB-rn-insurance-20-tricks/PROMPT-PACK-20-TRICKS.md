# Prompt Pack: 20 Tricks, How People Actually Ask

These prompts intentionally sound like real-world requests: ambiguous, sometimes semantically off, and still recoverable with good steering.

## 1) Start with talk, not edits

I think there are a few bugs in this quote flow, maybe auth-ish, maybe not.  
Don’t edit anything yet. Tell me what’s likely broken, bucket it into three clusters, and give me a clean fix order in five bullets max.  
You win if I can approve without asking follow-ups.

## 2) Write the contract before touching code

I need this logic cleaned up but let’s not code yet.  
Give me a strict contract: what changes, what does not, and what done looks like.  
If this is broader than one module, say so explicitly.

## 3) One ticket, one room

Treat this like one ticket only: eligibility correctness.  
Keep the fix scoped to the smallest relevant logic area. Don’t wander into UI or tracking unless it is required.  
If you must cross scope, ask first.

## 4) Minimal blast radius, surgical fix

There’s a user-facing eligibility label that looks logically inverted.  
Fix it with the smallest diff possible. No cleanup pass, no style crusade, no “while I’m here” edits.  
Give me two lines: before intent and after intent.

## 5) Prove it, don’t vibe it

Assume the fix is in.  
Now show me evidence with a four-row scenario table: expected, actual, pass/fail.  
If there’s uncertainty, call it out directly.

## 6) Plan first, then build in two moves

We’ve probably got one UI semantics bug and one telemetry semantics bug.  
Give me a two-step plan with risk notes and wait for approval before coding.  
Coding before approval is a fail.

## 7) Shared style only, no personal flourishes

Fix factual bugs only.  
Don’t rewrite for taste, naming aesthetics, or personal style unless that’s the actual defect.  
Tell me what you intentionally left alone.

## 8) Stay in local context, no mythology

Use only what you can actually see here.  
No invented services, no invented data contracts, no made-up architecture from prior projects.  
Start with: “Grounded in files:” and list what you used.

## 9) Rename negative booleans into positive intent

Where booleans read like double negatives, simplify them to positive intent naming.  
Keep behavior stable unless behavior is already wrong.  
Return a rename map and one behavior note.

## 10) Make render logic explicit

There’s a conditional render that probably relies on truthy/falsy shortcuts.  
Make it explicit and safe for weird values like `0` and empty strings.  
No UI redesign.

## 11) Explorer mode: findings only

Run this like an audit pass.  
No edits. Just list mismatches in event payload semantics: field, expected, actual, severity.  
If you start patching, you missed the assignment.

## 12) Should this be parallel work?

Can UI bugfix and telemetry bugfix run in parallel safely, or do they share coupling?  
Binary answer first, then two concrete reasons.  
No edits.

## 13) Reward contract fix for telemetry

Apply telemetry fixes only if you can satisfy this reward contract:
- eligibility flag is not inverted
- numeric premium fields stay numeric
- field names remain stable

Return pass/fail checklist against those three.

## 14) Curate the story, not just the diff

Assume bug clusters are fixed.  
Propose up to four commits that a human reviewer can read like a story, not a token explosion.  
No git commands, just grouping.

## 15) Write a reviewer-friendly PR in one pass

Draft PR text with:
- Summary (3 bullets)
- Test plan (checklist)
- Risks and rollback

Keep it readable in under one minute.

## 16) Add lifecycle metadata like a grown-up

Create a run note with frontmatter and status metadata so this can be indexed later.  
Then add three bullets on what changed and why.

## 17) Log one experiment with a real decision

Write one experiment entry from this run:
hypothesis, result, keep/modify/drop, one follow-up.  
No decision = incomplete.

## 18) Run a rule-pruning simulation

Design a pruning experiment:
baseline -> prune -> rerun -> compare.  
Make the comparison measurable, not vibes.

## 19) Show safe external context ingest

Give me a secure checklist for pulling external context (tickets/design docs) into this flow.  
Use env-var token style only, no real creds.  
Include one generic curl example.

## 20) End with a devil’s-advocate pass

Before rollout, list top five failure risks, one stress test per risk, and owner.  
No generic risk poetry.

---

## Steering Micro-Prompts (when user request is semantically off)

Use these to recover from fuzzy asks like “find auth bug” when it’s actually a UI/logic issue.

- `Quick sanity check: I can inspect auth-adjacent areas, but current symptoms look UI/eligibility-related. Do you want broad scan or narrow fix first?`
- `I can start with a broad bug sweep, or stay tight on this visible behavior. Pick one: breadth or depth.`
- `This request sounds cross-domain. I recommend: discuss first, then contract, then scoped edit.`

## Technical Notes (short)

- Fuzzy human prompts are normal; correction loops are part of good prompting.
- Determinism comes from constraints + reward checks, not from perfect first phrasing.
- “Grounded in files” reduces hallucinated context transfer.
- Verification prompts move model behavior from generation to evaluation.
