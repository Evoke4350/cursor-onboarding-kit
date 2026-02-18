//! Cut command - Extract atomic insight (P-01)

use anyhow::{Result, bail};
use std::path::PathBuf;
use crate::modules::pipeline::{Shaving, CutConfig, extract_shaving, save_shaving};

pub fn run(source: String, with_code_ref: bool, output: Option<String>, robot: bool) -> Result<()> {
    let source_path = PathBuf::from(&source);

    if !source_path.exists() {
        bail!("Source file not found: {}", source);
    }

    let content = std::fs::read_to_string(&source_path)?;

    let config = CutConfig {
        with_code_ref,
        max_text_length: 2000,
        auto_detect_language: true,
    };

    let shaving = extract_shaving(&source_path, &content, None, &config)?;

    // Determine output directory
    let output_dir = match output {
        Some(path) => PathBuf::from(path),
        None => {
            // Default to shavings/ in current directory or parent if exists
            let cwd = std::env::current_dir()?;
            if cwd.join("shavings").exists() {
                cwd.join("shavings")
            } else if cwd.parent().map(|p| p.join("shavings").exists()).unwrap_or(false) {
                cwd.parent().unwrap().join("shavings")
            } else {
                cwd.join("shavings")
            }
        }
    };

    let saved_path = save_shaving(&shaving, &output_dir)?;

    if robot {
        output_json(&shaving, &saved_path)?;
    } else {
        output_human(&shaving, &saved_path);
    }

    Ok(())
}

fn output_human(shaving: &Shaving, saved_path: &PathBuf) {
    println!("✂️  Cut: Extracted insight");
    println!();
    println!("   Summary: {}", shaving.summary);
    println!("   Source:  {}", shaving.source.display());
    println!("   Saved:   {}", saved_path.display());

    if !shaving.tags.is_empty() {
        println!("   Tags:    {}", shaving.tags.join(" "));
    }

    if let Some(ref code) = shaving.code_ref {
        println!();
        println!("   Code ref: {} ({})", code.symbol.as_deref().unwrap_or("unknown"), code.language);
    }

    println!();
    println!("   ID: {}", shaving.id);
}

fn output_json(shaving: &Shaving, saved_path: &PathBuf) -> Result<()> {
    let json = serde_json::json!({
        "status": "success",
        "command": "cut",
        "shaving": {
            "id": shaving.id,
            "source": shaving.source.to_string_lossy(),
            "summary": shaving.summary,
            "text_length": shaving.text.len(),
            "tags": shaving.tags,
            "created_at": shaving.created_at.to_rfc3339(),
            "saved_to": saved_path.to_string_lossy(),
        }
    });

    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
