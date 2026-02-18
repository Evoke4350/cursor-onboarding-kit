//! Bubble parsing for Cursor v0.40+ (C-03)
//!
//! Port from: cursor.rs:245-282

use serde::{Deserialize, Serialize};

/// Individual message bubble
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bubble {
    pub text: String,
    pub role: String,
    pub created_at: i64,
}

// TODO: Implement parse_bubbles()
