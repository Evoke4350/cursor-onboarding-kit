# PRD: Bootstrap Agentic Blackboard (Workshop)
**Format:** Engineering-First Knowledge System
**Date:** 2026-02-17
**Epic bead:** cursor-onboarding-kit-a4z

---

## Problem

We want an **agentic blackboard** — a persistent cognitive architecture for AI agents that persists knowledge across sessions, supports structured note-taking for engineers, and works with Cursor IDE (file-based, no hooks API).

### Why This Approach

**Parameter space is unmanageable.** Even advanced techniques like GRPO (Group Relative Policy Optimization) require costly parameter updates that are:
- Expensive to compute
- Irreversible once applied
- Brittle and destroy generality

**Context space is the alternative.** From arXiv:2510.08191 (Training-Free GRPO):
> LLMs can achieve similar output distribution effects by learning experiential knowledge as a token prior — lightweight, addresses data scarcity, avoids overfitting.

The model introspects its own attempts, extracts natural language rules about what worked, stores them in an experience library, and injects them in future prompts.

**The blackboard IS this experience library.**

Inspired by:
- **arscontexta** — Claude Code plugin that generates knowledge systems from conversation, backed by 249 research claims
- **agent-os** — Operating system for AI agents with specs, standards, and product context
- **Ralph Loop** — Autonomous agent iteration with fresh context per phase

We're not copying arscontexta. We're building a **workshop-themed** variant tailored for engineering workflows, with file-based hooks and a custom framework.

---

## Goal

Build a **Bootstrap Agentic Blackboard** system with:

1. **Workshop naming**: bench/, shavings/, sawdust/ folders
2. **Engineering-first framework**: Custom note-taking designed for code/technical work
3. **File-based hooks**: Watch .md changes as integration point
4. **Generated slash commands**: Adapted from arscontexta patterns, workshop-themed
5. **Interactive 6-phase setup**: Similar to arscontexta:setup but as a script

---

## Workshop Naming Scheme

| Concept | Folder | Purpose |
|---------|--------|---------|
| **Graph Space** | `bench/` | The workbench — where you work. Contains MOCs, identity, methodology |
| **Notes** | `shavings/` | Curled off the work — atomic knowledge units, insights, patterns |
| **Ops/Scribbles** | `sawdust/` | The ephemeral byproduct — session logs, queues, transient state |

### Extended Terminology

- **Tool rack** → Skills/slash commands available
- **Blueprint** → Template for a note type
- **Caliper** → Schema validation (measures if it fits)
- **Chamfer** → Edge-smoothing (rewriting/quality pass)
- **Clamp** → Hook that holds structure in place
- **Joint** → Wiki link connecting shavings
- **Dowel** → Cross-reference pin (bidirectional link verification)

---

## Engineering-First Framework (Draft)

Extending beyond Cornell 5 Rs and Zettelkasten for engineers:

### The 5 Cs (Construction)

| Phase | What | Command |
|-------|------|---------|
| **Capture** | Zero-friction capture to sawdust/inbox/ | Manual |
| **Cut** | Extract atomic insight with code context | `/cut` |
| **Carve** | Find connections, join with joints | `/carve` |
| **Chamfer** | Smooth edges — update older shavings | `/chamfer` |
| **Check** | Caliper validation + health | `/check` |

Plus meta-cognitive:
- **Sharpen** → `/sharpen` — Refine the system itself (like arscontexta's /rethink)

### Fresh Context Per Phase

Like arscontexta's Ralph integration, each phase spawns a fresh agent:
- Prevents context pollution ("the gutter")
- State persists in files and git, not LLM memory

---

## File-Based Hooks

Since Cursor IDE has no public hook API, we use filesystem watching:

| Event | What Triggers | Action |
|-------|--------------|--------|
| **Write to shavings/** | New .md file created | Caliper validation (schema check) |
| **Write to bench/identity.md** | Identity file changed | Re-orient on next session |
| **Session end** | Agent stops | Capture session state to sawdust/sessions/ |
| **Manual trigger** | `/check` command | Full vault health scan |

Implementation: Use `fswatch` or similar to monitor .md changes, run validation scripts.

---

## Slash Commands (Workshop-Generated)

### Core (always available after setup)

| Command | What |
|---------|------|
| `/cut` | Extract insight from source with code context |
| `/carve` | Find connections, update MOCs |
| `/chamfer` | Backward pass — update older shavings |
| `/check` | Caliper validation + vault health |
| `/sharpen` | Meta-cognitive: refine the system |
| `/next` | Next-action recommendation |
| `/stats` | Vault metrics |

### Setup/Meta

| Command | What |
|---------|------|
| `/workshop:setup` | 6-phase interactive setup |
| `/workshop:help` | Contextual guidance |
| `/workshop:health` | Diagnostic checks |
| `/workshop:calibrate` | Adjust configuration |

---

## 6-Phase Setup Process

Inspired by arscontexta's setup, but as an interactive script:

| Phase | What Happens |
|-------|-------------|
| **1. Detect** | Check environment (Cursor/Claude Code), available tools |
| **2. Understand** | 2-4 questions about your engineering domain |
| **3. Derive** | Map signals to configuration dimensions |
| **4. Propose** | Show what will be generated in workshop terms |
| **5. Generate** | Create folders, templates, commands, hooks config |
| **6. Validate** | Smoke test, show first-success guidance |

---

## Bead Hierarchy

```
cursor-onboarding-kit-a4z  [EPIC] Bootstrap Agentic Blackboard
├── cursor-onboarding-kit-6sq  [P0] Design engineering-first framework
├── cursor-onboarding-kit-yy6  [P0] Design workshop naming scheme
├── cursor-onboarding-kit-wkg  [P0] Design file-based hook system
├── cursor-onboarding-kit-e3y  [P0] Generate slash commands inventory
├── cursor-onboarding-kit-7vv  [P1] Research arscontexta 6-phase setup
├── cursor-onboarding-kit-2df  [P1] Research agent-os shape-spec
├── cursor-onboarding-kit-wly  [P1] Research note-taking frameworks
└── cursor-onboarding-kit-hs5  [P1] Inventory Cursor IDE hooks
```

---

## Definition of Done

- [ ] Workshop naming scheme documented with full terminology
- [ ] Engineering-first framework (5 Cs) defined
- [ ] File-based hook system designed
- [ ] Slash commands generated and inventoried
- [ ] 6-phase interactive setup script created
- [ ] First working prototype (can create a shaving, find joints)
- [ ] Documentation: how to bootstrap a new workshop

---

## Open Questions

1. Should we support multiple workshops per project, or one workshop per codebase?
2. How do shavings relate to code? Embed file:line references? Git blame?
3. What's the migration path from existing note systems (Obsidian, etc)?
4. Do we need semantic search (qmd) or is ripgrep + MOCs sufficient?
