//! Carve command - Find connections (P-02)

use anyhow::Result;

pub fn run(query: String, domain: Option<String>, robot: bool) -> Result<()> {
    if robot {
        println!(r#"{{"status": "not_implemented", "command": "carve"}}"#);
        return Ok(());
    }

    println!("🔪 Carve: Search for '{}'", query);
    println!("   Domain: {:?}", domain);
    println!();
    println!("TODO: Implement P-02");
    Ok(())
}
