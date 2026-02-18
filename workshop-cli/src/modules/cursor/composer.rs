//! Composer data parsing (C-02)
//!
//! Port from: cursor.rs:422-593

use serde::{Deserialize, Serialize};

/// Composer conversation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposerData {
    pub full_conversation_headers_only: Vec<MessageHeader>,
    pub created_at: i64,
    pub last_updated_at: i64,
}

/// Message header in composer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub role: String,
    pub content: String,
    pub timestamp: Option<i64>,
}

// TODO: Implement parse_composer_data()
