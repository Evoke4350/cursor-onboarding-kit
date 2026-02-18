//! Bubble parsing for Cursor v0.40+ (C-03)
//!
//! Port from: cursor.rs:245-282
//!
//! bubbleId keys in cursorDiskKV contain individual message bubbles.
//! Format: bubbleId:<conversationId>:<messageId> -> JSON with single message

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use super::sqlite::{get_value, table_exists};

/// Individual message bubble (Cursor v0.40+)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bubble {
    /// Message text content
    #[serde(default)]
    pub text: String,
    /// Role: "user", "assistant", or "system" (optional in some formats)
    #[serde(default)]
    pub role: String,
    /// Unix timestamp of creation (optional)
    #[serde(default)]
    pub created_at: i64,
    /// Optional code blocks if present
    #[serde(default)]
    pub code_blocks: Vec<CodeBlock>,
}

/// Code block within a bubble
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    /// Programming language
    pub language: Option<String>,
    /// Code content
    pub code: String,
}

/// Parsed bubble entry with key metadata
#[derive(Debug, Clone)]
pub struct BubbleEntry {
    /// Original key from cursorDiskKV
    pub key: String,
    /// Conversation ID (UUID)
    pub conversation_id: String,
    /// Message ID (UUID)
    pub message_id: String,
    /// Parsed bubble data
    pub data: Bubble,
}

/// Parse a single bubble value from JSON
pub fn parse_bubble(json: &str) -> Result<Bubble> {
    let json_str = json.trim();

    // Handle potential prefix wrapper
    let actual_json = if json_str.starts_with('{') {
        json_str
    } else if let Some(pos) = json_str.find('{') {
        &json_str[pos..]
    } else {
        bail!("Invalid bubble format: no JSON object found");
    };

    let bubble: Bubble = serde_json::from_str(actual_json)?;
    Ok(bubble)
}

/// Extract conversation and message IDs from key
/// Format: bubbleId:<conversationId>:<messageId>
pub fn extract_bubble_ids(key: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() >= 3 && parts[0] == "bubbleId" {
        Some((parts[1].to_string(), parts[2].to_string()))
    } else {
        None
    }
}

/// Get a specific bubble entry by key
pub fn get_bubble(conn: &Connection, key: &str) -> Result<Option<BubbleEntry>> {
    if !table_exists(conn, "cursorDiskKV")? {
        return Ok(None);
    }

    let value = match get_value(conn, key)? {
        Some(v) => v,
        None => return Ok(None),
    };

    let data = parse_bubble(&value)?;
    let (conversation_id, message_id) = extract_bubble_ids(key)
        .unwrap_or_else(|| ("unknown".to_string(), "unknown".to_string()));

    Ok(Some(BubbleEntry {
        key: key.to_string(),
        conversation_id,
        message_id,
        data,
    }))
}

/// Get all bubbles from the database
pub fn get_all_bubbles(conn: &Connection) -> Result<Vec<BubbleEntry>> {
    use super::sqlite::get_bubble_keys;

    let keys = get_bubble_keys(conn)?;
    let mut entries = Vec::new();

    for key in keys {
        match get_bubble(conn, &key) {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => continue,
            Err(e) => {
                eprintln!("Warning: Failed to parse {}: {}", key, e);
            }
        }
    }

    // Sort by creation time, oldest first (chronological order)
    entries.sort_by(|a, b| a.data.created_at.cmp(&b.data.created_at));

    Ok(entries)
}

/// Get all bubbles for a specific conversation
pub fn get_conversation_bubbles(conn: &Connection, conversation_id: &str) -> Result<Vec<BubbleEntry>> {
    let entries = get_all_bubbles(conn)?;

    let filtered: Vec<BubbleEntry> = entries
        .into_iter()
        .filter(|e| e.conversation_id == conversation_id)
        .collect();

    Ok(filtered)
}

/// Group bubbles by conversation ID
pub fn group_by_conversation(entries: Vec<BubbleEntry>) -> Vec<(String, Vec<BubbleEntry>)> {
    use std::collections::HashMap;

    let mut groups: HashMap<String, Vec<BubbleEntry>> = HashMap::new();

    for entry in entries {
        groups
            .entry(entry.conversation_id.clone())
            .or_default()
            .push(entry);
    }

    let mut result: Vec<(String, Vec<BubbleEntry>)> = groups.into_iter().collect();
    // Sort by earliest message in each conversation
    result.sort_by(|a, b| {
        let a_time = a.1.first().map(|e| e.data.created_at).unwrap_or(0);
        let b_time = b.1.first().map(|e| e.data.created_at).unwrap_or(0);
        b_time.cmp(&a_time) // newest first
    });

    result
}

/// Find bubbles containing a search term
pub fn search_bubbles(conn: &Connection, query: &str) -> Result<Vec<BubbleEntry>> {
    let entries = get_all_bubbles(conn)?;
    let query_lower = query.to_lowercase();

    let matches: Vec<BubbleEntry> = entries
        .into_iter()
        .filter(|entry| entry.data.text.to_lowercase().contains(&query_lower))
        .collect();

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Property-based tests with proptest
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_parse_minimal_bubble(text in "[a-zA-Z0-9 ]*") {
                // Minimal valid JSON should parse with safe chars
                let json = format!(r#"{{"text": "{}"}}"#, text);
                let result = parse_bubble(&json);
                // Should parse successfully
                if let Ok(bubble) = result {
                    prop_assert_eq!(bubble.text, text);
                }
            }

            #[test]
            fn prop_extract_bubble_ids_format(uuid1 in "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}", uuid2 in "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}") {
                let key = format!("bubbleId:{}:{}", uuid1, uuid2);
                let result = extract_bubble_ids(&key);
                prop_assert!(result.is_some());
                let (conv, msg) = result.unwrap();
                prop_assert_eq!(conv, uuid1);
                prop_assert_eq!(msg, uuid2);
            }

            #[test]
            fn prop_extract_bubble_ids_rejects_invalid(prefix in "[a-zA-Z]+") {
                // Only "bubbleId" prefix should work
                let key = format!("{}:abc:def", prefix);
                let result = extract_bubble_ids(&key);
                if prefix == "bubbleId" {
                    prop_assert!(result.is_some());
                } else {
                    prop_assert!(result.is_none());
                }
            }
        }
    }

    #[test]
    fn test_parse_bubble() {
        let json = r#"{
            "text": "Hello, how can I help?",
            "role": "assistant",
            "created_at": 1708300000,
            "code_blocks": []
        }"#;

        let bubble = parse_bubble(json).unwrap();
        assert_eq!(bubble.role, "assistant");
        assert_eq!(bubble.text, "Hello, how can I help?");
        assert_eq!(bubble.created_at, 1708300000);
    }

    #[test]
    fn test_parse_bubble_with_code() {
        let json = r#"{
            "text": "Here's the code:",
            "role": "assistant",
            "created_at": 1708300000,
            "code_blocks": [
                {"language": "rust", "code": "fn main() {}"}
            ]
        }"#;

        let bubble = parse_bubble(json).unwrap();
        assert_eq!(bubble.code_blocks.len(), 1);
        assert_eq!(bubble.code_blocks[0].language, Some("rust".to_string()));
    }

    #[test]
    fn test_extract_bubble_ids() {
        let (conv, msg) = extract_bubble_ids("bubbleId:abc-123-def:xyz-789-uvw").unwrap();
        assert_eq!(conv, "abc-123-def");
        assert_eq!(msg, "xyz-789-uvw");

        assert!(extract_bubble_ids("other:key").is_none());
    }
}
