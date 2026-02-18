//! Chamfer command - Update older work (P-03)

use anyhow::{Result, bail};
use std::path::PathBuf;
use crate::modules::pipeline::{ChamferResult, ChamferConfig, update_shaving};

pub fn run(shaving: String, context: String, robot: bool) -> Result<()> {
    let shaving_path = resolve_shaving_path(&shaving)?;

    if !shaving_path.exists() {
        bail!("Shaving not found: {}", shaving);
    }

    let config = ChamferConfig::default();
    let result = update_shaving(&shaving_path, &context, &config)?;

    if robot {
        output_json(&result)?;
    } else {
        output_human(&result);
    }

    Ok(())
}

/// Resolve shaving path (allow relative or partial match)
fn resolve_shaving_path(input: &str) -> Result<PathBuf> {
    let path = PathBuf::from(input);

    if path.exists() {
        return Ok(path);
    }

    // Try as relative to shavings directory
    let cwd = std::env::current_dir()?;
    let shavings_dir = find_shavings_dir(&cwd);

    // Try exact match in shavings
    let in_shavings = shavings_dir.join(input);
    if in_shavings.exists() {
        return Ok(in_shavings);
    }

    // Try partial match (substring of filename)
    if shavings_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&shavings_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.contains(input) || input.contains(&name.replace(".md", "")) {
                    return Ok(entry.path());
                }
            }
        }
    }

    // Return original path (will fail existence check)
    Ok(path)
}

/// Find shavings directory (current or parent)
fn find_shavings_dir(cwd: &PathBuf) -> PathBuf {
    if cwd.join("shavings").exists() {
        cwd.join("shavings")
    } else if let Some(parent) = cwd.parent() {
        if parent.join("shavings").exists() {
            parent.join("shavings")
        } else {
            cwd.join("shavings")
        }
    } else {
        cwd.join("shavings")
    }
}

fn output_human(result: &ChamferResult) {
    println!("🔄 Chamfer: Updated shaving");
    println!();
    println!("   Path: {}", result.path.display());
    println!("   Context added: {}", result.context_added);
    println!("   Updated: {}", result.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));

    if result.references > 0 {
        println!("   References found: {}", result.references);
    }
}

fn output_json(result: &ChamferResult) -> Result<()> {
    let json = serde_json::json!({
        "status": "success",
        "command": "chamfer",
        "result": {
            "path": result.path.to_string_lossy(),
            "updated": result.updated,
            "context_added": result.context_added,
            "updated_at": result.updated_at.to_rfc3339(),
            "references": result.references,
        }
    });

    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
