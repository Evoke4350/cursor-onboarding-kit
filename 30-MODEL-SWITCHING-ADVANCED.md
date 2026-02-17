# Model Switching (Advanced Module)

Use model switching after teams are stable on the baseline workflow.

## Why Switch Models

Different models are better at different tasks. Switching can improve speed, quality, or cost.

## Practical Routing Heuristics

- Fast model: broad search, codebase exploration, simple edits, scaffolding
- Balanced model: most implementation tasks, tests, refactors
- High-capability model: architecture, security review, complex multi-file reasoning

## Decision Triggers

Switch up when:

- output misses nuance after two attempts
- task involves conflicting constraints/tradeoffs
- change spans many modules with hidden coupling

Switch down when:

- task is repetitive or mechanical
- scope is narrow and well-defined
- quick iteration matters more than deep reasoning

## Team Guidance

- Keep defaults simple for new users.
- Teach model switching as an optimization layer, not a dependency.
- Capture before/after outcomes in experiments log.

## Example Prompt Add-On

`Use a faster model for repo exploration first, then switch to a more capable model for final implementation decisions if tradeoffs are non-trivial.`
