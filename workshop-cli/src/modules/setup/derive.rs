//! Configuration derivation (O-03)
//!
//! Port from: claude-md.md:116-125

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Work type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WorkType {
    #[default]
    Software,
    Research,
    Writing,
}

/// Code reference mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CodeRefMode {
    #[default]
    Optional,
    Required,
    Disabled,
}

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    #[default]
    Basic,
    None,
    Paranoid,
}

/// Derived configuration from user answers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DerivedConfig {
    pub work_type: WorkType,
    pub code_refs: CodeRefMode,
    pub location: PathBuf,
    pub security_level: SecurityLevel,
}

impl DerivedConfig {
    /// Derive configuration from question answers
    pub fn from_answers(answers: std::collections::HashMap<String, String>) -> Self {
        let mut config = Self::default();

        if let Some(wt) = answers.get("work_type") {
            config.work_type = match wt.to_lowercase().as_str() {
                "research" => WorkType::Research,
                "writing" => WorkType::Writing,
                _ => WorkType::Software,
            };
        }

        if let Some(cr) = answers.get("code_refs") {
            config.code_refs = match cr.to_lowercase().as_str() {
                "required" | "yes" => CodeRefMode::Required,
                "disabled" | "no" => CodeRefMode::Disabled,
                _ => CodeRefMode::Optional,
            };
        }

        if let Some(loc) = answers.get("location") {
            config.location = PathBuf::from(loc);
        }

        if let Some(sec) = answers.get("security") {
            config.security_level = match sec.to_lowercase().as_str() {
                "none" => SecurityLevel::None,
                "paranoid" => SecurityLevel::Paranoid,
                _ => SecurityLevel::Basic,
            };
        }

        config
    }
}
