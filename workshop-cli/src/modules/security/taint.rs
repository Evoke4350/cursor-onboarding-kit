//! Taint tracking engine
//!
//! Concept from Universalis doc + ~/agno/libs/agno/agno/guardrails/pii.py

use std::collections::BTreeMap;
use std::path::PathBuf;

/// Taint level with explicit ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TaintLevel {
    #[default]
    Clean = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Taint state tracker - immutable for safety
#[derive(Debug, Clone, Default)]
pub struct TaintTracker {
    /// Currently tainted paths with their levels
    tainted: BTreeMap<PathBuf, TaintLevel>,
}

impl TaintTracker {
    /// Create a new clean tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any tainted data exists
    pub fn is_tainted(&self) -> bool {
        !self.tainted.is_empty()
    }

    /// Add taint to a path (only increases, never decreases)
    pub fn add_taint(&mut self, path: PathBuf, level: TaintLevel) {
        self.tainted
            .entry(path)
            .and_modify(|existing| *existing = (*existing).max(level))
            .or_insert(level);
    }

    /// Check if a command should be blocked
    pub fn should_block(&self, cmd: &str, sinks: &[&str]) -> bool {
        self.is_tainted() && sinks.iter().any(|s| cmd.starts_with(s))
    }

    /// Get current taint level
    pub fn max_taint_level(&self) -> TaintLevel {
        self.tainted.values().copied().max().unwrap_or(TaintLevel::Clean)
    }

    /// Get all tainted paths
    pub fn tainted_paths(&self) -> Vec<&PathBuf> {
        self.tainted.keys().collect()
    }
}

// TODO: Implement full taint propagation logic

#[cfg(test)]
mod tests {
    use super::*;

    // Property-based tests with proptest
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_taint_level_ordering(a in 0usize..5, b in 0usize..5) {
                let level_a = match a {
                    0 => TaintLevel::Clean,
                    1 => TaintLevel::Low,
                    2 => TaintLevel::Medium,
                    3 => TaintLevel::High,
                    _ => TaintLevel::Critical,
                };
                let level_b = match b {
                    0 => TaintLevel::Clean,
                    1 => TaintLevel::Low,
                    2 => TaintLevel::Medium,
                    3 => TaintLevel::High,
                    _ => TaintLevel::Critical,
                };
                // Ordering should be consistent
                prop_assert_eq!(level_a <= level_b, a <= b);
                prop_assert_eq!(level_a >= level_b, a >= b);
            }

            #[test]
            fn prop_add_taint_monotonic(path in "[a-z]+", level_idx in 0usize..5) {
                let mut tracker = TaintTracker::new();
                let level = match level_idx {
                    0 => TaintLevel::Clean,
                    1 => TaintLevel::Low,
                    2 => TaintLevel::Medium,
                    3 => TaintLevel::High,
                    _ => TaintLevel::Critical,
                };

                tracker.add_taint(PathBuf::from(&path), level);
                let initial = tracker.max_taint_level();

                // Adding lower or equal level shouldn't decrease
                tracker.add_taint(PathBuf::from(&path), TaintLevel::Low);
                prop_assert!(tracker.max_taint_level() >= initial);
            }

            #[test]
            fn prop_max_taint_is_max(paths in prop::collection::vec("[a-z]+", 1..10), levels in prop::collection::vec(0usize..5, 1..10)) {
                let mut tracker = TaintTracker::new();
                let mut max_level = TaintLevel::Clean;

                for (path, level_idx) in paths.iter().zip(levels.iter()) {
                    let level = match level_idx {
                        0 => TaintLevel::Clean,
                        1 => TaintLevel::Low,
                        2 => TaintLevel::Medium,
                        3 => TaintLevel::High,
                        _ => TaintLevel::Critical,
                    };
                    tracker.add_taint(PathBuf::from(path), level);
                    max_level = max_level.max(level);
                }

                // Tracker max should match what we actually added
                prop_assert_eq!(tracker.max_taint_level(), max_level);
            }

            #[test]
            fn prop_should_block_requires_taint_and_sink(cmd in "[a-z]+", has_taint: bool, sinks in prop::collection::vec("[a-z]+", 0..3)) {
                let mut tracker = TaintTracker::new();
                if has_taint {
                    tracker.add_taint(PathBuf::from("tainted"), TaintLevel::Low);
                }

                let sink_refs: Vec<&str> = sinks.iter().map(|s| s.as_str()).collect();
                let result = tracker.should_block(&cmd, &sink_refs);

                // Should only block if tainted AND command matches a sink
                let matches_sink = sinks.iter().any(|s| cmd.starts_with(s));
                prop_assert_eq!(result, has_taint && matches_sink);
            }
        }
    }
}
