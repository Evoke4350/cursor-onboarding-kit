#!/usr/bin/env bash
set -euo pipefail

# Bootstrap local-only Cursor onboarding workspace in a target repo.
# Usage:
#   ./bootstrap-cursor-onboarding-kit.sh [target_repo_path]

TARGET_REPO="${1:-$(pwd)}"

if [ ! -d "$TARGET_REPO/.git" ]; then
  echo "Error: target is not a git repository: $TARGET_REPO" >&2
  exit 1
fi

KIT_ROOT="$TARGET_REPO/.agentic-blackboard/CURSOR-ONBOARDING-KIT"
PRIVATE_ROOT="$TARGET_REPO/.agentic-blackboard/private"
RULES_DIR="$TARGET_REPO/.cursor/rules"
EXCLUDE_FILE="$TARGET_REPO/.git/info/exclude"

mkdir -p "$KIT_ROOT"
mkdir -p "$KIT_ROOT/40-TEMPLATES"
mkdir -p "$KIT_ROOT/scripts"
mkdir -p "$PRIVATE_ROOT/raw" "$PRIVATE_ROOT/normalized"
mkdir -p "$RULES_DIR"

touch "$EXCLUDE_FILE"

append_if_missing() {
  local line="$1"
  local file="$2"
  if ! rg -n --fixed-strings "^${line}$" "$file" >/dev/null 2>&1; then
    printf '%s\n' "$line" >> "$file"
  fi
}

# Local-only patterns to avoid git noise.
append_if_missing "# Cursor local-only personalization" "$EXCLUDE_FILE"
append_if_missing "AGENTS.local.md" "$EXCLUDE_FILE"
append_if_missing "CLAUDE.local.md" "$EXCLUDE_FILE"
append_if_missing ".cursor/local/**" "$EXCLUDE_FILE"
append_if_missing ".cursor/private/**" "$EXCLUDE_FILE"
append_if_missing ".agentic-blackboard/private/**" "$EXCLUDE_FILE"

echo "Bootstrap complete."
echo "Target repo: $TARGET_REPO"
echo "Created: $KIT_ROOT"
echo "Created: $PRIVATE_ROOT/{raw,normalized}"
echo "Updated local excludes: $EXCLUDE_FILE"
echo
echo "Next:"
echo "1) Copy docs/templates into $KIT_ROOT"
echo "2) Add project-specific commands in onboarding docs"
echo "3) Export tokens as env vars (never commit secrets)"
