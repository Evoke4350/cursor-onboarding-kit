# React Native Insurance Bug Lab (20 Tricks)

This lab is a compact, instructor-ready exercise for teaching prompt quality and workflow discipline in Cursor.

Training note: the bugs are intentionally planted. The point is to practice the habit (triage -> scope -> fix -> verify -> package), not to pretend this is the chaos of production.

## Lab Shape

- 3 source files with intentionally small, realistic bugs
- 20 trick-aligned prompts (mono-semantic, contract-first)
- deterministic solution key
- instructor runbook
- FAQ and LLM mechanics appendix

## Scenario

You are supporting a React Native insurance quote flow for a fictional company. Recent changes introduced subtle bugs in eligibility logic, UI rendering behavior, and telemetry semantics.

## Source Files

- `src/PolicyQuoteScreen.tsx`
- `src/eligibility.ts`
- `src/telemetry.ts`

## Bug Clusters

1. Falsy rendering and UI drift
2. Negative boolean naming and boundary logic bugs
3. Telemetry event contract and semantics bugs

## Instructor Goal

Teach engineers to move from conversation -> constraints -> execution -> verification, with predictable outcomes and review-ready artifacts.
