//! Cursor command - Read Cursor SQLite (C-01)

use anyhow::Result;

pub fn run(list: bool, export: Option<String>, robot: bool) -> Result<()> {
    if robot {
        println!(r#"{{"status": "not_implemented", "command": "cursor"}}"#);
        return Ok(());
    }

    println!("🗄️  Cursor: SQLite integration");
    println!("   List: {}", list);
    println!("   Export: {:?}", export);
    println!();
    println!("TODO: Implement C-01 through C-04");
    Ok(())
}
