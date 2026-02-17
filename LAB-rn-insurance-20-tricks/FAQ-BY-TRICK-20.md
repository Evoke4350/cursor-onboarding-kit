# FAQ by Trick (20)

Each trick has one practical answer and one technical answer.

## 1) Discussion-Only Alignment
**Q:** Why discuss before editing?  
**No-fluff:** It prevents wrong-file edits and rework.  
**Technical:** This is intent disambiguation before generation; it improves attention allocation.

## 2) Contract Skeleton
**Q:** Why include in/out of scope explicitly?  
**No-fluff:** It blocks silent scope creep.  
**Technical:** Constraints reduce action entropy in decoding.

## 3) One Ticket, One Session
**Q:** Why not batch tasks?  
**No-fluff:** Mixed prompts create mixed results.  
**Technical:** Context blending causes retrieval collision.

## 4) Minimal Blast Radius
**Q:** Why force small diffs?  
**No-fluff:** Easier review, lower regression risk.  
**Technical:** Smaller edit surface reduces cumulative error probability.

## 5) Verification-First
**Q:** Why verify if code looks good?  
**No-fluff:** Looks right is not behaviorally correct.  
**Technical:** LLM confidence is stylistic, not execution-grounded.

## 6) Plan-Then-Build
**Q:** Why plan mode first?  
**No-fluff:** You catch contradictions early.  
**Technical:** Two-pass reasoning separates decomposition from token-expensive generation.

## 7) Team vs Personal Separation
**Q:** Why separate team and personal rules?  
**No-fluff:** Prevents process churn and disputes.  
**Technical:** Reduces instruction conflict in hierarchy.

## 8) Local Exclude for Personal Files
**Q:** Why local excludes?  
**No-fluff:** Avoids git noise and accidental policy leakage.  
**Technical:** Keeps private context outside shared retrieval surfaces.

## 9) Positive Boolean Naming
**Q:** Why avoid negative booleans?  
**No-fluff:** Fewer logic mistakes and clearer conditions.  
**Technical:** Reduces negation depth and parsing ambiguity.

## 10) Explicit Ternary Guards
**Q:** Why avoid truthy/falsy render shortcuts?  
**No-fluff:** Prevents `0`/`\"\"` edge-case rendering bugs.  
**Technical:** Enforces explicit boolean coercion.

## 11) Sub-Agent Role Split
**Q:** Why split explorer/executor/verifier?  
**No-fluff:** Cleaner outputs and fewer accidental edits.  
**Technical:** Role specialization mitigates mode collapse.

## 12) Parallelize Carefully
**Q:** Why limit parallel agents?  
**No-fluff:** Too many agents create merge chaos.  
**Technical:** Coordination overhead can exceed throughput gains.

## 13) Reward Contract
**Q:** Why define reward criteria?  
**No-fluff:** Makes “done” measurable.  
**Technical:** Reward shaping narrows search toward valid end states.

## 14) Commit Grouping by Intent
**Q:** Why curate commit history?  
**No-fluff:** Humans review narrative, not raw generation order.  
**Technical:** Semantic chunking lowers reviewer cognitive load.

## 15) PR Standardization
**Q:** Why fixed PR format?  
**No-fluff:** Speeds review and risk triage.  
**Technical:** Output schemas improve consistency and reduce variance.

## 16) Markdown Lifecycle
**Q:** Why statuses and folders for docs?  
**No-fluff:** Stops sprawl and stale docs.  
**Technical:** Structured metadata improves retrieval relevance.

## 17) Experiments Log
**Q:** Why log keep/modify/drop decisions?  
**No-fluff:** Converts anecdotes into repeatable practice.  
**Technical:** Enables feedback loop tuning.

## 18) Rule Pruning
**Q:** Why remove rules over time?  
**No-fluff:** Over-rules slow and confuse outcomes.  
**Technical:** Shrinks preamble token tax; improves salient instruction ratio.

## 19) Safe External Ingest
**Q:** Why separate raw and normalized artifacts?  
**No-fluff:** Better security and sharing hygiene.  
**Technical:** Isolates secret-bearing context from reusable summaries.

## 20) Adversarial Pass
**Q:** Why challenge a workflow that already works?  
**No-fluff:** To catch failure modes before rollout.  
**Technical:** Negative-case testing reveals hidden brittle assumptions.
