//! Security policy - sources and sinks
//!
//! New: YAML-based configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Protected source specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSpec {
    /// Glob pattern for protected paths
    pub pattern: String,
    /// Taint level for matches
    pub taint: String,
    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,
}

/// Exfiltration sink specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkSpec {
    /// Command name to block
    pub command: String,
    /// Block if conversation tainted
    pub block_if_tainted: bool,
    /// Human-readable reason
    #[serde(default)]
    pub reason: Option<String>,
}

/// Source policy loaded from sources.yaml
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourcePolicy {
    pub sources: Vec<SourceSpec>,
}

/// Sink policy loaded from sinks.yaml
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SinkPolicy {
    pub sinks: Vec<SinkSpec>,
}

impl SourcePolicy {
    /// Load from file
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let policy: Self = serde_yaml::from_str(&content)?;
        Ok(policy)
    }

    /// Check if path matches any protected pattern
    pub fn is_protected(&self, path: &PathBuf) -> bool {
        let path_str = path.to_string_lossy();
        self.sources.iter().any(|s| {
            glob_match::glob_match_with_cwd(&s.pattern, path_str.as_ref()).unwrap_or(false)
        })
    }
}

impl SinkPolicy {
    /// Load from file
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let policy: Self = serde_yaml::from_str(&content)?;
        Ok(policy)
    }

    /// Check if command is a sink
    pub fn is_sink(&self, cmd: &str) -> bool {
        self.sinks.iter().any(|s| cmd.starts_with(&s.command))
    }

    /// Get list of sink commands
    pub fn sink_commands(&self) -> Vec<&str> {
        self.sinks.iter().map(|s| s.command.as_str()).collect()
    }
}

// TODO: Implement glob matching properly
mod glob_match {
    pub fn glob_match_with_cwd(_pattern: &str, _path: &str) -> Option<bool> {
        // Placeholder - use glob crate properly
        Some(false)
    }
}
