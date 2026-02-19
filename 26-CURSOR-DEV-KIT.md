# Cursor Dev Kit: Agent-Native Workflows for CI, Review, and Shipping

**References:**
- [Cursor Documentation](https://docs.cursor.com/)
- [PatrickJS/awesome-cursorrules](https://github.com/PatrickJS/awesome-cursorrules)
- [25-AGENTS-MD-EFFECTIVENESS.md](25-AGENTS-MD-EFFECTIVENESS.md) - What the research says
- [78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md](78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md) - Surfaces overview

---

## The Core Thesis

> **When agents get stuck, change the codebase rather than the prompt. Surprising failures reveal what the agent doesn't understand.**

The Cursor AI ecosystem has matured significantly. The old approach—adding more instructions to context files—doesn't work. Research shows LLM-generated context files make agents *worse* (-3% success rate, +20% cost).

The new approach is structural: **optimize the codebase for agent comprehension, not agent instruction**.

---

## Agent Psychology: Counterintuitive Techniques

### The "Change Codebase, Not Prompt" Rule

When an agent repeatedly fails on a specific pattern:

1. **Don't** add more rules explaining what to do
2. **Do** restructure the code so the right path is obvious

Example: If an agent keeps missing error handling in async flows, don't write a rule about error handling. Instead:
- Add explicit error types that make the pattern visible
- Create helper functions with clear signatures
- Use the type system to encode expectations

The agent's confusion is diagnostic. It tells you where your codebase has friction.

### The "Step 3" Trick (Controlled Misdirection)

Counterintuitive but effective: if an agent struggles with step 2, tell it to do step 3. The agent often completes step 2 in the process.

```
Agent stuck on: "Write tests for this module"
Tell it: "Deploy this to staging" (requires tests first)
Result: Agent writes tests as a prerequisite
```

This works because:
- Agents reason forward from instructions
- Changing the target reframes the problem
- The "lie" is a feature—controlled misdirection for better outcomes

### Greenfield Projects: Ship Fast, Fix Later

For greenfield projects with no production constraints:

1. **Any change is acceptable** — optimize for agent success, not architectural purity
2. **Move fast** — velocity matters more than perfection
3. **Fix later** — accumulated technical debt can be addressed when patterns emerge

The goal isn't perfect code. The goal is working code that the agent can iterate on.

---

## The Four Surfaces: Hooks, Commands, Skills, Subagents

| Surface | Purpose | Scope |
|---------|---------|-------|
| **Hooks** | Guardrails and telemetry | Global local-only → project committed |
| **Commands** | Repeatable entry points | Shared workflows |
| **Skills** | Reusable procedures | Narrow, testable, composable |
| **Subagents** | Parallel specialists | Isolated context, discrete tasks |

### One-line mental model:
- Hooks = guardrails and telemetry
- Commands = repeatable entry points
- Skills = reusable procedures
- Subagents = parallel specialists with isolated context

---

## CI Workflows: Agent-Native Patterns

### Skill: `looping-on-ci`

The agent runs CI in a loop until green, fixing failures iteratively.

```markdown
# SKILL.md for looping-on-ci

## Trigger
CI is failing and the fix is non-obvious.

## Steps
1. Run CI: `npm test` or equivalent
2. Parse failure output
3. Identify root cause
4. Apply minimal fix
5. Re-run CI
6. Repeat until green or 5 attempts exhausted

## Stop Condition
- CI passes
- 5 attempts without progress
- New error type requires human decision

## Output
- Summary of changes made
- Final CI status
- Any remaining issues requiring human attention
```

### Skill: `fix-ci`

Targeted fix for a specific CI failure.

```markdown
# SKILL.md for fix-ci

## Trigger
A specific test or lint error needs fixing.

## Input
- Error message or test name
- File path (if known)

## Steps
1. Locate the failing test/lint rule
2. Understand what it's checking
3. Identify the code causing the failure
4. Apply the minimal fix
5. Verify fix locally

## Output
- Description of the fix
- Files changed
- Verification steps taken
```

### Skill: `run-smoke-tests`

Quick validation that core functionality works.

```markdown
# SKILL.md for run-smoke-tests

## Trigger
Before/after significant changes, validate system health.

## Steps
1. Run smoke test suite: `npm run smoke` or equivalent
2. If failures:
   - Categorize: environment, data, code
   - For code failures: apply fix-ci skill
   - For environment/data: document and escalate
3. Report results

## Output
- Pass/fail status
- Failure categorization
- Recommended next steps
```

### Skill: `review-and-ship`

Complete PR workflow from review to merge.

```markdown
# SKILL.md for review-and-ship

## Trigger
A PR is ready for final review and merge.

## Steps
1. Checkout PR branch
2. Run full CI suite
3. Review diff for:
   - Test coverage
   - Documentation updates
   - Breaking changes
4. Address any review comments
5. Squash/rebase if required
6. Merge when green

## Output
- Merge status
- Summary of changes
- Any post-merge actions needed
```

---

## Subagent: `ci-watcher`

A specialized agent that monitors CI and triggers follow-up actions.

### Configuration

```json
{
  "name": "ci-watcher",
  "description": "Monitors CI pipelines and triggers remediation",
  "model": "claude-sonnet-4-5",
  "tools": ["bash", "read", "write"],
  "trigger": "ci-failed",
  "context": ".cursor/subagents/ci-watcher.md"
}
```

### Use Cases

1. **Auto-triage** — Categorize failures by type (flake, real bug, environment)
2. **Auto-fix** — Apply known fix patterns for common failures
3. **Escalation** — Notify humans when fixes exceed complexity threshold
4. **Flake detection** — Track test stability over time

### Workflow

```
CI Failed → ci-watcher triggered
    ↓
Parse failure → Categorize
    ↓
┌─────────────────┬─────────────────┬─────────────────┐
│ Known fix       │ Unknown fix     │ Environmental   │
│ pattern         │ pattern         │ issue           │
├─────────────────┼─────────────────┼─────────────────┤
│ Apply fix       │ Create issue    │ Notify infra    │
│ Re-run CI       │ Tag for triage  │ Suggest retry   │
└─────────────────┴─────────────────┴─────────────────┘
```

---

## Rules: Project-Specific Conventions

### Rule: `typescript-exhaustive-switch`

Ensures all switch cases are handled in TypeScript.

```yaml
# .cursor/rules/typescript-exhaustive-switch.mdc
---
description: Enforce exhaustive switch statements in TypeScript
globs: **/*.ts, **/*.tsx
---

## TypeScript Exhaustive Switch

Always use exhaustive switch checking in TypeScript:

```typescript
// Preferred pattern
type Status = 'pending' | 'active' | 'completed';

function getStatusLabel(status: Status): string {
  switch (status) {
    case 'pending': return 'Pending';
    case 'active': return 'Active';
    case 'completed': return 'Completed';
    default:
      const _exhaustive: never = status;
      throw new Error(`Unknown status: ${status}`);
  }
}
```

The `default: never` pattern ensures TypeScript will error if a new case is added to the union but not to the switch.
```

### Rule: `no-inline-imports`

Prevents inline imports that break tree-shaking and code navigation.

```yaml
# .cursor/rules/no-inline-imports.mdc
---
description: Prevent inline dynamic imports in favor of top-level imports
globs: **/*.ts, **/*.tsx, **/*.js, **/*.jsx
---

## No Inline Imports

Avoid inline imports except when code-splitting:

```typescript
// Bad - inline import for no reason
function formatDate(date: Date) {
  const { format } = require('date-fns'); // Don't do this
  return format(date, 'yyyy-MM-dd');
}

// Good - top-level import
import { format } from 'date-fns';

function formatDate(date: Date) {
  return format(date, 'yyyy-MM-dd');
}

// Acceptable - intentional code-splitting
async function loadHeavyComponent() {
  const { HeavyComponent } = await import('./HeavyComponent');
  return HeavyComponent;
}
```
```

---

## Cursor Dev Kit Plugin Architecture

The Cursor Dev Kit is a plugin ecosystem for extending Cursor's capabilities.

### Plugin Structure

```
.cursor/
├── commands/           # Shared workflows
│   ├── ship-pr.md
│   └── weekly-review.md
├── skills/             # Reusable procedures
│   ├── looping-on-ci/
│   │   └── SKILL.md
│   ├── fix-ci/
│   │   └── SKILL.md
│   └── review-and-ship/
│       └── SKILL.md
├── rules/              # Project conventions
│   ├── typescript-exhaustive-switch.mdc
│   └── no-inline-imports.mdc
├── subagents/          # Specialized agents
│   ├── ci-watcher.md
│   └── code-reviewer.md
├── hooks.json          # Guardrails
└── settings.json       # Project configuration
```

### Plugin Manifest

```json
{
  "name": "cursor-dev-kit",
  "version": "1.0.0",
  "description": "Internal workflows for CI, code review, and shipping",
  "commands": [
    { "name": "ship-pr", "file": "commands/ship-pr.md" },
    { "name": "fix-ci", "file": "commands/fix-ci.md" }
  ],
  "skills": [
    { "name": "looping-on-ci", "path": "skills/looping-on-ci" },
    { "name": "fix-ci", "path": "skills/fix-ci" },
    { "name": "review-and-ship", "path": "skills/review-and-ship" }
  ],
  "subagents": [
    { "name": "ci-watcher", "file": "subagents/ci-watcher.md" }
  ],
  "rules": [
    { "name": "typescript-exhaustive-switch", "file": "rules/typescript-exhaustive-switch.mdc" },
    { "name": "no-inline-imports", "file": "rules/no-inline-imports.mdc" }
  ]
}
```

---

## Best Practices from the Ecosystem

### From awesome-cursorrules

The [PatrickJS/awesome-cursorrules](https://github.com/PatrickJS/awesome-cursorrules) repository contains 150+ curated rule files. Key patterns:

1. **Description frontmatter** — Every rule starts with a clear description
2. **Glob patterns** — Scope rules to specific file types
3. **Concrete examples** — Show preferred patterns with code
4. **Minimal rules** — Focus on what matters, skip the obvious

### Rule Categories

| Category | Examples |
|----------|----------|
| **Frontend** | React, Vue, Svelte, Next.js patterns |
| **Backend** | FastAPI, NestJS, Rails conventions |
| **Testing** | Jest, Vitest, Playwright, Cypress |
| **DevOps** | Kubernetes, CI/CD, Git flow |
| **Language** | TypeScript, Python, Go best practices |

### What Makes a Good Rule

1. **Specific** — Targets a real pain point
2. **Actionable** — Tells the agent exactly what to do
3. **Scoped** — Applies only where relevant (globs)
4. **Short** — Fits in context without bloating
5. **Tested** — Verified against actual agent behavior

---

## Mapping to Cursor Onboarding Kit

| Dev Kit Concept | Kit Equivalent |
|-----------------|----------------|
| Skills | `40-TEMPLATES/` starter pack |
| Commands | Slash commands in AGENTS.md |
| Subagents | Subagent patterns in `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md` |
| Rules | `.cursorrules` and `AGENTS.md` conventions |
| Hooks | Guardrails in AGENTS.md (Before You Stop, Landing the Plane) |

---

## Implementation Checklist

- [ ] Create `.cursor/` directory structure
- [ ] Add skills for: looping-on-ci, fix-ci, review-and-ship, run-smoke-tests
- [ ] Configure ci-watcher subagent
- [ ] Add rules for: typescript-exhaustive-switch, no-inline-imports
- [ ] Set up hooks.json with audit/telemetry
- [ ] Test with failing CI scenario
- [ ] Document team conventions

---

## Further Reading

- [25-AGENTS-MD-EFFECTIVENESS.md](25-AGENTS-MD-EFFECTIVENESS.md) - Why context files underperform
- [78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md](78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md) - Surfaces in depth
- [22-BCG-ENTERPRISE-AGENTS.md](22-BCG-ENTERPRISE-AGENTS.md) - Agent Design Cards
- [23-FORMAL-VERIFICATION-AGENTS.md](23-FORMAL-VERIFICATION-AGENTS.md) - Property-based testing with agents
- [Cursor Documentation](https://docs.cursor.com/)
- [awesome-cursorrules](https://github.com/PatrickJS/awesome-cursorrules)
