# Verification: Oracles For Agents

Agents are hypothesis generators; verification is the truth machine.

If you want AI to write more of your app, you need sharper oracles:

- property-based tests for invariants ("lemmas")
- E2E tests for user flows
- visual regression for UX

This lab is intentionally buggy. That is the point: you practice building better oracles until the agent has something real to grind against.

## Stairway (Practical)

1. **Static**: lint + typecheck (cheap, catches obvious lies)
2. **Unit tests**: fast, local correctness (but can be a false oracle if you encode the bug)
3. **Property tests**: invariants across many generated cases (shrinks to a minimal counterexample)
4. **E2E**: real browser + real server + real UI state transitions
5. **Visual regression**: screenshots as a guardrail for "fixed the logic, broke the UX"

## What Exists In This Lab

- Characterization tests (what it does today): `src/__tests__/`
- Verification upgrades (what it should do): `src/__property_tests__/`, `e2e/`

Characterization tests are useful when inheriting a mess. Verification upgrades are what make agents safer.

## Quick Start

From the repo root:

```bash
cd LAB-scheduling-demo
```

Expected results today:

- `tsx --test src/__tests__/*.ts`: PASS (characterization baseline)
- `npm run test:e2e`: PASS (smoke)
- `npm run test:pbt`: FAIL (spec oracle, by design)
- `npm run test:spec`: FAIL (spec oracle, by design)
- `npm run test:visual`: FAIL until you generate a baseline

Run the characterization suite:

```bash
tsx --test src/__tests__/*.ts
```

Optional: install verification tooling:

```bash
npm install
```

## Property-Based Tests (fast-check)

Run:

```bash
npm run test:pbt
```

These tests are meant to fail at first. They encode the intended invariants (the oracle), not the current broken behavior.

## End-To-End Tests (Playwright)

Install browsers once:

```bash
npx playwright install
```

Run smoke E2E:

```bash
npm run test:e2e
```

Run spec E2E (expected to fail until bugs are fixed):

```bash
npm run test:spec
```

## Visual Regression (Playwright Screenshots)

Generate/update baseline snapshots:

```bash
npm run test:visual -- --update-snapshots
```

Then run:

```bash
npm run test:visual
```

## Bombadil (Antithesis): Property-Based UI Testing

Playwright is a scalpel. You write a script, it follows the script.

Bombadil is a fuzzing rig for UIs:

- you describe properties (linear temporal logic) instead of hand-writing one path
- it generates action sequences to explore states you didn't think to test
- when it finds a violation, it gives you a reproducer trace (JSONL + screenshots)

This is why it can be a game changer: it turns "UI correctness" from a handful of curated paths into a state-space search with a real counterexample.

Think of it like this:

- Playwright answers: "Does this exact path work?"
- Bombadil answers: "What path breaks my rule?"

Run it against this lab:

1. Start the lab server: `npm run dev` (or `tsx ui/server.ts`)
2. Optional: deep-link to a scenario: `http://localhost:4173/?scenario=coverage-inflation`
3. Create a Bombadil spec (TypeScript module exporting properties)
   - Optional: install TS types: `npm install -D @antithesishq/bombadil`
4. Run:

```bash
# Get the CLI from Bombadil releases (or use nix).
bombadil test --exit-on-violation --output-path=/tmp/bombadil \
  http://localhost:4173/?scenario=coverage-inflation \
  your-spec.ts
```

Minimal lab-flavored property (spec oracle):

```ts
import { always, extract } from "@antithesishq/bombadil";

function findCardValue(document, labelText) {
  for (const card of document.querySelectorAll("#cards .card")) {
    const label = card.querySelector(".card-label")?.textContent?.trim();
    if (label === labelText) return card.querySelector(".card-value")?.textContent ?? "";
  }
  return "";
}

const teamCoverageText = extract((state) => findCardValue(state.document, "Team Coverage"));
const teamCoveragePct = extract((state) => {
  const raw = teamCoverageText.current.replace("%", "").trim();
  const n = Number(raw);
  return Number.isFinite(n) ? n : null;
});

export const coverage_never_above_100 = always(
  () => teamCoveragePct.current === null || teamCoveragePct.current <= 100,
);
```

Reference:

- Bombadil repo: https://github.com/antithesishq/bombadil

## Reference Paper

- Property-based testing: Climbing the stairway to verification (University of Melbourne / Trustworthy Systems): https://trustworthy.systems/publications/papers/Chen_ROSKHK_22.pdf
