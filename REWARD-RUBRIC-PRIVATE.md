# Private Reward Rubric: $2,000 KPI Self-Evaluation

**Total budget:** 2,000 points ($1 = 1 point)  
**Evaluation cadence:** per talk, per release, per engagement  
**Rule:** outcomes are the scoreboard; process is supporting evidence.

---

## Scoring Model

| Layer | Points | What It Means |
|------|-------:|---------------|
| **A. Outcome Impact** | 1400 | Did we ship better results for the team or client? |
| **B. Execution Discipline** | 600 | Did we use a reliable workflow to get those results? |

### Score Caps (Anti-Gaming)

- If there is no shipped outcome and no before/after evidence, final score is capped at **900**.
- If process artifacts are excellent but adoption is zero after 14 days, final score is capped at **1200**.
- If a change increases escaped defects, the score for Outcome Quality cannot exceed **50%**.

---

## Layer A: Outcome Impact (1,400 pts)

### O1: Cycle Time and Throughput (400 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Median lead time improved on comparable tickets | 150 | Before/after lead time snapshot |
| Review cycles reduced (fewer back-and-forth rounds) | 125 | PR history (round count delta) |
| Meaningful throughput gain without quality drop | 125 | Tickets/week and quality gates together |

### O2: Quality and Reliability (400 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Escaped defect rate improved | 150 | Bug backlog trend or incident log |
| Rollback/hotfix risk reduced | 125 | Rollback/hotfix frequency trend |
| Edge-case coverage materially improved | 125 | New tests, scenario tables, or QA coverage notes |

### O3: Adoption and Habit Formation (350 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| At least 2 engineers reused the chain on real work | 125 | PRs/issues from distinct contributors |
| Repeat usage after first success | 125 | Same contributors using chain again |
| New-user path remained lightweight (no framework fatigue) | 100 | Onboarding feedback or session notes |

### O4: Stakeholder Value (250 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Client/team confidence improved (faster approvals, less skepticism) | 100 | Review comments or stakeholder notes |
| Demo/talk converted into concrete next step | 100 | Pilot request, follow-up meeting, or tracked action item |
| Work mapped to a business-relevant outcome | 50 | Explicit objective/result mapping |

---

## Layer B: Execution Discipline (600 pts)

### D1: Discovery and Contract Quality (150 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Discussion-first triage before edits | 50 | Transcript |
| In-scope and out-of-scope were explicit | 50 | Contract artifact |
| Done criteria were measurable | 50 | Testable acceptance criteria |

### D2: Scoped Implementation (200 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Minimal blast radius maintained | 100 | Focused diff |
| Refactors preserved behavior | 50 | Rename map/equivalence notes |
| AI drift recovered quickly | 50 | Correction prompt and resulting diff |

### D3: Verification Rigor (150 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Scenario or test table with pass/fail | 75 | Verification artifact |
| Uncertainty called out explicitly | 25 | Risk/uncertainty note |
| Verification tied to fixed code state | 50 | Explicit statement and checks |

### D4: Delivery Hygiene (100 pts)

| Criterion | Points | Evidence |
|-----------|-------:|----------|
| Commits grouped by intent | 40 | Commit history or grouping plan |
| PR summary/test plan/rollback present | 40 | PR artifact |
| DCO and repo contribution rules followed | 20 | Commit trailers and checklist |

---

## Talk Bonus Multipliers

| Event | Multiplier | Condition |
|-------|:----------:|-----------|
| Thursday Cursor RN | 1.1x | Live fix path stayed on the five-step chain |
| Friday Enterprise Guild | 1.1x | Audience asks for pilot or adoption follow-up |
| Both talks + one post-talk pilot | 1.2x combined | Demo led to concrete implementation next step |

---

## Scoring Worksheet

```text
Event: _______________
Date: _______________

O1 Cycle/Throughput: ___/400
O2 Quality:          ___/400
O3 Adoption:         ___/350
O4 Stakeholder:      ___/250
                     --------
Layer A subtotal:    ___/1400

D1 Discovery:        ___/150
D2 Implementation:   ___/200
D3 Verification:     ___/150
D4 Delivery:         ___/100
                     --------
Layer B subtotal:    ___/600

Raw total:           ___/2000
Multiplier:          ___x
Final score:         ___
```

---

## Benchmarks

| Score Range | Rating | Interpretation |
|-------------|--------|----------------|
| 1600-2000+ | Exceptional | Outcomes improved and process remained lightweight |
| 1200-1599 | Strong | Solid gains, but adoption or quality deltas still uneven |
| 800-1199 | Mixed | Process happened, outcomes unclear or weak |
| < 800 | Incomplete | Little measurable impact or no shipped result |
