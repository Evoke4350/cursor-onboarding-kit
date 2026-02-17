#!/usr/bin/env bash
set -euo pipefail

# Bonus helper to inspect local "YOLO-like" settings without mutating anything.
# Keep this as a diagnostic script, not a team default policy.

CONFIG_PATH="${1:-$HOME/.cursor/cli-config.json}"

echo "== Cursor YOLO Troubleshoot (read-only) =="
echo "Config: $CONFIG_PATH"
echo

if [[ ! -f "$CONFIG_PATH" ]]; then
  echo "No CLI config found at $CONFIG_PATH"
  echo "Tip: run Cursor Agent CLI once to generate it, then rerun this script."
  exit 0
fi

python3 - "$CONFIG_PATH" <<'PY'
import json
import sys
from pathlib import Path

config_path = Path(sys.argv[1])
data = json.loads(config_path.read_text())

approval_mode = data.get("approvalMode", "(unset)")
sandbox = data.get("sandbox", {})
sandbox_mode = sandbox.get("mode", "(unset)")
network_access = sandbox.get("networkAccess", "(unset)")

allow_rules = data.get("permissions", {}).get("allow", [])
deny_rules = data.get("permissions", {}).get("deny", [])

print("Current posture")
print("--------------")
print(f"approvalMode: {approval_mode}")
print(f"sandbox.mode: {sandbox_mode}")
print(f"sandbox.networkAccess: {network_access}")
print(f"allow rules: {len(allow_rules)}")
print(f"deny rules: {len(deny_rules)}")
print()

risk_flags = []
if sandbox_mode == "disabled":
    risk_flags.append("sandbox is disabled")
if approval_mode in {"yolo", "never", "auto", "off"}:
    risk_flags.append(f"approval mode looks high-autonomy: {approval_mode}")
if any("Shell(*)" in r or "Shell(" in r and "*" in r for r in allow_rules):
    risk_flags.append("broad shell allow rule detected")

print("Risk signals")
print("------------")
if risk_flags:
    for item in risk_flags:
        print(f"- {item}")
else:
    print("- none detected from basic checks")
print()

print("Suggested profiles (copy manually, do not auto-apply)")
print("------------------------------------------------------")
print("Strict:")
print('  approvalMode = "default"')
print('  sandbox.mode = "enabled"')
print('  sandbox.networkAccess = "deny"')
print()
print("Balanced:")
print('  approvalMode = "allowlist"')
print('  sandbox.mode = "enabled"')
print('  sandbox.networkAccess = "allowlist"')
print()
print("YOLO-Lab-Only (disposable repos):")
print('  approvalMode = "allowlist" or broader only if isolated')
print('  sandbox.mode = "enabled" where possible')
print('  branch/repo isolation + rollback required')
PY

echo
echo "Optional local checks:"
echo "- Inspect shell aliases/functions for high-autonomy shortcuts"
echo "- Search for '--dangerously-skip-permissions' in your dotfiles"
echo "- Keep YOLO shortcuts local-only; do not commit to team scripts"
