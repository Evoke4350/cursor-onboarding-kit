//! UserMemory schema (K-01)
//!
//! Port from: memory.py:8-58

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// User memory entry with Agno fields + Blackboard extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMemory {
    /// The memory content
    pub memory: String,

    /// Unique identifier
    #[serde(default)]
    pub memory_id: Option<String>,

    /// Topics/tags for categorization
    #[serde(default)]
    pub topics: Option<Vec<String>>,

    /// User ID this memory belongs to
    #[serde(default)]
    pub user_id: Option<String>,

    /// Creation timestamp
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Last update timestamp
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// Feedback on this memory
    #[serde(default)]
    pub feedback: Option<String>,

    /// Agent that created this memory
    #[serde(default)]
    pub agent_id: Option<String>,

    /// Team ID if applicable
    #[serde(default)]
    pub team_id: Option<String>,

    // Blackboard extensions
    /// Taint sources this memory references
    #[serde(default)]
    pub taint: Option<Vec<String>>,

    /// Code reference for this memory
    #[serde(default)]
    pub code_ref: Option<CodeReference>,
}

/// Code reference for linking memory to source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReference {
    pub file: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub commit: Option<String>,
}

impl UserMemory {
    /// Create a new memory with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            memory: content.into(),
            created_at: Some(chrono::Utc::now().timestamp()),
            ..Default::default()
        }
    }
}

impl Default for UserMemory {
    fn default() -> Self {
        Self {
            memory: String::new(),
            memory_id: Some(uuid::Uuid::new_v4().to_string()),
            topics: None,
            user_id: None,
            created_at: Some(chrono::Utc::now().timestamp()),
            updated_at: None,
            feedback: None,
            agent_id: None,
            team_id: None,
            taint: None,
            code_ref: None,
        }
    }
}
