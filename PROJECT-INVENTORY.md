# Project Inventory - Cursor Onboarding Kit

**Generated**: 2026-02-18
**Purpose**: Full inventory for restructuring as simple cursor drop-in

---

## Core Files (Drop-In Required)

| File | Purpose | Status |
|------|---------|--------|
| `AGENTS.md` | Main project instructions | ✅ Core |
| `CLAUDE.md` | Symlink to AGENTS.md | ✅ Symlink |

---

## Guides (Numbered 00-99)

### Onboarding (00-09)
- `00-START-HERE.md` - Entry point
- `01-WEEK-ONE-CHECKLIST.md` - Week 1 tasks
- `02-NO-FLUFF-OPERATING-GUIDE.md` - Operating principles
- `03-EXPERIENCED-ENGINEER-LENS-QA.md` - QA lens
- `04-TIPS-AND-TRICKS-SCORECARD.md` - Scorecard
- `05-AGENT-OPERATOR-PATTERN.md` - Agent pattern
- `06-DAY-IN-THE-LIFE-CURSOR-ENGINEER.md` - Daily workflow
- `07-CURSOR-TAB-KEY.md` - Tab key usage

### Foundations (10-19)
- `10-WORKFLOW-FOUNDATIONS.md` - Workflow basics
- `20-PROMPT-PATTERNS.md` - Prompt patterns

### Advanced (30-79)
- `30-MODEL-SWITCHING-ADVANCED.md`
- `35-INSTRUCTION-FILES-ADVANCED.md`
- `50-MARKDOWN-OPS.md`
- `60-PERSONALIZATION-LOCAL-ONLY-CONFIG.md`
- `70-ROVO-CONTENT-INGEST.md`
- `75-GITHUB-COPILOT-CONFIG-ADVANCED.md`
- `76-OTHER-AGENT-HARNESSES-ADVANCED.md`
- `77-CURSOR-AGENT-CLI-WEB-BOT-ENTERPRISE-ADVANCED.md`
- `78-HOOKS-SKILLS-COMMANDS-SUBAGENTS-ADVANCED.md`
- `79-CURSOR-CONTEXT-FOCUS-30-WAYS.md`

### Labs (80-89)
- `80-SAMPLE-PROJECT-LAB.md`
- `81-CURSOR-SHORTCUTS-ADVANCED-FOLLOW-UP.md`
- `82-CURSOR-DEBUG-MODE-UI-AFFORDANCES.md`
- `83-CURSOR-SETTINGS-CONFIG-FIRST.md`

### Reference (90-99)
- `90-GLOSSARY-AND-FRONTMATTER.md`
- `95-READING-LIST.md`
- `96-EXTERNAL-EVIDENCE-MAP-2026.md`
- `97-DEVILS-ADVOCATE-ADVERSARIAL-TAKE.md`
- `99-EPILOGUE-FRONTIER-SUBAGENT-ORCHESTRATION.md`
- `99-EPILOGUE-STATE-MACHINE.md`
- `99B-SUBAGENT-PROMPT-LIBRARY.md`
- `99C-MANUAL-REVIEW-COMMIT-HISTORY-CURATION.md`

---

## Templates (40-TEMPLATES/)

### Starter Pack
- `INSTRUCTION-STARTER-PACK/` - Complete starter kit
  - `AGENTS.md`, `CLAUDE.md` - Core templates
  - `AGENTS.local.md`, `CLAUDE.local.md` - Local overrides
  - `*.template.*.md` - Pattern templates
  - `SETUP-COMPAT-INSTRUCTIONS.sh` - Setup script

### Templates (20+)
- `TEMPLATE-handoff.md` - Session handoff
- `TEMPLATE-session-log.md` - Session logging
- `TEMPLATE-decision-record.md` - Decision tracking
- `TEMPLATE-pr.md` - PR template
- `TEMPLATE-rfc.md` - RFC template
- `TEMPLATE-runbook.md` - Runbook template
- `TEMPLATE-experiment.md` - Experiment template
- `TEMPLATE-briefing.md` - Briefing template
- `META-PROMPT-*.md` - Meta prompts

---

## Specs (specs/)

### bootstrap/
- `BOOTSTRAP-SEMANTIC-SPEC.md` - Semantic delimiter spec
- `bootstrap.sh` - Universal bootstrap script
- `templates/vocabulary.yaml` - Semantic vocabulary

### semantic/
- `PROMPT-SEMANTIC-MAP.md` - AQAL-inspired mapping
- `PROMPT-CROSSWALK.md` - Cross-system translation

### integrations/
- `AGENT-OS-PATTERNS.md` - Extracted patterns
- `ARSCONTEXTA-PATTERNS.md` - Extracted patterns
- `BEADS-INTEGRATION-SPEC.md` - Optional integration

### blackboard-bootstrap/
- 27 files - Full blackboard spec (PRD, architecture, etc.)

---

## Workshop CLI (workshop-cli/)

### Rust Source
- `src/main.rs` - Entry point
- `src/cli/` - CLI commands
- `src/modules/` - Core modules

### Hooks (Python)
- `hooks/workshop-hook-read.py` - Read hook
- `hooks/workshop-hook-shell.py` - Shell hook
- `hooks/hooks.json` - Hook config

### Workshop Structure
- `bench/` - Identity files
- `shavings/` - Long-term memory
- `sawdust/` - Session data
- `.workshop/` - Config

---

## Labs

- `LAB-changelog-viewer/` - Changelog lab
- `LAB-rn-insurance-20-tricks/` - RN tricks lab
- `LAB-scheduling-demo/` - Scheduling demo

---

## Beads Integration

- `.beads/` - Issue tracking
- `BEADS-ARCHITECTURE.md` - Beads docs
- `BEADS-DEPENDENCY-TYPES.md` - Edge types

---

## Proposed Restructure

### Minimal Drop-In (Core)
```
AGENTS.md           # Main instructions
CLAUDE.md           # Symlink to AGENTS.md
00-START-HERE.md    # Entry point
01-WEEK-ONE-CHECKLIST.md  # Quick start
```

### Optional Add-Ons
```
guides/             # All 00-99 guides
templates/          # Templates directory
specs/              # Architecture specs
workshop-cli/       # Optional CLI tool
```

---

## Beads-Driven Workflow

1. `bd ready` → Find available work
2. `bd update <id> --status=in_progress` → Claim
3. Work → Track in checkpoint
4. `bd close <id>` → Complete
5. `bd sync` → Push to remote

---

## Next Steps

1. Close stale beads issues
2. Create new issues for simplified structure
3. Extract core drop-in package
4. Document beads workflow in AGENTS.md
