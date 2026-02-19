# Optional Components

These are not required for the drop-in package but provide deeper integration.

## specs/

Architecture specifications and design documents:

- `bootstrap/` - Universal project bootstrap with semantic delimiters
- `semantic/` - AQAL-inspired prompt mapping
- `integrations/` - Extracted patterns from agent-os, arscontexta
- `blackboard-bootstrap/` - Full blackboard architecture spec

## workshop-cli/

Rust CLI tool with Cursor hooks for taint tracking and memory management.

### Commands

- `workshop init` - Create workshop structure
- `workshop cut` - Extract insight to long-term memory
- `workshop carve` - Search memory
- `workshop chamfer` - Update existing knowledge
- `workshop check` - Validate structure

### Hooks

Python hooks for Cursor IDE:

- `workshop-hook-read.py` - Read permission gate
- `workshop-hook-shell.py` - Shell command gate

### Structure

```
bench/          - Identity files
shavings/       - Long-term memory
sawdust/        - Session data
.workshop/      - Configuration
```
