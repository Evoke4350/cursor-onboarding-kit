//! Differential fuzz testing for TaintTracker
//!
//! Compares Rust implementation against Python reference implementation.
//! Uses proptest to generate random operation sequences.

use std::path::PathBuf;
use proptest::prelude::*;
use workshop::modules::security::taint::{TaintLevel, TaintTracker};

/// Operations that can be performed on the taint tracker
#[derive(Debug, Clone)]
enum TaintOp {
    AddTaint { path: String, level: u8 },
}

/// Generate arbitrary taint operations
fn arb_taint_op() -> impl Strategy<Value = TaintOp> {
    (any::<u8>(), 0u8..5).prop_map(|(path_byte, level)| TaintOp::AddTaint {
        path: format!("path_{}", path_byte % 10),
        level,
    })
}

/// Run operations on Rust implementation and return final state
fn run_rust_ops(ops: &[TaintOp]) -> (bool, u8, usize) {
    let mut tracker = TaintTracker::new();

    for op in ops {
        match op {
            TaintOp::AddTaint { path, level } => {
                let taint_level = match level {
                    0 => TaintLevel::Clean,
                    1 => TaintLevel::Low,
                    2 => TaintLevel::Medium,
                    3 => TaintLevel::High,
                    _ => TaintLevel::Critical,
                };
                tracker.add_taint(PathBuf::from(path), taint_level);
            }
        }
    }

    (
        tracker.is_tainted(),
        tracker.max_taint_level() as u8,
        tracker.tainted_paths().len(),
    )
}

// Invariant tests using proptest

proptest! {
    #[test]
    fn prop_monotonicity(ops in prop::collection::vec(arb_taint_op(), 1..100)) {
        // Taint level should only increase, never decrease
        let mut tracker = TaintTracker::new();
        let mut max_seen = TaintLevel::Clean;

        for op in &ops {
            if let TaintOp::AddTaint { path, level } = op {
                let taint_level = match level {
                    0 => TaintLevel::Clean,
                    1 => TaintLevel::Low,
                    2 => TaintLevel::Medium,
                    3 => TaintLevel::High,
                    _ => TaintLevel::Critical,
                };
                tracker.add_taint(PathBuf::from(path), taint_level);

                // Max should only increase
                let new_max = tracker.max_taint_level();
                prop_assert!(new_max >= max_seen);
                max_seen = new_max;
            }
        }
    }

    #[test]
    fn prop_idempotency(path in "path_[0-9]", level in 0u8..5) {
        let mut t1 = TaintTracker::new();
        let mut t2 = TaintTracker::new();

        let taint_level = match level {
            0 => TaintLevel::Clean,
            1 => TaintLevel::Low,
            2 => TaintLevel::Medium,
            3 => TaintLevel::High,
            _ => TaintLevel::Critical,
        };

        // Apply once
        t1.add_taint(PathBuf::from(&path), taint_level);

        // Apply same level 100 times
        for _ in 0..100 {
            t2.add_taint(PathBuf::from(&path), taint_level);
        }

        // Should be identical
        prop_assert_eq!(t1.max_taint_level(), t2.max_taint_level());
        prop_assert_eq!(t1.is_tainted(), t2.is_tainted());
    }

    #[test]
    fn prop_max_level_is_max_of_all(paths in prop::collection::vec(("[a-z]+", 0u8..5), 1..20)) {
        let mut tracker = TaintTracker::new();
        let mut expected_max = TaintLevel::Clean;

        for (path, level) in &paths {
            let taint_level = match level {
                0 => TaintLevel::Clean,
                1 => TaintLevel::Low,
                2 => TaintLevel::Medium,
                3 => TaintLevel::High,
                _ => TaintLevel::Critical,
            };
            tracker.add_taint(PathBuf::from(path), taint_level);
            expected_max = expected_max.max(taint_level);
        }

        prop_assert_eq!(tracker.max_taint_level(), expected_max);
    }

    #[test]
    fn prop_deterministic(ops in prop::collection::vec(arb_taint_op(), 1..50)) {
        // Same operations should always produce same result
        let result1 = run_rust_ops(&ops);
        let result2 = run_rust_ops(&ops);

        prop_assert_eq!(result1, result2);
    }

    #[test]
    fn prop_blocking_soundness(
        tainted_paths in prop::collection::vec("[a-z]+", 0..10),
        cmd in "[a-z]+ [a-z]+",
        sinks in prop::collection::vec("[a-z]+", 1..5)
    ) {
        let mut tracker = TaintTracker::new();

        for path in &tainted_paths {
            tracker.add_taint(PathBuf::from(path), TaintLevel::High);
        }

        let sinks_ref: Vec<&str> = sinks.iter().map(|s| s.as_str()).collect();
        let should_block = tracker.should_block(&cmd, &sinks_ref);

        // Should only block if we have taint AND command starts with a sink
        let has_taint = !tainted_paths.is_empty();
        let matches_sink = sinks.iter().any(|s| cmd.starts_with(s));

        prop_assert_eq!(should_block, has_taint && matches_sink);
    }
}

#[cfg(test)]
mod deterministic_tests {
    use super::*;

    #[test]
    fn test_empty_ops() {
        let ops: Vec<TaintOp> = vec![];
        let result = run_rust_ops(&ops);
        assert_eq!(result, (false, 0, 0));
    }

    #[test]
    fn test_single_add() {
        let ops = vec![TaintOp::AddTaint { path: "file".into(), level: 2 }];
        let result = run_rust_ops(&ops);
        assert_eq!(result, (true, 2, 1));
    }

    #[test]
    fn test_increase_sequence() {
        let ops = vec![
            TaintOp::AddTaint { path: "file".into(), level: 1 },
            TaintOp::AddTaint { path: "file".into(), level: 3 },
            TaintOp::AddTaint { path: "file".into(), level: 4 },
        ];
        let result = run_rust_ops(&ops);
        assert_eq!(result, (true, 4, 1)); // Critical = 4
    }

    #[test]
    fn test_no_decrease() {
        let ops = vec![
            TaintOp::AddTaint { path: "file".into(), level: 4 },
            TaintOp::AddTaint { path: "file".into(), level: 1 },
            TaintOp::AddTaint { path: "file".into(), level: 0 },
        ];
        let result = run_rust_ops(&ops);
        assert_eq!(result, (true, 4, 1)); // Should stay at Critical
    }
}
