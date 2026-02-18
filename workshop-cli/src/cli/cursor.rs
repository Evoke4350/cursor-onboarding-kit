//! Cursor command - Read Cursor SQLite (C-01)

use anyhow::Result;
use crate::modules::cursor::{sqlite, conversation};

pub fn run(list: bool, export: Option<String>, robot: bool) -> Result<()> {
    // Find and open database
    let db_path = match sqlite::primary_database() {
        Ok(path) => path,
        Err(_) => {
            if robot {
                let json = serde_json::json!({
                    "status": "error",
                    "error": "Cursor database not found"
                });
                println!("{}", serde_json::to_string(&json)?);
            } else {
                println!("❌ Cursor database not found");
                println!("   Expected at: ~/Library/Application Support/Cursor/User/globalStorage/state.vscdb");
            }
            return Ok(());
        }
    };

    let conn = sqlite::open_database(&db_path)?;

    if let Some(conversation_id) = export {
        export_conversation(&conn, &conversation_id, robot)
    } else if list {
        list_conversations(&conn, robot)
    } else {
        show_stats(&conn, robot)
    }
}

fn list_conversations(conn: &rusqlite::Connection, robot: bool) -> Result<()> {
    let conversations = conversation::get_all_conversations(conn)?;

    if robot {
        let items: Vec<serde_json::Value> = conversations
            .iter()
            .map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "message_count": c.message_count,
                    "created_at": c.created_at,
                    "source": c.source,
                })
            })
            .collect();

        let json = serde_json::json!({
            "status": "success",
            "command": "cursor",
            "count": conversations.len(),
            "conversations": items
        });
        println!("{}", serde_json::to_string(&json)?);
    } else {
        println!("🗄️  Cursor: {} conversations found", conversations.len());
        println!();

        for conv in conversations.iter().take(20) {
            let first_msg = conv.messages.first()
                .map(|m| if m.text.len() > 50 { format!("{}...", &m.text[..50]) } else { m.text.clone() })
                .unwrap_or_else(|| "No messages".to_string());

            println!("   {} ({} messages)", conv.id.split('-').next().unwrap_or(&conv.id), conv.message_count);
            println!("      {}", first_msg.replace('\n', " "));
        }

        if conversations.len() > 20 {
            println!();
            println!("   ... and {} more", conversations.len() - 20);
        }
    }

    Ok(())
}

fn export_conversation(conn: &rusqlite::Connection, id: &str, robot: bool) -> Result<()> {
    let conv = conversation::get_conversation_by_id(conn, id)?;

    match conv {
        Some(c) => {
            if robot {
                let json = serde_json::json!({
                    "status": "success",
                    "command": "cursor",
                    "conversation": c
                });
                println!("{}", serde_json::to_string(&json)?);
            } else {
                println!("🗄️  Conversation: {}", c.id);
                println!("   Messages: {}", c.message_count);
                println!("   Source: {}", c.source);
                println!();

                for msg in &c.messages {
                    let role_icon = match msg.role.as_str() {
                        "user" => "👤",
                        "assistant" => "🤖",
                        _ => "💬",
                    };
                    println!("{} [{}] {}", role_icon, msg.role, msg.text.lines().next().unwrap_or(""));
                }
            }
            Ok(())
        }
        None => {
            if robot {
                let json = serde_json::json!({
                    "status": "error",
                    "error": "Conversation not found"
                });
                println!("{}", serde_json::to_string(&json)?);
            } else {
                println!("❌ Conversation not found: {}", id);
            }
            Ok(())
        }
    }
}

fn show_stats(conn: &rusqlite::Connection, robot: bool) -> Result<()> {
    let tables = sqlite::list_tables(conn)?;
    let composer_keys = sqlite::get_composer_keys(conn)?;
    let bubble_keys = sqlite::get_bubble_keys(conn)?;

    if robot {
        let json = serde_json::json!({
            "status": "success",
            "command": "cursor",
            "stats": {
                "tables": tables,
                "composer_entries": composer_keys.len(),
                "bubble_entries": bubble_keys.len(),
            }
        });
        println!("{}", serde_json::to_string(&json)?);
    } else {
        println!("🗄️  Cursor SQLite Stats");
        println!();
        println!("   Tables: {}", tables.join(", "));
        println!("   Composer entries: {}", composer_keys.len());
        println!("   Bubble entries: {}", bubble_keys.len());
        println!();
        println!("   Use --list to see conversations");
        println!("   Use --export <id> to export a conversation");
    }

    Ok(())
}
