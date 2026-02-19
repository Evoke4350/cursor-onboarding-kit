# Bootstrap System

Universal agent project bootstrap with semantic delimiters.

## Quick Start

```bash
# In any project directory
curl -sSL https://raw.githubusercontent.com/.../bootstrap.sh | bash

# Or clone and run
./bootstrap.sh
```

## What It Does

1. **Creates structure** - Memory, state, and config directories
2. **Generates derivation.yaml** - Maps semantic delimiters to project paths
3. **Creates identity file** - Project description for agents
4. **Creates checkpoint schema** - Recoverable state format

## Naming Schemes

| Scheme | Identity | Memory | Sessions | State |
|--------|----------|--------|----------|-------|
| workshop | `bench/identity.md` | `shavings/` | `sawdust/` | `sawdust/state/` |
| standard | `identity.md` | `notes/` | `sessions/` | `.state/` |
| minimal | `.identity.md` | `.memory/` | `.scratch/` | `.state/` |
| custom | (you choose) | (you choose) | (you choose) | (you choose) |

## Semantic Delimiters

Templates use delimiters that resolve to project-specific paths:

| Delimiter | Resolves To |
|-----------|-------------|
| `{{PROJECT_NAME}}` | Human-readable name |
| `{{PROJECT_SLUG}}` | URL-safe identifier |
| `{{MEMORY:identity}}` | Identity file path |
| `{{MEMORY:long}}` | Long-term memory directory |
| `{{MEMORY:short}}` | Session/ephemera directory |
| `{{MEMORY:state}}` | Checkpoint directory |
| `{{MEMORY:config}}` | Configuration directory |
| `{{WORKFLOW:capture}}` | Capture command |
| `{{WORKFLOW:search}}` | Search command |

## Example: store-ui Project

```bash
$ cd ~/projects/store-ui
$ ./bootstrap.sh

Project name: Store UI
Project type: software
Naming style: minimal (.identity/.memory/.state)

✓ Bootstrapped Store UI

Structure:
  .identity.md     - Project identity
  .memory/         - Long-term memory
  .scratch/        - Session data
  .state/          - Checkpoints
  .agent/          - Configuration
```

Generated `derivation.yaml`:

```yaml
project:
  name: "Store UI"
  slug: "store-ui"
  type: software

memory:
  identity: ".identity.md"
  long: ".memory/"
  short: ".scratch/"
  state: ".state/"
  config: ".agent/"
```

## Templates

Templates in `templates/` use semantic delimiters:

```markdown
# Session Log
Location: {{MEMORY:short}}/sessions/YYYYMMDD-HHMM.md

State file: {{MEMORY:state}}/checkpoint.yaml
Memory: {{MEMORY:long}}/insight.md
```

When copied to a project, resolve to project-specific paths.

## Integration with Workshop CLI

```bash
# Workshop reads derivation.yaml for naming
workshop init . --from-derivation

# Templates resolve automatically
workshop cut src/auth.rs  # Creates in {{MEMORY:long}}/
```

## Files

```
specs/bootstrap/
├── bootstrap.sh              # Main bootstrap script
├── templates/
│   ├── checkpoint.yaml       # Generic checkpoint schema
│   ├── session-log.md        # Generic session log
│   └── handoff.md            # Generic handoff template
└── README.md                 # This file
```

## For Template Authors

Use semantic delimiters in templates:

```markdown
# Bad (workshop-specific)
State file: sawdust/state/checkpoint.yaml

# Good (semantic)
State file: {{MEMORY:state}}/checkpoint.yaml
```

Resolution happens at copy/generation time based on `derivation.yaml`.
