# Cursor Meta-Extraction: Comprehensive Knowledge Base

**Reference:** Processed output from Issue #13 meta-prompt extraction template.
**Source:** Cursor documentation at https://docs.cursor.com

---

## Overview

This document synthesizes Cursor's key features across 8 critical surfaces: Configuration, Cookbooks, Shared Transcripts, Semantic Search, Subagents, Rules, Hooks, and Security. It serves as a consolidated reference for teams adopting Cursor as a workflow platform.

---

## 1. Configuration (CLI & Editor)

### File Locations

| Scope | Path | Platform |
|-------|------|----------|
| **User/Global** | `~/.cursor/` | All |
| **Project** | `.cursor/` | All |
| **Enterprise** | Managed by org policy | All |

### Key Configuration Files

```
.cursor/
├── commands/          # Reusable entry points (*.md)
├── skills/            # Reusable procedures (*/SKILL.md)
├── rules/             # Project rules (*.mdc)
├── hooks.json         # Event automation config
└── AGENTS.md          # Simple markdown context (alternative to rules)

~/.cursor/
├── hooks.json         # User-level hooks
├── hooks/             # Hook scripts
└── mcp.json           # MCP server config
```

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `CURSOR_PROJECT_DIR` | Project root path |
| `CURSOR_VERSION` | Cursor version |
| `PATH` | Shell tool access |
| Proxy vars | HTTP/HTTPS proxy settings |

### Shell Profile Loading

If tools are available in native terminal but missing in Cursor:
1. Verify integrated shell profile is correct
2. Verify `PATH` and exported environment variables
3. Restart terminal/editor after profile changes

Reference: https://docs.cursor.com/configuration/shell

### Model Selection

| Model | Default Context | Max Mode |
|-------|-----------------|----------|
| Claude 4.6 Opus | 200k | 1M |
| Claude 4.6 Sonnet | 200k | 1M |
| Composer 1.5 | 200k | - |
| Gemini 3 Flash | 200k | 1M |
| Gemini 3 Pro | 200k | 1M |
| GPT-5.2 | 272k | - |
| GPT-5.3 Codex | 272k | - |
| Grok Code | 256k | - |

### CLI Configuration

```bash
# Installation
curl https://cursor.com/install -fsS | bash

# Authentication
cursor-agent login              # Browser-based
cursor-agent login --api-key    # CI/CD environments

# Model switching
/model claude-4-6-opus
/model claude-4-6-sonnet

# Update
cursor-agent update
cursor-agent upgrade
```

References:
- https://cursor.com/docs/cli/overview
- https://cursor.com/docs/cli/reference/parameters
- https://cursor.com/docs/cli/reference/permissions

---

## 2. Cookbooks (Practical Workflows)

### TDD Workflow

1. Write failing test first
2. Run test, confirm failure
3. Implement minimal code to pass
4. Run test, confirm pass
5. Refactor if needed
6. Repeat

Example prompts:
- "Write a test for X, then implement it"
- "Make this test pass with minimal changes"

### Git Workflows

**PR Creation:**
```bash
/ship-pr    # Commit + PR checklist flow
```

**Issue Fixing:**
1. Read issue
2. Reproduce problem
3. Write test case
4. Implement fix
5. Verify tests pass
6. Create PR with issue reference

### Codebase Understanding

Onboarding questions that work:
- "What is the architecture of this codebase?"
- "Where is X feature implemented?"
- "How does data flow from A to B?"
- "What are the main entry points?"

Exploration strategies:
- Start with entry points (main, index, app)
- Follow data flow
- Map module boundaries
- Identify key abstractions

### Architecture Diagrams

Generate Mermaid diagrams:
```
"Generate a Mermaid diagram showing the data flow for user authentication"
```

### Long-Running Agent Loops

Good halting conditions:
- "Keep iterating until lint/tests pass"
- "Continue until PR summary matches contract"

Bad halting conditions:
- "Keep iterating until it's done" (infinite yak-shaving)

### Design to Code

1. Attach image/screenshot
2. Describe desired behavior
3. Agent generates implementation
4. Iterate on styling/details

### Command Structures

```
.cursor/commands/
├── ship-pr.md
├── weekly-insights.md
├── risk-review.md
└── tdd-cycle.md
```

---

## 3. Shared Transcripts

### Feature Overview

Shared transcripts allow teams to:
- Share agent conversations
- Fork and continue from shared context
- Maintain audit trails

### Visibility Types

| Type | Access | URL Format |
|------|--------|------------|
| **Team** | Team members only | Internal link |
| **Public** | Anyone with link | Public URL |

### Sharing Methods

**From Cursor:**
1. Complete a conversation
2. Use share/export function
3. Set visibility (Team/Public)
4. Copy link

**From Dashboard:**
- Manage existing transcripts
- Change visibility
- Delete transcripts

### Viewing and Forking

- Deeplinks open specific conversation state
- Fork creates a new conversation starting from shared point
- Continuation workflows preserve context

### Privacy Considerations

- What's shared: Full conversation history, code context
- Redaction: Manual review before sharing
- Storage: Cursor servers (encrypted)

### Requirements

- Plan requirements vary by feature
- Privacy mode affects what can be shared
- Team admin can enable/disable controls

References:
- https://cursor.com/docs/shared-transcripts
- https://cursor.com/docs/agent/chat/export

---

## 4. Semantic Search

### How It Works (7-Step Process)

1. **File sync** - Detect files to index
2. **Chunking** - Split files into smaller pieces
3. **Embedding** - Generate vector embeddings
4. **Storage** - Store in vector database
5. **Query** - Convert query to vector
6. **Search** - Find similar embeddings
7. **Results** - Return relevant chunks

### Architecture

```
┌─────────────┐
│  Codebase   │
└──────┬──────┘
       ↓
┌─────────────┐
│  Chunking   │  Split files into semantic units
└──────┬──────┘
       ↓
┌─────────────┐
│  Embeddings │  Vector representation (encrypted)
└──────┬──────┘
       ↓
┌─────────────┐
│  Vector DB  │  Similarity search
└─────────────┘
```

### Benefits vs Grep

| Feature | Semantic Search | Grep |
|---------|-----------------|------|
| **Conceptual matching** | ✅ | ❌ |
| **Exact string** | ✅ | ✅ |
| **Speed** | Fast | Very fast |
| **Accuracy** | Contextual | Literal |

### Indexing Process

**First-time setup:**
- Automatic when opening project
- Indexed when ~80% threshold reached
- Progress shown in UI

**Automatic sync:**
- Every 5 minutes (batch processing)
- Detects new/modified/deleted files
- Uses Merkle tree for efficiency

### File Handling

| File State | Action |
|------------|--------|
| New | Index |
| Modified | Re-index changed portions |
| Deleted | Remove from index |
| Skipped | `.gitignore`, `.cursorignore` |

### Privacy and Security

- File names and code are encrypted/obfuscated
- Server cannot reconstruct original code
- Embeddings are one-way
- Respects `.gitignore` and `.cursorignore`

### Configuration

**Settings:**
- Cursor Settings > Indexing & Docs
- View indexed files
- Configure ignore patterns

**Retention:**
- 6-week inactivity deletion

### Multi-Root Workspaces

- Supported with limitations
- Each root indexed separately
- Cross-root search available

Reference: https://cursor.com/docs/context/codebase

---

## 5. Subagents

### Core Concepts

| Concept | Description |
|---------|-------------|
| **Context isolation** | Each subagent has isolated context |
| **Parallel execution** | Run multiple subagents simultaneously |
| **Specialization** | Purpose-built for specific tasks |

### Built-in Subagents

| Subagent | Purpose |
|----------|---------|
| **Explore** | Codebase exploration, file discovery |
| **Bash** | Command execution specialist |
| **Browser** | Web automation, UI testing |

### Execution Modes

| Mode | Description |
|------|-------------|
| **Foreground** | Blocking, returns results to main agent |
| **Background** | Non-blocking, can run long tasks |

### File Structure

**Project location:** `.cursor/subagents/*.md`
**User location:** `~/.cursor/subagents/*.md`

Precedence: Project > User

### File Format (YAML Frontmatter)

```yaml
---
name: my-subagent
description: What this subagent does
model: fast | inherit | claude-4-6-sonnet
readonly: true | false
is_background: true | false
---

# Instructions

Your subagent instructions here...
```

### Model Configuration

| Value | Meaning |
|-------|---------|
| `fast` | Use fast model (Haiku-class) |
| `inherit` | Use parent agent's model |
| `claude-4-6-sonnet` | Specific model ID |
| Max Mode | Requires model that supports it |

### Usage Patterns

**Automatic delegation:**
- Agent decides when to delegate
- Based on task complexity

**Explicit invocation:**
- "Use the explore subagent to find X"
- "Spawn a background subagent to run tests"

**Parallel execution:**
- Run multiple subagents for independent tasks
- Aggregate results

### Resuming Subagents

- Each subagent has a unique agent ID
- Can resume from previous execution
- Useful for long-running tasks

### Common Patterns

| Pattern | Use Case |
|---------|----------|
| **Verification agent** | Verify changes before merge |
| **Orchestrator** | Coordinate multiple specialists |
| **Debugger** | Isolate and fix bugs |
| **Test runner** | Execute test suites |

### Best Practices

- **Focused agents** - One purpose per subagent
- **Clear descriptions** - Help main agent decide when to delegate
- **Version control** - Commit stable subagent configs

### Anti-Patterns

- Vague descriptions
- Too many agents (cognitive overhead)
- Duplicating built-in commands

### Performance Considerations

- Token usage increases with parallel agents
- Overhead from context switching
- Cost scales with subagent count

Reference: https://cursor.com/docs/context/subagents

---

## 6. Rules

### Rule Types

| Type | Location | Scope |
|------|----------|-------|
| **Project** | `.cursor/rules/*.mdc` | Team-shared |
| **User** | Settings > General > Rules for AI | Personal |
| **Team** | Dashboard | Organization |
| **AGENTS.md** | Repository root | Simple markdown |

### How Rules Work

- Injected into prompt context
- Present in model's "memory" for session
- Matched by file patterns (globs) or always applied

### Project Rules (.mdc format)

```yaml
---
description: TypeScript component standards
globs: "*.tsx"
alwaysApply: false
---

# Component Standards

- Use functional components
- Props via interface
- Export default for pages
```

### Rule Anatomy

| Type | Behavior |
|------|----------|
| **Always Apply** | Included in every request |
| **Apply Intelligently** | Agent decides when relevant |
| **Apply to Specific Files** | Matched by globs |
| **Apply Manually** | Only when explicitly requested |

### Creating Rules

1. Command palette: "New Cursor Rule"
2. Or create `.mdc` file in `.cursor/rules/`
3. Add frontmatter and content
4. Commit to share with team

### Best Practices

- **Length limits** - Keep concise (<500 words)
- **Focus** - One rule, one purpose
- **Examples** - Include concrete examples
- **Avoid duplication** - Don't repeat existing docs

### What to Avoid

- Copying style guides model already knows
- Edge cases that rarely trigger
- Duplicating codebase content
- Long exhaustive documentation

### Team Rules

- Managed via Cursor Dashboard
- Activation by team admin
- Enforcement across team
- Precedence: Team > Project > User

### Importing Rules

- Remote/GitHub rules
- Agent Skills from marketplace

### AGENTS.md (Simple Alternative)

```markdown
# Build & Test
- Run tests: `pytest tests/`
- Lint: `ruff check .`

# Conventions
- Use `uv` for dependency management
- Follow existing module patterns
```

**Supports:**
- Nested directories (inheritance)
- Simple markdown format
- Lower overhead than .mdc

### Legacy Support

- `.cursorrules` deprecated
- Migrate to `.cursor/rules/` or `AGENTS.md`

Reference: https://cursor.com/docs/context/rules

---

## 7. Hooks

### Overview

Hooks observe, control, and extend the agent loop via stdio bidirectional JSON communication.

### Use Cases

| Use Case | Example |
|----------|---------|
| **Formatters** | Auto-format after edit |
| **Analytics** | Log agent actions |
| **PII scanning** | Block sensitive data |
| **Gating** | Require conditions before actions |

### Agent vs Tab Support

Different hook events available for:
- Agent mode (full agentic loop)
- Tab mode (inline completions)

### Hook Types

| Type | Mechanism |
|------|-----------|
| **Command-based** | Shell script execution |
| **Prompt-based** | LLM evaluation |

### Exit Code Behavior

| Code | Meaning |
|------|---------|
| 0 | Success (allow) |
| 2 | Block (deny) |
| Other | Fail-open (allow with warning) |

### Prompt-Based Hooks

- LLM evaluates conditions
- Structured responses
- `$ARGUMENTS` placeholder for dynamic values

### Configuration Locations

Priority order:
1. **Enterprise** - Org policy
2. **Team** - Shared config
3. **Project** - `.cursor/hooks.json`
4. **User** - `~/.cursor/hooks.json`

### Working Directories

| Location | Working Directory |
|----------|-------------------|
| Project | Project root |
| User | `~/.cursor/` |
| Enterprise | Managed path |

### Configuration Options

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "./scripts/audit.sh",
            "timeout": 30000
          }
        ]
      }
    ]
  }
}
```

| Option | Description |
|--------|-------------|
| `version` | Config schema version |
| `command` | Script to execute |
| `type` | "command" or "prompt" |
| `timeout` | Max execution time |
| `loop_limit` | Max iterations |
| `matcher` | Tool/event filter |

### Matcher Configuration

- Tool type: `"Bash"`, `"Edit"`, `"Write"`
- Regex: `"Edit|Write"`, `"Web.*"`
- Wildcard: `"*"` (all tools)

### Hook Events (Complete Reference)

**Session Events:**
- `sessionStart` - New session begins
- `sessionEnd` - Session terminates

**Tool Events:**
- `preToolUse` - Before tool execution
- `postToolUse` - After successful tool use
- `postToolUseFailure` - After failed tool use

**Subagent Events:**
- `subagentStart` - Subagent spawned
- `subagentStop` - Subagent completes

**Shell/MCP Events:**
- `beforeShellExecution` - Before shell command
- `afterShellExecution` - After shell command
- `beforeMCPExecution` - Before MCP call
- `afterMCPExecution` - After MCP call

**File Events:**
- `beforeReadFile` - Before file read
- `afterFileEdit` - After file edit

**Tab Events:**
- `beforeTabFileRead` - Tab mode file read
- `afterTabFileEdit` - Tab mode file edit

**Prompt Events:**
- `beforeSubmitPrompt` - Before prompt sent

**Agent Events:**
- `afterAgentResponse` - After agent response
- `afterAgentThought` - After agent thinking

**Context Events:**
- `preCompact` - Before context compaction

**Completion:**
- `stop` - Session complete

### Environment Variables

| Variable | Description |
|----------|-------------|
| `CURSOR_PROJECT_DIR` | Project root |
| `CURSOR_VERSION` | Cursor version |

### Example Hooks

**Audit logging:**
```bash
#!/bin/bash
# scripts/hooks/audit.sh
echo "$(date): $CURSOR_PROJECT_DIR - Tool: $TOOL_NAME" >> ~/.cursor/audit.log
```

**Block destructive commands:**
```bash
#!/bin/bash
# scripts/hooks/block-git.sh
if [[ "$COMMAND" == *"rm -rf"* ]] || [[ "$COMMAND" == *"git push --force"* ]]; then
  echo "Blocked destructive command"
  exit 2
fi
```

### Team Distribution

- Project hooks via VCS (`.cursor/hooks.json`)
- MDM for enterprise distribution
- Cloud distribution for managed teams

### Partner Integrations

- MintMCP
- Oasis
- Runlayer
- Corridor
- Semgrep
- Endor Labs
- Snyk
- 1Password

Reference: https://cursor.com/docs/agent/hooks

---

## 8. Security

### Philosophy

> Guardrails by default. Manual approval for sensitive operations.

### First-Party Tool Calls

| Tool | Approval Required |
|------|-------------------|
| File read | Configurable |
| File edit | Yes (by default) |
| Terminal commands | Yes (by default) |

### File Access Control

- `.cursorignore` - Exclude files from indexing/agent
- Approval requirements for config files
- Protected file patterns

### Configuration File Protection

Files requiring explicit approval:
- `.env` files
- Credential files
- SSH keys
- API key files

### Terminal Command Approval

**Default behavior:**
- Commands require approval
- Allowlist available for safe commands

**Allowlist limitations:**
- Not exhaustive
- Use explicit approval for sensitive operations

### "Run Everything" Mode

**WARNING: Never use in production**

- Dramatically increases blast radius
- No approval prompts
- Only for disposable environments

### Third-Party Tools (MCP)

- MCP servers require approval
- Audit MCP server permissions
- Restrict by workspace policy

### Network Requests

Limited to:
- GitHub API
- Link fetching
- Web search (when enabled)

### Workspace Trust

- Disabled by default
- Configure trust per workspace
- Trusted workspaces have relaxed restrictions

### Approval Modes

| Mode | Description |
|------|-------------|
| **Ask** | Prompt for every action |
| **Allowlist** | Pre-approved commands only |
| **Auto** | Automatic (restrictive baseline) |

### Responsible Disclosure

- Email: security-reports@cursor.com
- Response timeline: Standard disclosure process

References:
- https://cursor.com/docs/agent/security
- https://cursor.com/docs/agent/terminal
- https://cursor.com/docs/cli/reference/permissions

---

## Quick Reference Tables

### File Locations Summary

| Purpose | Location |
|---------|----------|
| User config | `~/.cursor/` |
| Project config | `.cursor/` |
| Commands | `.cursor/commands/*.md` |
| Skills | `.cursor/skills/*/SKILL.md` |
| Rules | `.cursor/rules/*.mdc` or `AGENTS.md` |
| Hooks | `.cursor/hooks.json` |
| Subagents | `.cursor/subagents/*.md` |

### Hook Events Quick Reference

| Event | When |
|-------|------|
| `preToolUse` | Before any tool |
| `postToolUse` | After tool success |
| `beforeShellExecution` | Before shell command |
| `sessionStart` | Session begins |
| `sessionEnd` | Session ends |

### Security Defaults

| Action | Default |
|--------|---------|
| File read | Configurable |
| File edit | Approval required |
| Terminal | Approval required |
| MCP | Approval required |
| Network | Limited |

---

## Related Kit Files

| File | Topic |
|------|-------|
| `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md` | Surfaces deep-dive |
| `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md` | CLI, web, enterprise |
| `83-CURSOR-SETTINGS-CONFIG-FIRST.md` | Configuration |
| `25-AGENTS-MD-EFFECTIVENESS.md` | Context file research |
| `24-PERSONA-ANCHORS.md` | Agent behavior patterns |

---

## External References

- Cursor Docs: https://docs.cursor.com
- Changelog: https://www.cursor.com/changelog
- CLI Overview: https://cursor.com/docs/cli/overview
- Hooks: https://cursor.com/docs/agent/hooks
- Rules: https://cursor.com/docs/context/rules
- Subagents: https://cursor.com/docs/context/subagents
- Security: https://cursor.com/docs/agent/security
- Bugbot: https://cursor.com/docs/bugbot
- Cloud Agent: https://cursor.com/docs/cloud-agent
