#!/usr/bin/env bash
set -euo pipefail

# Compatibility setup for mixed-agent repos.
# - Keeps AGENTS.md as canonical by default
# - Bridges CLAUDE.md to AGENTS.md (import or symlink)
# - Adds local-only exclusions to .git/info/exclude
# - Supports teams working across multiple models and harnesses

ROOT_DIR="${1:-.}"
MODE="${2:-import}" # import | symlink

cd "$ROOT_DIR"

if [[ ! -f "AGENTS.md" ]]; then
  echo "ERROR: AGENTS.md not found in $ROOT_DIR"
  echo "Create AGENTS.md first, then re-run."
  exit 1
fi

mkdir -p .git/info
touch .git/info/exclude

append_exclude() {
  local pattern="$1"
  if ! rg -n "^${pattern}$" .git/info/exclude >/dev/null 2>&1; then
    printf "%s\n" "$pattern" >> .git/info/exclude
  fi
}

append_exclude "AGENTS.local.md"
append_exclude "CLAUDE.local.md"
append_exclude ".cursor/local/**"
append_exclude ".cursor/private/**"
append_exclude ".agentic-blackboard/private/**"

if [[ "$MODE" == "symlink" ]]; then
  if [[ -L "CLAUDE.md" || -f "CLAUDE.md" ]]; then
    rm -f "CLAUDE.md"
  fi
  ln -s "AGENTS.md" "CLAUDE.md"
  echo "Created symlink: CLAUDE.md -> AGENTS.md"
else
  cat > "CLAUDE.md" <<'EOF'
# CLAUDE.md

Compatibility bridge for Claude Code.
Canonical project instructions live in `AGENTS.md`.

@AGENTS.md
EOF
  echo "Created CLAUDE.md import bridge to AGENTS.md"
fi

if [[ ! -f "CLAUDE.local.md" ]]; then
  cat > "CLAUDE.local.md" <<'EOF'
# CLAUDE.local.md (local-only)

- Put personal machine or workflow preferences here.
- Keep this file uncommitted.
EOF
  echo "Created CLAUDE.local.md starter"
fi

if [[ ! -f "AGENTS.local.md" ]]; then
  cat > "AGENTS.local.md" <<'EOF'
# AGENTS.local.md (local-only)

- Put personal operator preferences here.
- Keep this file uncommitted.
EOF
  echo "Created AGENTS.local.md starter"
fi

echo
echo "Done."
echo "- Canonical source: AGENTS.md"
echo "- Claude bridge mode: $MODE"
echo "- Local excludes updated: .git/info/exclude"
echo
echo "Tip: run a quick duplicate check:"
echo "  rg -n \"^#|^##|Keep|Prefer|Avoid\" AGENTS.md CLAUDE.md .github/copilot-instructions.md"
