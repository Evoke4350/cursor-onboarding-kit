# Local-Only Personalization (No Repo Noise)

Use this when you want strong personal workflow customization without polluting project diffs.

## Goal

Keep personal files and machine-specific configs out of commits while retaining a powerful local setup.

## Preferred Strategy

1. Put personal assets in local paths (for example a blackboard folder).
2. Exclude those files locally via `.git/info/exclude`.
3. Optionally use a global excludes file for cross-repo patterns.
4. Keep team policy in committed files (`AGENTS.md`, shared rules).

## Local Exclude (Per Repo)

Edit `.git/info/exclude` in the current repo:

```gitignore
# Personal AI workflow files (local only)
AGENTS.local.md
CLAUDE.local.md
.cursor/local/**
.cursor/private/**
.agentic-blackboard/private/**
```

This file is not committed, so it avoids `.gitignore` churn.

## Global Exclude (Across Repos)

Use `~/.gitignore_global` for personal patterns you never want tracked anywhere.

Example:

```gitignore
.DS_Store
*.local.md
.env.local
.cursor/local/**
```

Then set once on your machine:

`git config --global core.excludesfile ~/.gitignore_global`

## Suggested Team Convention

- Team-shared standards: committed.
- Personal/operator preferences: local-only.
- Promote local patterns to team policy only after repeated value.

## Safety Checklist

- [ ] No tokens in committed files
- [ ] No personal local config in staged changes
- [ ] Local excludes documented in onboarding notes
