# Spec: Tool Architecture (File-Based Discovery)

**Date**: 2026-02-17
**Status**: Draft
**Bead**: cursor-onboarding-kit-[TBD]

---

## Problem

MCP servers require a running process. For a "blackboard" cognitive architecture, we want:
- Zero runtime dependencies for core functionality
- Cursor IDE compatibility (no hook API, but file-based tool discovery)
- Sub-millisecond tool discovery
- Agent-native: tools as files, not API calls

---

## Solution: Tools as Files (Cursor Pattern)

From [Cursor Dynamic Context Discovery](https://cursor.com/blog/dynamic-context-discovery):

> We create one folder per server, keeping each server's tools logically grouped. When the model lists a folder, it sees all tools from that server together and can understand them as a cohesive unit. Files also enable more powerful searching. The agent can use full `rg` parameters or even `jq` to filter tool descriptions.

**Result**: 46.9% token reduction when MCP tools are file-based vs always-loaded.

---

## Architecture

```
~/workshop/
├── bench/                    # Graph space (MOCs, identity)
│   ├── index.md             # Hub MOC
│   ├── identity.md          # Who am I, what do I know
│   └── methodology.md       # How I work
│
├── shavings/                 # Notes (atomic insights)
│   └── *.md                 # Individual shavings
│
├── sawdust/                  # Ops (sessions, queues)
│   ├── sessions/            # Session logs
│   └── queue/               # Processing queue
│
└── .workshop/               # Workshop configuration
    ├── tools/               # Tool definitions (files!)
    │   ├── core/
    │   │   ├── cut.md       # Extract insight from source
    │   │   ├── carve.md     # Find connections
    │   │   ├── chamfer.md   # Update older shavings
    │   │   ├── check.md     # Validation + health
    │   │   └── sharpen.md   # Meta-cognitive refinement
    │   ├── search/
    │   │   ├── rg.md        # Ripgrep search
    │   │   ├── semantic.md  # Vector search (optional)
    │   │   └── graph.md     # Graph traversal
    │   └── setup/
    │       ├── init.md      # Bootstrap new workshop
    │       ├── calibrate.md # Adjust configuration
    │       └── health.md    # Diagnostic checks
    │
    ├── schema.yaml          # Caliper validation rules
    ├── config.yaml          # Workshop configuration
    └── policy.univ          # Optional: taint policy (Universalis-style)
```

---

## Tool Definition Format

Each tool is a markdown file with YAML frontmatter:

```markdown
---
name: cut
description: Extract atomic insight from source with code context
category: core
requires:
  - ripgrep
performance:
  target: 1ms
  warning: 10ms
  panic: 50ms
invocation: workshop cut <source> [--with-code-ref]
---

# Cut

Extract an atomic insight (shaving) from a source file.

## Usage

```bash
workshop cut src/auth/login.ts --with-code-ref
```

## What it does

1. Reads the source file
2. Identifies key concepts/patterns
3. Creates a new shaving in `shavings/`
4. Optionally embeds `file:line` reference

## Output

Creates: `shavings/<prose-title>.md`

## Schema

```yaml
title: <prose proposition>
description: <mechanism or implication>
source: <file:line or URL>
created: <ISO date>
topics: [<topic>, ...]
```
```

---

## Tool Discovery Protocol

### For Cursor Agent

1. Agent lists `.workshop/tools/` to see available tools
2. Agent uses `rg` to search tool descriptions:
   ```bash
   rg -t md "extract" .workshop/tools/
   ```
3. Agent reads specific tool file for full instructions
4. Agent invokes tool via shell or CLI

### For Claude Code

Same protocol, but can also use MCP wrapper for structured I/O.

---

## Performance Budgets

From `destructive_command_guard` pattern:

| Tier | Target | Warning | Panic |
|------|--------|---------|-------|
| Quick reject | < 1μs | > 10μs | > 50μs |
| Fast path | < 75μs | > 200μs | > 500μs |
| Full pipeline | < 5ms | > 10ms | > 20ms |

Applied to workshop tools:

| Tool | Budget | Strategy |
|------|--------|----------|
| `cut` | 1ms | memchr quick reject, lazy file read |
| `carve` | 75ms | ripgrep + parallel processing |
| `chamfer` | 5ms | targeted updates only |
| `check` | 10ms | cached validation |
| `search` | 1ms | xf-style hybrid (Tantivy + vectors) |

---

## Rust CLI Structure

From `xf` and `destructive_command_guard` patterns:

```rust
// src/main.rs
fn main() -> Result<()> {
    let cli = Cli::parse();

    // Robot mode for machine-readable output
    let robot = cli.robot;

    match cli.command {
        Commands::Cut { source, with_code_ref } => {
            let budget = PerformanceBudget::fast_path();
            cmd::cut(source, with_code_ref, budget, robot)?
        }
        Commands::Carve { query } => {
            let budget = PerformanceBudget::full_pipeline();
            cmd::carve(query, budget, robot)?
        }
        // ...
    }
    Ok(())
}
```

### Key Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
memchr = "2"           # SIMD-accelerated search
ripgrep = "14"         # Or use grep-matcher crate
tantivy = "0.21"       # Full-text search (optional)
rayon = "1"            # Parallel processing
```

---

## Setup Script (Gum-Based)

From user requirement: professional, emoji-driven, energetic.

```bash
#!/usr/bin/env bash
# setup.sh - Bootstrap your Workshop

set -e

# Colors and emojis
GUM_SPIN_STYLE="gum style --foreground 212 --bold"
EMOJI_WORKSHOP="🪵"
EMOJI_BENCH="🪑"
EMOJI_SHAVING="🪚"
EMOJI_SAWDUST="💨"

echo ""
$GUM_SPIN_STYLE "$EMOJI_WORKHOUSE Welcome to the Workshop!"
echo ""

# Phase 1: Detect
gum spin --spinner dot --title "Detecting environment..." -- sleep 0.5
echo "  ✓ Found: $(uname -s)"
echo "  ✓ Gum: $(gum --version | head -1)"
echo ""

# Phase 2: Understand
$GUM_SPIN_STYLE "$EMOJI_BENCH Let's hang your blackboard..."
echo ""

WORK_TYPE=$(gum choose --header "What kind of work do you do?" \
    "software" "research" "writing" "other")

CODE_REFS=$(gum choose --header "Should shavings reference code?" \
    "yes" "no" "optional")

LOCATION=$(gum input --header "Where should we put the workshop?" \
    --value "./workshop")

# Phase 3: Derive (internal)
gum spin --spinner dot --title "Deriving configuration..." -- sleep 1

# Phase 4: Proposal
echo ""
$GUM_SPIN_STYLE "$EMOJI_SHAVING Here's what I'll create:"
echo ""
echo "  $LOCATION/"
echo "  ├── bench/      (your workbench)"
echo "  ├── shavings/   (curled insights)"
echo "  ├── sawdust/    (ephemeral byproduct)"
echo "  └── .workshop/  (configuration)"
echo ""

CONFIRM=$(gum confirm "Looks good?" && echo "yes" || echo "no")

if [ "$CONFIRM" != "yes" ]; then
    echo "Cancelled. Run again when ready."
    exit 0
fi

# Phase 5: Generate
gum spin --spinner dot --title "$EMOJI_SAWDUST Generating workshop..." -- {
    mkdir -p "$LOCATION"/{bench,shavings,sawdust,.workshop/tools/{core,search,setup}}

    # Generate identity
    cat > "$LOCATION/bench/identity.md" << EOF
---
created: $(date -Iseconds)
work_type: $WORK_TYPE
code_refs: $CODE_REFS
---

# Workshop Identity

This is your workshop. The bench is your workspace, shavings are your insights,
and sawdust is the ephemeral byproduct of thinking.

Configure me during setup or edit directly.
EOF

    # Generate tool definitions
    cp -r ~/.workshop/templates/tools/* "$LOCATION/.workshop/tools/"
}

# Phase 6: Validate
echo ""
$GUM_SPIN_STYLE "✨ Workshop ready!"
echo ""
echo "  Next steps:"
echo "    cd $LOCATION"
echo "    workshop health"
echo ""
```

---

## Taint Policy (Optional, Universalis-Style)

For security-conscious workflows, define a taint policy:

```yaml
# .workshop/policy.univ
# Security policy for workshop operations

sources:
  - pred: "read_file"
    field: "contents"
    reason: "Files may contain sensitive data"

sinks:
  - pred: "curl_post"
    data_field: "data"
    dest_field: "url"
    reason: "Network calls can exfiltrate data"

safe_destinations:
  - "internal.company.com"
  - "cdn.trusted.com"

rules:
  - name: "no_exfiltration"
    description: "Tainted data cannot reach untrusted destinations"
    violation: "source → sink → !safe"
```

This would be validated by a separate `workshop verify` command that runs taint analysis before executing workflows.

---

## Integration with Cursor

Since Cursor has no hook API, integration is file-based:

1. **Tool Discovery**: Cursor agent reads `.workshop/tools/**/*.md`
2. **Session Capture**: Write to `sawdust/sessions/` (file watcher optional)
3. **Context Injection**: Cursor can read `bench/identity.md` for context
4. **Terminal Sync**: Cursor syncs terminal output to files (already does this)

No MCP server needed for core functionality.

---

## Open Questions

1. Should tools be versioned with the workshop, or shared globally?
2. How do we handle tool updates without breaking existing workshops?
3. Should there be a tool registry/marketplace?
4. How does this interact with Claude Code's skill system?

---

## References

- [Cursor Dynamic Context Discovery](https://cursor.com/blog/dynamic-context-discovery)
- [destructive_command_guard](https://github.com/Dicklesworthstone/destructive_command_guard)
- [xf](https://github.com/Dicklesworthstone/xf)
- [coding_agent_session_search](https://github.com/Dicklesworthstone/coding_agent_session_search)
- Universalis: "In Code They Think; In Proof We Trust" (Google Doc)
- arscontexta: 249 research claims, 6-phase setup
