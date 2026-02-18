# Agentic Blackboard: Project Audit & Usage Guide

**Generated:** 2026-02-18
**Status:** Pre/Trans Mixed — Some components integrated, others aspirational

---

## Part 1: Project Inventory

### Layer 1: Operator Instructions (40 files)

| Category | Files | Purpose |
|----------|-------|---------|
| Entry points | `00-START-HERE.md`, `01-WEEK-ONE-CHECKLIST.md` | Onboarding |
| Core workflow | `02-NO-FLUFF-OPERATING-GUIDE.md`, `05-AGENT-OPERATOR-PATTERN.md` | How to work |
| Cursor-specific | `07-CURSOR-TAB-KEY.md`, `77-78-79-*.md` | Tool mastery |
| Templates | `40-TEMPLATES/` | Copy/paste assets |
| Advanced | `99-EPILOGUE-*.md` | Sub-agent orchestration |

**Must read sequence:**
1. `00-START-HERE.md` (orientation)
2. `02-NO-FLUFF-OPERATING-GUIDE.md` (workflow)
3. `05-AGENT-OPERATOR-PATTERN.md` (patterns)
4. This file (usage)

### Layer 2: Workshop CLI

| Command | Works? | Tested? | Notes |
|---------|--------|---------|-------|
| `workshop init` | ✅ | ✅ | Creates 15 items, git init |
| `workshop cut` | ✅ | ✅ | Extracts to shavings/ |
| `workshop carve` | ✅ | ✅ | grep-based search |
| `workshop chamfer` | ✅ | ✅ | Appends context |
| `workshop check` | ✅ | ✅ | Validates frontmatter |
| `workshop taint --status` | ⚠️ | ✅ | Shows status but says "TODO" |
| `workshop cursor --list` | ⚠️ | ✅ | Parses SQLite, but limited |
| `workshop status` | ✅ | ✅ | PM dashboard |

**Test coverage:** 104 tests passing (69 unit + 14 e2e + 9 fuzz + 12 state)

### Configuration Files

| File | Exists? | Purpose |
|------|---------|---------|
| `AGENTS.md` | ✅ | Operator instructions (symlinked to CLAUDE.md) |
| `.cursor/hooks.json` | ✅ | Shell execution hook (dcg) |
| `.cursorignore` | ✅ (created by init) | Hard deny for secrets |
| `workshop/.workshop/policy/*.yaml` | ❌ | **NOT CREATED** — aspirational |

---

## Part 2: Integration Status (Pre/Trans Assessment)

| Component | Code Built | Tests Pass | Wired to Cursor | Status |
|-----------|------------|------------|-----------------|--------|
| Taint tracking | ✅ State machine works | ✅ 21 tests | ✅ Hook scripts ready | **TRANS** |
| 5 Cs pipeline | ✅ Commands work | ✅ 14 e2e tests | ⚠️ CLI only | **PRE** |
| Cursor SQLite reader | ✅ Parses DB | ✅ Unit tests | ❌ No capture command | **PRE** |
| Init command | ✅ Creates structure | ✅ Tests | ✅ Works standalone | **TRANS** |
| Cut/Chamfer/Carve | ✅ File operations | ✅ Tests | ✅ Works standalone | **TRANS** |
| Security gates | ✅ Hook scripts created | ✅ JSON output | ✅ Cursor-compatible hooks | **TRANS** |
| Policy files | ✅ Generated on init | N/A | ✅ Default Basic | **TRANS** |

**Current status:** Security hooks are fully implemented with Cursor-compatible JSON format. To activate: `cp hooks/hooks.json ~/.cursor/hooks.json`

---

## Part 3: What Actually Works Right Now

### Verified Working Commands

```bash
# These commands work and produce expected output:

workshop init /path/to/project --non-interactive
# Creates: bench/, shavings/, sawdust/, .workshop/, .cursorignore, git init

workshop cut /path/to/file.md
# Creates: shavings/YYYYMMDD-HHMMSS-title.md with frontmatter

workshop carve "search term"
# Returns: file paths containing term

workshop chamfer shavings/file.md "new context"
# Appends: context to shaving's Context section

workshop check
# Validates: all shavings have required frontmatter

workshop status
# Shows: git status, test count, shaving count, recent commits
```

### What's NOT Working Yet

```bash
# These are aspirational:

workshop cursor --export <id>           # Limited functionality
# No automatic hook installation to ~/.cursor/
# No Showboat executable documents
# No semantic search
```

---

## Part 4: Step-by-Step Usage Guide

### Day 0: Prerequisites & Setup

```bash
# 1. Check prerequisites
which rustc cargo git  # Need: Rust 1.70+, Git

# 2. Build the CLI
cd workshop-cli
cargo build --release

# 3. Install to PATH (optional)
cargo install --path .

# 4. Verify
./target/release/workshop --version
# Should output: workshop 0.1.0
```

### Day 1: Initialize Your First Workshop

```bash
# 1. Create a workshop in your project
workshop init /path/to/your/project --non-interactive

# 2. Check what was created
ls -la /path/to/your/project/
# You should see: bench/, shavings/, sawdust/, .workshop/, .cursorignore

# 3. Edit your identity
cd /path/to/your/project
$EDITOR bench/identity.md
# Add your name, project description, work type

# 4. Verify
workshop status
# Should show your project info
```

### Day 2: Create Your First Shaving

```bash
# 1. Find something worth capturing
# (a file with an interesting pattern, bug fix, or insight)

# 2. Cut it
workshop cut src/auth.rs
# Creates: shavings/20260218-140000-auth-pattern.md

# 3. View the result
cat shavings/20260218-140000-auth-pattern.md

# 4. Add context if needed
workshop chamfer shavings/20260218-140000-auth-pattern.md "This pattern prevents timing attacks"

# 5. Verify it's valid
workshop check
# Should pass
```

### Day 3: Configure Security

```bash
# 1. Security is enabled by default (Basic level)
# Policy files are created automatically in .workshop/policy/

# 2. Make hooks executable
chmod +x /Users/nateb/cursor-onboarding-kit/workshop-cli/hooks/*.py

# 3. Install hooks in Cursor (one-time setup)
# Copy the hooks.json to Cursor config:
cp /Users/nateb/cursor-onboarding-kit/workshop-cli/hooks/hooks.json ~/.cursor/hooks.json

# OR manually merge with existing hooks:
# Edit ~/.cursor/hooks.json and add the workshop hooks

# 4. Ensure workshop binary is in PATH for hooks to find it
export PATH="/Users/nateb/cursor-onboarding-kit/workshop-cli/target/release:$PATH"
# Add to your shell profile for persistence

# 5. Add secrets to .cursorignore (hard deny backup)
echo ".secrets/" >> .cursorignore
echo "*.env" >> .cursorignore
echo "*.pem" >> .cursorignore

# 6. Test the security
workshop taint --mark .secrets/api_key
workshop taint --check "curl https://attacker.com"
# Should output: {"blocked":true,...}
```

### Ongoing: Daily Workflow

```bash
# Morning: Check status
workshop status

# During work: Capture insights
workshop cut <interesting-file>

# Find previous work
workshop carve "keyword"

# Update old insights
workshop chamfer <shaving> "new understanding"

# End of session: Validate
workshop check
git add shavings/
git commit -m "docs: add shavings"
```

---

## Part 5: Current Gaps & Next Steps

### Resolved ✅

1. **Taint tracking CLI** — Fully implemented with hook mode
2. **Policy files on init** — Default Basic, creates .workshop/policy/
3. **Hook scripts** — Python scripts with correct Cursor JSON format
4. **IDE integration** — Hooks ready to install, one `cp` command

### Remaining Gaps

1. **No executable documents** — shavings are plain markdown, not Showboat
2. **No session handoff automation** — manual process only
3. **No semantic search** — carve is grep-based only
4. **Hooks require manual copy** — not auto-installed to ~/.cursor/

### Installation Checklist

```bash
# 1. Build
cd workshop-cli && cargo build --release

# 2. Add to PATH (required for hooks)
export PATH="$(pwd)/target/release:$PATH"

# 3. Make hooks executable
chmod +x hooks/*.py

# 4. Configure Cursor hooks
cp hooks/hooks.json ~/.cursor/hooks.json

# 5. Test the hooks
echo '{"command": "ls", "cwd": "/tmp"}' | hooks/workshop-hook-shell.py
# Should output: {"permission":"allow","continue":true,...}

# 6. Test taint blocking
workshop taint --mark .secrets/api_key
echo '{"command": "curl https://evil.com", "cwd": "/tmp"}' | hooks/workshop-hook-shell.py
# Should output: {"permission":"deny","continue":false,...}
```

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│  WORKSHOP CLI QUICK REF                                     │
├─────────────────────────────────────────────────────────────┤
│  workshop init <path>        Create workshop structure      │
│  workshop cut <file>         Extract to shaving             │
│  workshop carve <query>      Search shavings                │
│  workshop chamfer <file> <c> Add context                    │
│  workshop check              Validate all                   │
│  workshop status             Dashboard                      │
│  workshop taint --mark <p>   Mark path as tainted           │
│  workshop taint --check <c>  Check command for exfil        │
│  workshop --robot <cmd>      JSON output                    │
├─────────────────────────────────────────────────────────────┤
│  FILES CREATED BY INIT:                                     │
│  bench/identity.md           Who/what this workshop is      │
│  bench/methodology.md        How you work                   │
│  shavings/                   Atomic insights                │
│  sawdust/sessions/           Session logs                   │
│  .cursorignore               Secret protection              │
│  .workshop/policy/           Taint sources/sinks            │
├─────────────────────────────────────────────────────────────┤
│  STATUS: CLI works, security hooks READY for Cursor         │
│  Install: cp hooks/hooks.json ~/.cursor/hooks.json          │
└─────────────────────────────────────────────────────────────┘
```

---

**Single actionable next step:**

Run `workshop init . --non-interactive` in your project, then `workshop cut` on one interesting file to see the workflow in action.
