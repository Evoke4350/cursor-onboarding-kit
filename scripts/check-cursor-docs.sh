#!/usr/bin/env bash
set -euo pipefail

# Quick docs health check for Cursor onboarding references.
# Fetches pages with curl and checks for expected keywords.

check() {
  local url="$1"
  local needle="$2"

  if curl -fsSL "$url" | rg -qi "$needle"; then
    echo "OK   $url"
  else
    echo "WARN $url (missing expected text: $needle)"
  fi
}

echo "Checking Cursor docs..."
check "https://cursor.com/docs/cli/overview" "CLI|agent"
check "https://cursor.com/docs/cloud-agent" "cloud|agent"
check "https://cursor.com/docs/cloud-agent/api/endpoints" "endpoint|api"
check "https://cursor.com/docs/bugbot" "bugbot|review"
check "https://cursor.com/docs/integrations/github" "github|integration"
check "https://cursor.com/docs/agent/security" "security|permission"
check "https://cursor.com/docs/cli/reference/permissions" "allow|deny|permission"
check "https://www.cursor.com/changelog" "changelog|release"
check "https://changelog.cursor.sh/" "changelog|release"

echo "Done."
