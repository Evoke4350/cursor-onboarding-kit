# Ghost Codebase Exercise (This Repo)

This exercise is for `LAB-scheduling-demo` only.

Treat this codebase like an inherited system with unclear intent, mixed quality, and production pressure.

## Objective

Land one scoped, PR-ready change in this repo while preserving baseline behavior outside your scope.

## Non-Negotiables

1. Do not edit tests just to force green.
2. Keep intentional lab bugs unless your selected ticket explicitly targets them.
3. One behavior theme per PR.
4. No opportunistic refactor drift.

## 75-Minute Workflow

### 1) Recon (10 min)

- Read `README.md`, `AGENTS.md`, and `WEBMCP-AGENT-QUICKSTART.md`.
- Build a quick map:
  - scheduling logic: `src/rotation.ts`, `src/availability.ts`, `src/notifications.ts`, `src/scheduleReport.ts`
  - UI/API surface: `ui/server.ts`, `ui/public/*`
  - tests: `src/__tests__/*.ts`

### 2) Baseline (10 min)

Run and record:

```bash
cd cursor-onboarding-kit/LAB-scheduling-demo
tsx --test src/__tests__/*.ts
AGENT_DEV_KEY=lab-agent-key tsx ui/server.ts
```

Optional API sanity check:

```bash
curl -s http://localhost:4173/api/scenarios
```

### 3) Scope Contract (10 min)

Choose exactly one ticket/problem theme (example: `SCHED-2419` or `SCHED-2427`) and write:

- in-scope behavior
- out-of-scope behavior
- acceptance checks

If you cannot state scope in 3 sentences, scope is too broad.

### 4) Reproduce (15 min)

- Reproduce the bug/issue with a command, endpoint call, or failing assertion.
- Capture the current output as baseline evidence.

### 5) Patch (20 min)

- Implement the smallest coherent fix.
- Touch only files needed for the chosen behavior.

### 6) Verify + Handoff (10 min)

- Run targeted checks, then `tsx --test src/__tests__/*.ts`.
- Write a short handoff:
  - what changed
  - why this scope only
  - exact verification commands
  - known risks/gaps

## Deliverable Standard

A reviewer should understand in one pass:

1. problem
2. fix
3. evidence
4. boundaries

## Failure Modes

- mixing multiple ticket themes in one PR
- editing unrelated modules "while you're in there"
- claiming verification without command evidence
