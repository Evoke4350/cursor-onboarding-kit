# Engineering Principles Template

## Operating order

1. Secure
2. Correct
3. Performant
4. Readable

## Build and review principles

- Prefer small, reviewable changes over large rewrites
- Use explicit naming for booleans and states
- Prefer explicit rendering logic over implicit truthiness in UI
- Treat tests as behavior contracts, not implementation locks
- Log errors with context; never swallow exceptions silently

## AI-era principles

- Assume AI assistance is normal, not exceptional
- Keep human accountability with ICs and reviewers
- Use backpressure (lint, tests, CI, review checklists) as required controls
- Preserve human-readable commit narrative for reviewability
- Track provenance for high-impact AI-generated artifacts

## Definition of done

- Code passes required checks
- User impact validated
- Observability considered
- Documentation updated when behavior changes
