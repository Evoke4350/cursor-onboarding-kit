//! Composer data parsing (C-02)
//!
//! Port from: cursor.rs:422-593
//!
//! composerData keys in cursorDiskKV contain JSON with conversation history.
//! Format: composerData:<uuid> -> JSON with full_conversation_headers_only

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use super::sqlite::{get_value, table_exists};

/// Composer conversation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposerData {
    /// Array of message headers (summaries, not full content)
    pub full_conversation_headers_only: Vec<MessageHeader>,
    /// Unix timestamp of conversation creation
    pub created_at: i64,
    /// Unix timestamp of last update
    pub last_updated_at: i64,
}

/// Message header in composer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Role: "user", "assistant", or "system"
    pub role: String,
    /// Text content (truncated in headers)
    pub text: String,
    /// Unix timestamp (optional, may not be present)
    #[serde(default)]
    pub timestamp: Option<i64>,
}

/// Parsed composer entry with key metadata
#[derive(Debug, Clone)]
pub struct ComposerEntry {
    /// Original key from cursorDiskKV
    pub key: String,
    /// Composer ID (UUID portion)
    pub id: String,
    /// Parsed data
    pub data: ComposerData,
}

/// Parse a single composerData value from JSON
pub fn parse_composer_data(json: &str) -> Result<ComposerData> {
    // Cursor may store prefixed JSON like "composerData:..." or raw JSON
    let json_str = json.trim();

    // Handle potential prefix wrapper
    let actual_json = if json_str.starts_with('{') {
        json_str
    } else if let Some(pos) = json_str.find('{') {
        &json_str[pos..]
    } else {
        bail!("Invalid composerData format: no JSON object found");
    };

    let data: ComposerData = serde_json::from_str(actual_json)?;

    // Validate essential fields
    if data.full_conversation_headers_only.is_empty() {
        // Empty conversation is valid but worth noting
    }

    Ok(data)
}

/// Extract composer ID from key (format: composerData:<uuid>)
pub fn extract_composer_id(key: &str) -> Option<String> {
    key.strip_prefix("composerData:")
        .map(|s| s.to_string())
}

/// Get a specific composer entry by key
pub fn get_composer(conn: &Connection, key: &str) -> Result<Option<ComposerEntry>> {
    if !table_exists(conn, "cursorDiskKV")? {
        return Ok(None);
    }

    let value = match get_value(conn, key)? {
        Some(v) => v,
        None => return Ok(None),
    };

    let data = parse_composer_data(&value)?;
    let id = extract_composer_id(key)
        .unwrap_or_else(|| "unknown".to_string());

    Ok(Some(ComposerEntry {
        key: key.to_string(),
        id,
        data,
    }))
}

/// Get all composer entries from the database
pub fn get_all_composers(conn: &Connection) -> Result<Vec<ComposerEntry>> {
    use super::sqlite::get_composer_keys;

    let keys = get_composer_keys(conn)?;
    let mut entries = Vec::new();

    for key in keys {
        match get_composer(conn, &key) {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => continue,
            Err(e) => {
                // Log but continue - one malformed entry shouldn't fail the batch
                eprintln!("Warning: Failed to parse {}: {}", key, e);
            }
        }
    }

    // Sort by creation time, newest first
    entries.sort_by(|a, b| b.data.created_at.cmp(&a.data.created_at));

    Ok(entries)
}

/// Find composers containing a search term
pub fn search_composers(conn: &Connection, query: &str) -> Result<Vec<ComposerEntry>> {
    let entries = get_all_composers(conn)?;
    let query_lower = query.to_lowercase();

    let matches: Vec<ComposerEntry> = entries
        .into_iter()
        .filter(|entry| {
            entry.data.full_conversation_headers_only
                .iter()
                .any(|msg| msg.text.to_lowercase().contains(&query_lower))
        })
        .collect();

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_composer_data() {
        let json = r#"{
            "full_conversation_headers_only": [
                {"role": "user", "text": "Hello", "timestamp": 1708300000},
                {"role": "assistant", "text": "Hi there!", "timestamp": 1708300001}
            ],
            "created_at": 1708300000,
            "last_updated_at": 1708300100
        }"#;

        let data = parse_composer_data(json).unwrap();
        assert_eq!(data.full_conversation_headers_only.len(), 2);
        assert_eq!(data.full_conversation_headers_only[0].role, "user");
        assert_eq!(data.created_at, 1708300000);
    }

    #[test]
    fn test_extract_composer_id() {
        assert_eq!(
            extract_composer_id("composerData:abc-123-def"),
            Some("abc-123-def".to_string())
        );
        assert_eq!(extract_composer_id("other:key"), None);
    }

    #[test]
    fn test_parse_with_missing_optional() {
        // Timestamp is optional
        let json = r#"{
            "full_conversation_headers_only": [
                {"role": "user", "text": "Test"}
            ],
            "created_at": 1708300000,
            "last_updated_at": 1708300100
        }"#;

        let data = parse_composer_data(json).unwrap();
        assert_eq!(data.full_conversation_headers_only[0].timestamp, None);
    }
}
