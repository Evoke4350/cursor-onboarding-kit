//! Cursor SQLite reader (C-01)
//!
//! Port from: cursor.rs:156-239

use anyhow::Result;
use rusqlite::{Connection, OpenFlags, params};
use std::path::PathBuf;
use std::time::Duration;

/// Find all Cursor SQLite databases
pub fn find_cursor_databases() -> Result<Vec<PathBuf>> {
    let base = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?
        .join("Cursor/User/globalStorage");

    let mut dbs = Vec::new();

    if !base.exists() {
        return Ok(dbs);
    }

    for entry in std::fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        let state_db = path.join("state.vscdb");

        if state_db.exists() {
            dbs.push(state_db);
        }
    }

    Ok(dbs)
}

/// Primary Cursor database path
pub fn primary_database() -> Result<PathBuf> {
    let base = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?
        .join("Cursor/User/globalStorage/state.vscdb");

    if base.exists() {
        Ok(base)
    } else {
        Err(anyhow::anyhow!("Cursor database not found at {:?}", base))
    }
}

/// Open Cursor database read-only with busy timeout
pub fn open_database(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;

    // Set busy timeout to handle Cursor running
    conn.busy_timeout(Duration::from_millis(5000))?;

    Ok(conn)
}

/// List all tables in the database
pub fn list_tables(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )?;

    let tables: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(tables)
}

/// Check if a table exists
pub fn table_exists(conn: &Connection, table: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
        params![table],
        |row| row.get(0),
    )?;

    Ok(count > 0)
}

/// Get all keys from cursorDiskKV table
pub fn get_all_keys(conn: &Connection) -> Result<Vec<String>> {
    if !table_exists(conn, "cursorDiskKV")? {
        return Ok(Vec::new());
    }

    let mut stmt = conn.prepare("SELECT key FROM cursorDiskKV")?;
    let keys: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(keys)
}

/// Get value for a specific key from cursorDiskKV
pub fn get_value(conn: &Connection, key: &str) -> Result<Option<String>> {
    if !table_exists(conn, "cursorDiskKV")? {
        return Ok(None);
    }

    let result = conn.query_row(
        "SELECT value FROM cursorDiskKV WHERE key = ?",
        params![key],
        |row| row.get(0),
    );

    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Get all composer data keys
pub fn get_composer_keys(conn: &Connection) -> Result<Vec<String>> {
    let keys = get_all_keys(conn)?;
    let composer_keys: Vec<String> = keys
        .into_iter()
        .filter(|k| k.starts_with("composerData:"))
        .collect();

    Ok(composer_keys)
}

/// Get all bubble keys
pub fn get_bubble_keys(conn: &Connection) -> Result<Vec<String>> {
    let keys = get_all_keys(conn)?;
    let bubble_keys: Vec<String> = keys
        .into_iter()
        .filter(|k| k.starts_with("bubbleId:"))
        .collect();

    Ok(bubble_keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_database_path() {
        // This test checks if the path construction works
        let result = primary_database();
        // Will fail if Cursor not installed, which is fine
        if let Ok(path) = result {
            assert!(path.to_string_lossy().contains("Cursor"));
        }
    }

    #[test]
    fn test_find_cursor_databases() {
        let result = find_cursor_databases();
        assert!(result.is_ok());
    }
}
