//! Cursor SQLite reader (C-01)
//!
//! Port from: cursor.rs:156-239

use anyhow::Result;
use std::path::PathBuf;

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

// TODO: Implement C-01 fully
