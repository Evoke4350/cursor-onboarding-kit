//! Normalized conversation representation (C-04)
//!
//! Unifies composerData and bubbleId formats into a single structure
//! for consistent consumption by the pipeline.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use super::composer::{ComposerEntry, get_all_composers};
use super::bubble::{BubbleEntry, get_all_bubbles, group_by_conversation};

/// Normalized message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message role: "user", "assistant", or "system"
    pub role: String,
    /// Message text content
    pub text: String,
    /// Unix timestamp (may be None for header-only entries)
    pub timestamp: Option<i64>,
    /// Source: "composer" or "bubble"
    pub source: String,
}

/// Normalized conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique conversation ID (UUID)
    pub id: String,
    /// Array of messages in chronological order
    pub messages: Vec<Message>,
    /// Unix timestamp of first message
    pub created_at: i64,
    /// Unix timestamp of last message
    pub last_updated_at: i64,
    /// Number of messages
    pub message_count: usize,
    /// Source: "composer", "bubbles", or "merged"
    pub source: String,
}

/// Extract conversations from composerData
pub fn from_composers(entries: Vec<ComposerEntry>) -> Vec<Conversation> {
    entries
        .into_iter()
        .map(|entry| {
            let message_count = entry.data.full_conversation_headers_only.len();
            Conversation {
                id: entry.id,
                messages: entry
                    .data
                    .full_conversation_headers_only
                    .into_iter()
                    .map(|h| Message {
                        role: h.role,
                        text: h.text,
                        timestamp: h.timestamp,
                        source: "composer".to_string(),
                    })
                    .collect(),
                created_at: entry.data.created_at,
                last_updated_at: entry.data.last_updated_at,
                message_count,
                source: "composer".to_string(),
            }
        })
        .collect()
}

/// Extract conversations from bubbles
pub fn from_bubbles(entries: Vec<BubbleEntry>) -> Vec<Conversation> {
    let grouped = group_by_conversation(entries);

    grouped
        .into_iter()
        .map(|(id, bubbles)| {
            let created_at = bubbles
                .first()
                .map(|b| b.data.created_at)
                .unwrap_or(0);
            let last_updated_at = bubbles
                .last()
                .map(|b| b.data.created_at)
                .unwrap_or(0);

            Conversation {
                id,
                messages: bubbles
                    .into_iter()
                    .map(|b| Message {
                        role: b.data.role,
                        text: b.data.text,
                        timestamp: Some(b.data.created_at),
                        source: "bubble".to_string(),
                    })
                    .collect(),
                created_at,
                last_updated_at,
                message_count: 0, // Updated below
                source: "bubbles".to_string(),
            }
        })
        .map(|mut c| {
            c.message_count = c.messages.len();
            c
        })
        .collect()
}

/// Get all conversations from database, preferring bubbles for v0.40+
pub fn get_all_conversations(conn: &Connection) -> Result<Vec<Conversation>> {
    // Try bubbles first (more detailed, v0.40+)
    let bubbles = get_all_bubbles(conn)?;
    if !bubbles.is_empty() {
        return Ok(from_bubbles(bubbles));
    }

    // Fall back to composerData (older format)
    let composers = get_all_composers(conn)?;
    Ok(from_composers(composers))
}

/// Merge composer and bubble data for a conversation
/// Bubbles have full content, composer has headers - prefer bubbles
pub fn merge_conversation(
    composer: Option<ComposerEntry>,
    bubbles: Vec<BubbleEntry>,
) -> Option<Conversation> {
    // Prefer bubbles if available (more detailed)
    if !bubbles.is_empty() {
        let id = bubbles
            .first()
            .map(|b| b.conversation_id.clone())
            .unwrap_or_default();

        let created_at = bubbles
            .first()
            .map(|b| b.data.created_at)
            .unwrap_or(0);
        let last_updated_at = bubbles
            .last()
            .map(|b| b.data.created_at)
            .unwrap_or(0);

        return Some(Conversation {
            id,
            messages: bubbles
                .into_iter()
                .map(|b| Message {
                    role: b.data.role,
                    text: b.data.text,
                    timestamp: Some(b.data.created_at),
                    source: "bubble".to_string(),
                })
                .collect(),
            created_at,
            last_updated_at,
            message_count: 0,
            source: "merged".to_string(),
        });
    }

    // Fall back to composer headers
    composer.map(|entry| {
        let message_count = entry.data.full_conversation_headers_only.len();
        Conversation {
            id: entry.id,
            messages: entry
                .data
                .full_conversation_headers_only
                .into_iter()
                .map(|h| Message {
                    role: h.role,
                    text: h.text,
                    timestamp: h.timestamp,
                    source: "composer".to_string(),
                })
                .collect(),
            created_at: entry.data.created_at,
            last_updated_at: entry.data.last_updated_at,
            message_count,
            source: "composer".to_string(),
        }
    })
}

/// Search conversations by content
pub fn search_conversations(conn: &Connection, query: &str) -> Result<Vec<Conversation>> {
    let conversations = get_all_conversations(conn)?;
    let query_lower = query.to_lowercase();

    let matches: Vec<Conversation> = conversations
        .into_iter()
        .filter(|conv| {
            conv.messages
                .iter()
                .any(|m| m.text.to_lowercase().contains(&query_lower))
        })
        .collect();

    Ok(matches)
}

/// Get a conversation by ID
pub fn get_conversation_by_id(conn: &Connection, id: &str) -> Result<Option<Conversation>> {
    let conversations = get_all_conversations(conn)?;

    Ok(conversations.into_iter().find(|c| c.id == id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::composer::ComposerData;
    use super::super::bubble::Bubble;

    #[test]
    fn test_from_composers() {
        let entry = ComposerEntry {
            key: "composerData:test-id".to_string(),
            id: "test-id".to_string(),
            data: ComposerData {
                full_conversation_headers_only: vec![
                    super::super::composer::MessageHeader {
                        role: "user".to_string(),
                        text: "Hello".to_string(),
                        timestamp: Some(1000),
                    },
                ],
                created_at: 1000,
                last_updated_at: 2000,
            },
        };

        let conversations = from_composers(vec![entry]);
        assert_eq!(conversations.len(), 1);
        assert_eq!(conversations[0].id, "test-id");
        assert_eq!(conversations[0].messages.len(), 1);
    }

    #[test]
    fn test_from_bubbles() {
        let entry = BubbleEntry {
            key: "bubbleId:conv1:msg1".to_string(),
            conversation_id: "conv1".to_string(),
            message_id: "msg1".to_string(),
            data: Bubble {
                role: "user".to_string(),
                text: "Test".to_string(),
                created_at: 1000,
                code_blocks: vec![],
            },
        };

        let conversations = from_bubbles(vec![entry]);
        assert_eq!(conversations.len(), 1);
        assert_eq!(conversations[0].id, "conv1");
        assert_eq!(conversations[0].messages.len(), 1);
    }
}
