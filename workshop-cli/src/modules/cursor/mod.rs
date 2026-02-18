//! Cursor Integration (C-01 through C-04)
//!
//! Source: ~/Dicklesworthstone/coding_agent_session_search/src/connectors/cursor.rs

pub mod sqlite;
pub mod composer;
pub mod bubble;

pub use sqlite::find_cursor_databases;
pub use composer::ComposerData;
pub use bubble::Bubble;
