# Experienced Engineer Lens (New to Cursor/AI)

This section is for engineers with strong delivery instincts who are new to AI-assisted workflows.

Format:

- relevant question
- direct answer
- short quote from the kit
- where to read next

## 1) "If I already have strong engineering taste, am I expected to give that up?"

No. Keep taste, but encode it as constraints and policy so it scales.

Quote:

> "Keep personal preferences local; promote only proven patterns."

Read next:

- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `## Non-Negotiables (Both Tracks)`
- `10-WORKFLOW-FOUNDATIONS.md` -> `## Team vs Personal Separation`

## 2) "I am used to backpressure mostly from CI/CD. Why add more?"

Because AI increases throughput. Earlier backpressure prevents later rework.

Quote:

> "`AGENTS.md` is backpressure. Treat it as a guardrail, not optional prose."

Read next:

- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `## Non-Negotiables (Both Tracks)`
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md` -> `## Backpressure Framing`

## 3) "Do I need perfect prompts upfront?"

No. Start with discussion, then convert to task contracts.

Quote:

> "You do not need a perfect prompt on first attempt. Use a short discussion loop to refine intent."

Read next:

- `99B-SUBAGENT-PROMPT-LIBRARY.md` -> `## Conversation-To-Execution Bridge`
- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `## Discussion-First Is A Feature, Not A Delay`

## 4) "How do I preserve reviewability when AI generates a lot of code?"

Curate commit history to match human reasoning milestones.

Quote:

> "The commit timeline should reflect the problem-solving trajectory in human-readable steps, not the accidental order of agent edits."

Read next:

- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md` -> `## Core Principle`
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md` -> `## Recommended SOP`

## 5) "Who owns quality when AI is doing a lot of implementation?"

Ownership does not move to the tool.

Quote:

> "**ICs (individual contributors):** own correctness, commit quality, and evidence quality."

Read next:

- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md` -> `## Position Statement (Generic Corporate Voice)`
- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `## Track A: AI-Experienced Engineer`

## 6) "When should I use sub-agents vs just one agent?"

Use orchestration only when the task is cross-domain or parallelizable.

Quote:

> "Use sub-agents when: task spans multiple domains..."

Read next:

- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md` -> `## When Sub-Agents Are Worth It`
- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `### Task Routing Heuristic`

## 7) "How do I avoid letting sub-agents run wild?"

Use measurable reward contracts: objective, boundary, evidence, done condition.

Quote:

> "Treat 'reward' as measurable completion criteria."

Read next:

- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md` -> `## Reward Structure For Agent Success`
- `99B-SUBAGENT-PROMPT-LIBRARY.md` -> `## Universal Task Contract (Use In Every Sub-Agent Prompt)`

## 8) "How do I keep org policy, team rules, and personal setup from colliding?"

Separate the layers and only promote what proves value.

Quote:

> "Promote local patterns to team policy only after repeated positive outcomes."

Read next:

- `10-WORKFLOW-FOUNDATIONS.md` -> `## Team vs Personal Separation`
- `60-PERSONALIZATION-LOCAL-ONLY-CONFIG.md` -> `## Suggested Team Convention`

## 9) "What changes in daily behavior for experienced engineers?"

Less manual typing, more constraint-setting and verification discipline.

Quote:

> "You are no longer typing every line. You are: setting constraints... enforcing quality gates..."

Read next:

- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `## Track B: 3-7 Year Engineer Increasing Speed`
- `10-WORKFLOW-FOUNDATIONS.md` -> `## Core Loop`

## 10) "What is the shortest reliable operating loop?"

Clarify -> constrain -> implement small -> verify -> package.

Quote:

> "1. Clarify task outcome ... 5. Summarize, commit, and open PR"

Read next:

- `10-WORKFLOW-FOUNDATIONS.md` -> `## Core Loop`
- `02-NO-FLUFF-OPERATING-GUIDE.md` -> `## 7-Point Readiness Check`

## 11) "Can I run a second 'supervisor' agent to guide the main one?"

Usually not by default. It can create task alignment drift unless role boundaries are strict.
Also, more memory is not automatically better: extra memory can increase tool and scope confusion if not curated.

Quote:

> "What feels like helpful ambient guidance for humans often adds competing context for language models."

Read next:

- `05-AGENT-OPERATOR-PATTERN.md` -> `## Advanced Caution: "Subconscious Supervisor" Drift`
- `05-AGENT-OPERATOR-PATTERN.md` -> `## How to evaluate whether memory is helping`
- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md` -> `## Role Design`
- `99B-SUBAGENT-PROMPT-LIBRARY.md` -> `## Universal Task Contract (Use In Every Sub-Agent Prompt)`

## Adoption Advice For Senior Engineers

- Start with one live ticket, not a full process migration.
- Preserve your taste by encoding it as reviewable constraints.
- Treat AI speed as raw throughput; backpressure converts it to production quality.
