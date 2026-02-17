# Workflow Chain Template (Conversation -> Code -> PR)

## 1) Frame

- Objective: <what success looks like>
- Scope: <in/out>
- Constraints: <non-negotiables>
- Reward: <what "good" earns>

## 2) Discuss

- Clarify assumptions
- Identify trade-offs
- Lock a small first slice

## 3) Execute

- Read related files
- Implement smallest safe change
- Validate with tests/lint/typecheck

## 4) Evaluate

- Compare output to objective
- Record misses and deltas
- Decide iterate vs ship

## 5) Package

- Curate commit history for humans
- Produce PR narrative and test plan
- Attach risks and follow-ups

## Prompt skeleton

```
Context: <ticket + relevant module>
Objective: <concrete outcome>
In scope: <list>
Out of scope: <list>
Constraints: <style/testing/runtime rules>
Deliverable: <code + test + PR notes>
Reward: <why this matters>
```
