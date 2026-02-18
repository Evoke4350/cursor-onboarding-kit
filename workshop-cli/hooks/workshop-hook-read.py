#!/usr/bin/env python3
"""
Cursor hook: beforeFileRead
Calls workshop taint --hook-read and checks if file should be allowed.
"""
import sys
import json
import subprocess

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"allow": True}))
        sys.exit(0)

    filepath = sys.argv[1]

    try:
        result = subprocess.run(
            ["workshop", "taint", "--hook-read", filepath],
            capture_output=True,
            text=True,
            timeout=5
        )

        if result.returncode == 0 and result.stdout.strip():
            output = json.loads(result.stdout.strip())
            # If it's a source, mark it (but still allow read)
            if output.get("is_source"):
                # Mark as tainted for future checks
                subprocess.run(
                    ["workshop", "taint", "--mark", filepath],
                    capture_output=True,
                    timeout=5
                )
            print(json.dumps(output))
        else:
            # On error, allow but log
            print(json.dumps({"allow": True, "error": result.stderr}))

    except Exception as e:
        # On error, allow (fail open)
        print(json.dumps({"allow": True, "error": str(e)}))

    sys.exit(0)

if __name__ == "__main__":
    main()
