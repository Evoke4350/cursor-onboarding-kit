#!/usr/bin/env python3
"""
Reference implementation of TaintTracker for differential fuzzing.
This is the "oracle" that the Rust implementation must match.
"""

from enum import IntEnum
from typing import Dict, Set, Optional
from dataclasses import dataclass, field


class TaintLevel(IntEnum):
    CLEAN = 0
    LOW = 1
    MEDIUM = 2
    HIGH = 3
    CRITICAL = 4

    def __str__(self):
        return self.name.capitalize()


@dataclass
class TaintTracker:
    """Reference implementation matching the Quint spec."""
    tainted: Dict[str, TaintLevel] = field(default_factory=dict)

    def is_tainted(self) -> bool:
        return len(self.tainted) > 0

    def add_taint(self, path: str, level: TaintLevel) -> None:
        """Add taint - only increases, never decreases."""
        current = self.tainted.get(path, TaintLevel.CLEAN)
        self.tainted[path] = max(current, level, key=lambda x: int(x))

    def should_block(self, cmd: str, sinks: Set[str]) -> bool:
        """Block if tainted and command starts with a sink."""
        return self.is_tainted() and any(cmd.startswith(s) for s in sinks)

    def max_taint_level(self) -> TaintLevel:
        """Get maximum taint level across all paths."""
        if not self.tainted:
            return TaintLevel.CLEAN
        return max(self.tainted.values(), key=lambda x: int(x))

    def tainted_paths(self) -> Set[str]:
        """Get all tainted paths."""
        return set(self.tainted.keys())

    def to_json(self) -> dict:
        """Export state for comparison with Rust."""
        return {
            "is_tainted": self.is_tainted(),
            "max_taint_level": int(self.max_taint_level()),
            "tainted_paths": sorted(self.tainted_paths()),
            "taint_map": {k: int(v) for k, v in sorted(self.tainted.items())}
        }


def run_operations(ops: list) -> dict:
    """
    Run a sequence of operations and return final state.
    Used for differential testing against Rust.

    ops format: [{"op": "add_taint", "path": "...", "level": N}, ...]
    """
    tracker = TaintTracker()

    for op in ops:
        op_type = op["op"]

        if op_type == "add_taint":
            level = TaintLevel(op["level"])
            tracker.add_taint(op["path"], level)

        elif op_type == "should_block":
            # Just record the result
            pass

        elif op_type == "reset":
            tracker = TaintTracker()

    return tracker.to_json()


if __name__ == "__main__":
    import json
    import sys

    # Read operations from stdin
    ops = json.load(sys.stdin)
    result = run_operations(ops)
    print(json.dumps(result))
