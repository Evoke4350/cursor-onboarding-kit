//! Carve command - Find connections (P-02)

use anyhow::Result;
use std::path::PathBuf;
use crate::modules::pipeline::{SearchResult, SearchConfig, search, search_shavings};

pub fn run(query: String, domain: Option<String>, robot: bool) -> Result<()> {
    // Determine search directory based on domain
    let (search_dir, config) = match domain.as_deref() {
        Some("shavings") | None => {
            let cwd = std::env::current_dir()?;
            let shavings = find_shavings_dir(&cwd);
            (shavings, SearchConfig::default())
        }
        Some("all") => {
            let cwd = std::env::current_dir()?;
            (
                cwd,
                SearchConfig {
                    extensions: vec!["md".to_string(), "txt".to_string()],
                    ..Default::default()
                },
            )
        }
        Some(path) => {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                (path_buf, SearchConfig::default())
            } else {
                anyhow::bail!("Domain path not found: {}", path);
            }
        }
    };

    if !search_dir.exists() {
        if robot {
            println!(r#"{{"status": "no_results", "command": "carve", "results": []}}"#);
        } else {
            println!("🔪 Carve: No shavings directory found");
            println!("   Run `workshop cut <file>` to create shavings first");
        }
        return Ok(());
    }

    let results = search(&query, &search_dir, &config)?;

    if robot {
        output_json(&query, &results)?;
    } else {
        output_human(&query, &results);
    }

    Ok(())
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

fn output_human(query: &str, results: &[SearchResult]) {
    println!("🔪 Carve: {} results for '{}'", results.len(), query);
    println!();

    if results.is_empty() {
        println!("   No matches found");
        return;
    }

    for result in results {
        println!("━━━ {} (score: {:.2})", result.path.display(), result.score);

        // Print context with match highlighted
        for line in result.context.lines() {
            if line.starts_with(">>>") {
                println!("\x1b[33m{}\x1b[0m", line); // Yellow for match line
            } else {
                println!("{}", line);
            }
        }
        println!();
    }
}

fn output_json(query: &str, results: &[SearchResult]) -> Result<()> {
    let json = serde_json::json!({
        "status": "success",
        "command": "carve",
        "query": query,
        "count": results.len(),
        "results": results.iter().map(|r| serde_json::json!({
            "path": r.path.to_string_lossy(),
            "line": r.line,
            "score": r.score,
            "match_type": format!("{:?}", r.match_type),
            "context": r.context,
        })).collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
