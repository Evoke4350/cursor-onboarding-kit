# External Evidence Map (2026)

This document captures external references on Cursor/AI-tooling friction and maps each issue to this onboarding kit.

Scope:

- individual engineers
- consultancies
- small/medium company contexts
- product/vendor and community reports

## Reference Coverage Snapshot

- Target references: 20
- Collected references: 20

## High-Level Problem Themes

1. Context drift and poor scope control
2. Team chaos from inconsistent personal usage
3. Rule bloat and token/latency pressure
4. Reliability and performance instability
5. Review/provenance gaps (transcripts/history)
6. Speed illusion vs measured outcomes
7. Need for org-specific operational tuning

---

## References (20) + Direct Quotes + Kit Mapping

## 1) Grab Engineering (company engineering blog)

URL: `https://engineering.grab.com/cursor-at-grab-adoption-and-impact`

Quote:

> "Integrating Cursor effectively at Grab required custom tooling."

Implication: default setup is insufficient at organizational scale.

Kit mapping:

- `35-INSTRUCTION-FILES-ADVANCED.md`
- `75-GITHUB-COPILOT-CONFIG-ADVANCED.md`
- `10-WORKFLOW-FOUNDATIONS.md`

## 2) Dan Podina (Software Architecture Consultant)

URL: `https://gqlteam.com/blog/make-cursor-work/`

Quote:

> "I spent months thinking I was bad at prompting. Turns out, I was using Cursor wrong."

Implication: process quality often matters more than model quality.

Kit mapping:

- `02-NO-FLUFF-OPERATING-GUIDE.md`
- `99B-SUBAGENT-PROMPT-LIBRARY.md`

## 3) Dan Podina (Software Architecture Consultant)

URL: `https://gqlteam.com/blog/make-cursor-work/`

Quote:

> "Define contracts — goals, non-goals, constraints, acceptance"

Implication: constraints reduce agent drift.

Kit mapping:

- `99B-SUBAGENT-PROMPT-LIBRARY.md`
- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`

## 4) Ridma Gamage (Technical Lead | AWS Solution Architect)

URL: `https://medium.com/@ridmag/using-cursor-ai-in-team-workflows-without-causing-chaos-c3d0da2add43`

Quote:

> "Cursor works great solo. Teams are where things get tricky."

Implication: shared operating standards are required.

Kit mapping:

- `10-WORKFLOW-FOUNDATIONS.md`
- `60-PERSONALIZATION-LOCAL-ONLY-CONFIG.md`

## 5) Ridma Gamage (Technical Lead | AWS Solution Architect)

URL: `https://medium.com/@ridmag/how-to-set-cursor-ai-rules-so-it-actually-works-the-way-you-expect-b392130e21d6`

Quote:

> "Most frustration with Cursor AI doesn’t come from the model. It comes from missing or badly written rules."

Implication: governance and rule craft are core capabilities.

Kit mapping:

- `20-PROMPT-PATTERNS.md`
- `90-GLOSSARY-AND-FRONTMATTER.md`
- `50-MARKDOWN-OPS.md`

## 6) Peakvance (individual practitioner writeup)

URL: `https://medium.com/@peakvance/guide-to-cursor-rules-engineering-context-speed-and-the-token-tax-16c0560a686a`

Quote:

> "there is a fine line between a well-guided AI and a bloated, expensive context window."

Implication: over-instruction becomes a performance and quality tax.

Kit mapping:

- `50-MARKDOWN-OPS.md`
- `99B-SUBAGENT-PROMPT-LIBRARY.md`

## 7) Peakvance

URL: `https://medium.com/@peakvance/guide-to-cursor-rules-engineering-context-speed-and-the-token-tax-16c0560a686a`

Quote:

> "Every word in your rules is a token."

Implication: concise rule design is operationally relevant.

Kit mapping:

- `90-GLOSSARY-AND-FRONTMATTER.md`
- `35-INSTRUCTION-FILES-ADVANCED.md`

## 8) Andrew Larsen (CTO)

URL: `https://andrew-larse514.medium.com/9-lessons-learned-from-using-cursor-coding-agents-in-production-a494dc9020d7`

Quote:

> "they’re not ‘good enough’ out of the box."

Implication: onboarding and SOP are required, not optional.

Kit mapping:

- `00-START-HERE.md`
- `10-WORKFLOW-FOUNDATIONS.md`

## 9) Andrew Larsen (CTO)

URL: `https://andrew-larse514.medium.com/9-lessons-learned-from-using-cursor-coding-agents-in-production-a494dc9020d7`

Quote:

> "research -> plan -> implement"

Implication: staged execution improves quality.

Kit mapping:

- `30-MODEL-SWITCHING-ADVANCED.md`
- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`

## 10) Divy Yadav (individual technical writer)

URL: `https://medium.com/@yadavdivy296/cursor-quietly-fixed-the-biggest-problem-of-ai-agents-in-2026-dynamic-context-discovery-explained-1adcac48c828`

Quote:

> "The longer you work, the dumber it gets."

Implication: unmanaged context accumulation degrades outcomes.

Kit mapping:

- `99B-SUBAGENT-PROMPT-LIBRARY.md`
- `50-MARKDOWN-OPS.md`

## 11) Vishnu KG (Technical Lead)

URL: `https://medium.com/%40vishnukgcherupuzha/the-rise-of-agentic-ides-cursor-windsurf-and-the-death-of-the-copilot-era-9ea7cc277375`

Quote:

> "The '20% Faster' Illusion"

Implication: subjective speed must be validated by objective outcomes.

Kit mapping:

- `02-NO-FLUFF-OPERATING-GUIDE.md`
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`

## 12) Leon Consulting (consultancy blog)

URL: `https://leonstaff.com/blogs/cursor-vs-vscode-efficiency-audit/`

Quote:

> "QA Fatigue"

Implication: AI output volume can exceed review capacity.

Kit mapping:

- `02-NO-FLUFF-OPERATING-GUIDE.md`
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`

## 13) Likhon (consultancy/operator blog)

URL: `https://brlikhon.engineer/blog/cowork-vs-cursor-vs-claude-code-the-ultimate-ai-coding-agent-battle-for-2026`

Quote:

> "which failure mode will cost you the least?"

Implication: decision quality should be failure-mode-centric.

Kit mapping:

- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`
- `02-NO-FLUFF-OPERATING-GUIDE.md`

## 14) Chris Dunlop (consultancy publication)

URL: `https://medium.com/realworld-ai-use-cases/why-has-it-been-so-hard-to-build-a-cursor-or-claude-code-in-other-industries-a667ed89c31c`

Quote:

> "why don’t we see a Cursor for construction, a Claude Code for law, a GitHub Copilot for logistics?"

Implication: transferability across domains is non-trivial.

Kit mapping:

- `80-SAMPLE-PROJECT-LAB.md`
- `03-EXPERIENCED-ENGINEER-LENS-QA.md`

## 15) Cursor Forum Bug Report (user report, Feb 2026)

URL: `https://forum.cursor.com/t/latest-version-hangs-on-planning-moves-and-waiting-for-terminal-script-responses/151725`

Quote:

> "hangs showing 'planning moves'"

Implication: orchestration reliability has practical operational limits.

Kit mapping:

- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`
- `02-NO-FLUFF-OPERATING-GUIDE.md`

## 16) Cursor Forum Bug Report (user report, Feb 2026)

URL: `https://forum.cursor.com/t/date-provided-in-user-info-shows-2025-in-2026/150925`

Quote:

> "gives 2025 as the year instead of 2026"

Implication: baseline trust requires verification even for simple outputs.

Kit mapping:

- `02-NO-FLUFF-OPERATING-GUIDE.md` (`verification defaults`)

## 17) Cursor Forum Feedback (user report, 2026)

URL: `https://forum.cursor.com/t/release-notes-what-has-been-changed-fixed/150892`

Quote:

> "Please start providing release notes for each release you do."

Implication: change transparency matters for operational confidence.

Kit mapping:

- `50-MARKDOWN-OPS.md` (internal transparency hygiene)
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`

## 18) Cursor Forum Bug Report (user report, Jan 2026)

URL: `https://forum.cursor.com/t/transcripts-no-longer-exported-in-full/150214`

Quote:

> "Transcripts should be just that, transcripts."

Implication: provenance and auditability are workflow-critical.

Kit mapping:

- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`
- `50-MARKDOWN-OPS.md`

## 19) Cursor Forum Thread (user issue cluster, 2026)

URL: `https://forum.cursor.com/t/cursor-is-not-usable-after-the-last-update/150165/56`

Quote:

> "out of memory crashes in less than an hour."

Implication: rollout safety and fallback process are necessary.

Kit mapping:

- `00-START-HERE.md` (phased adoption)
- `02-NO-FLUFF-OPERATING-GUIDE.md` (failure recovery)

## 20) Cursor Product Sources (official)

URLs:

- `https://cursor.com/changelog/cli-jan-16-2026`
- `https://cursor.com/blog/plan-mode`

Quotes:

> "Plan mode in CLI"  
> "Most new features at Cursor now begin with Agent writing a plan."

Implication: planning-first workflows are now product-supported behavior.

Kit mapping:

- `00-START-HERE.md`
- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`

---

## Engineer-Background Bonus References

Examples where the source includes explicit role/background context:

- Dan Podina page subtitle: "Software Architecture Consultant"
- Ridma Gamage profile: "Technical Lead | AWS Solution Architect"
- Andrew Larsen profile: "CTO @ compozelabs.com"
- Chris Dunlop publication bio: consultancy context and client exposure
