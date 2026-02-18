// Quint specification for TaintTracker state machine
// Formal spec that Rust implementation must satisfy

module taint {
  // Level constants
  val Clean: int = 0
  val Low: int = 1
  val Medium: int = 2
  val High: int = 3
  val Critical: int = 4

  // Pure functions for level ordering
  pure def maxLevel(a: int, b: int): int = if (a >= b) a else b

  // Invariant: level is valid
  pure def validLevel(level: int): bool =
    level >= Clean and level <= Critical

  // Test: level ordering
  run levelOrderTest = all {
    assert(maxLevel(Low, High) == High),
    assert(maxLevel(Critical, Clean) == Critical),
    assert(maxLevel(Medium, Medium) == Medium),
  }

  // Test: valid levels
  run validLevelTest = all {
    assert(validLevel(Clean)),
    assert(validLevel(Low)),
    assert(validLevel(Critical)),
    assert(not(validLevel(-1))),
    assert(not(validLevel(5))),
  }
}
