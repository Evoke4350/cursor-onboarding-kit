#!/usr/bin/env python3
"""
Cursor hook: beforeShellExecution
Calls workshop taint --hook-shell and blocks if tainted + sink.
"""
import sys
import json
import subprocess

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"allow": True}))
        sys.exit(0)

    command = sys.argv[1]

    try:
        result = subprocess.run(
            ["workshop", "taint", "--hook-shell", command],
            capture_output=True,
            text=True,
            timeout=5
        )

        if result.returncode == 0 and result.stdout.strip():
            output = json.loads(result.stdout.strip())
            # Pass through the allow decision
            print(json.dumps(output))
        else:
            # On error, allow but log
            print(json.dumps({"allow": True, "error": result.stderr}))

    except Exception as e:
        # On error, allow (fail open for usability)
        print(json.dumps({"allow": True, "error": str(e)}))

    sys.exit(0)

if __name__ == "__main__":
    main()
