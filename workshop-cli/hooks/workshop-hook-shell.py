#!/usr/bin/env python3
"""
Cursor hook: beforeShellExecution with taint checking
Integrates with workshop taint system to block exfiltration attempts.

Input (from stdin):
  {"command": "curl ...", "cwd": "/path/to/project"}

Output (to stdout):
  {"permission": "allow|deny", "continue": bool, "userMessage": "...", "agentMessage": "..."}
"""
import json
import os
import subprocess
import sys

def emit(payload):
    sys.stdout.write(json.dumps(payload))
    sys.stdout.flush()

def allow():
    emit({
        "permission": "allow",
        "continue": True,
        "userMessage": "",
        "agentMessage": "",
        "user_message": "",
        "agent_message": "",
    })

def deny(reason):
    emit({
        "permission": "deny",
        "continue": False,
        "userMessage": reason,
        "agentMessage": reason,
        "user_message": reason,
        "agent_message": reason,
    })

def main():
    try:
        payload = json.load(sys.stdin)
    except Exception:
        allow()
        return 0

    command = payload.get("command") or ""
    cwd = payload.get("cwd") or ""

    if not command:
        allow()
        return 0

    # Check taint status via workshop CLI
    try:
        proc = subprocess.run(
            ["workshop", "taint", "--hook-shell", command],
            capture_output=True,
            text=True,
            timeout=5,
            cwd=cwd if cwd else None
        )

        if proc.returncode == 0 and proc.stdout.strip():
            result = json.loads(proc.stdout.strip())

            if result.get("blocked"):
                reason = result.get("reason", "Blocked by taint analysis")
                level = result.get("taint_level", "unknown")
                deny(f"🛡️ {reason} (taint level: {level})")
                return 0

    except Exception as e:
        # On error, fail open (allow) for usability
        pass

    allow()
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
