#!/usr/bin/env bash
# drop-in.sh - Install cursor-onboarding-kit essentials
# Usage: ./drop-in.sh [target-dir]
#
# Copies minimal files to start a new project:
#   - AGENTS.md (project instructions)
#   - CLAUDE.md (symlink to AGENTS.md)
#   - 00-START-HERE.md (entry point)
#   - 01-WEEK-ONE-CHECKLIST.md (quick start)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="${1:-.}"

# Create target if needed
mkdir -p "$TARGET"

# Copy core files
cp "$SCRIPT_DIR/AGENTS.md" "$TARGET/"
cp "$SCRIPT_DIR/00-START-HERE.md" "$TARGET/"
cp "$SCRIPT_DIR/01-WEEK-ONE-CHECKLIST.md" "$TARGET/"

# Create symlink
cd "$TARGET"
if [ -L CLAUDE.md ]; then
  rm CLAUDE.md
elif [ -f CLAUDE.md ]; then
  echo "Warning: CLAUDE.md exists, backing up to CLAUDE.md.bak"
  mv CLAUDE.md CLAUDE.md.bak
fi
ln -s AGENTS.md CLAUDE.md

# Initialize beads if available
if command -v bd &> /dev/null; then
  bd init --prefix "$(basename "$TARGET" | tr '[:upper:]' '[:lower:]' | tr -cd '[:alnum:]')"
  echo "Initialized beads with prefix: $(basename "$TARGET")"
fi

echo ""
echo "✓ Drop-in complete!"
echo ""
echo "Files:"
echo "  $TARGET/AGENTS.md           - Project instructions"
echo "  $TARGET/CLAUDE.md           - Symlink to AGENTS.md"
echo "  $TARGET/00-START-HERE.md    - Entry point"
echo "  $TARGET/01-WEEK-ONE-CHECKLIST.md - Quick start"
echo ""
echo "Next: Edit AGENTS.md with your project specifics"
