#!/usr/bin/env python3
"""
Cursor hook: beforeFileRead with taint checking
Marks sensitive files as tainted when read.

Input (from stdin):
  {"path": "/path/to/file", "cwd": "/project/root"}

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

    filepath = payload.get("path") or payload.get("file_path") or ""
    cwd = payload.get("cwd") or ""

    if not filepath:
        allow()
        return 0

    # Check taint status via workshop CLI
    try:
        proc = subprocess.run(
            ["workshop", "taint", "--hook-read", filepath],
            capture_output=True,
            text=True,
            timeout=5,
            cwd=cwd if cwd else None
        )

        if proc.returncode == 0 and proc.stdout.strip():
            result = json.loads(proc.stdout.strip())

            # If it's a source, mark it tainted (but still allow read)
            if result.get("is_source"):
                level = result.get("level", "unknown")
                # Auto-mark as tainted
                subprocess.run(
                    ["workshop", "taint", "--mark", filepath],
                    capture_output=True,
                    timeout=5,
                    cwd=cwd if cwd else None
                )
                # Allow the read but note it's tainted
                allow()
                return 0

            # Not a source, just allow
            allow()
            return 0

    except Exception as e:
        # On error, fail open (allow) for usability
        pass

    allow()
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
