//! Taint tracking engine
//!
//! Concept from Universalis doc + ~/agno/libs/agno/agno/guardrails/pii.py

use std::collections::BTreeMap;
use std::path::PathBuf;
use anyhow::Result;

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
