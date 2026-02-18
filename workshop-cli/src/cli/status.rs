//! Status command - PM dashboard for project insight

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn run(robot: bool) -> Result<()> {
    let mut sections = Vec::new();

    // Git status
    sections.push(git_section()?);

    // Test status
    sections.push(test_section()?);

    // Workshop structure
    sections.push(workshop_section()?);

    // Recent activity
    sections.push(activity_section()?);

    if robot {
        let json = serde_json::json!({
            "status": "success",
            "command": "status",
            "sections": sections
        });
        println!("{}", serde_json::to_string(&json)?);
    } else {
        println!("📊 Workshop Status Dashboard\n");

        for section in sections {
            if let Some(title) = section.get("title").and_then(|t| t.as_str()) {
                println!("## {}", title);
            }
            if let Some(content) = section.get("content").and_then(|c| c.as_str()) {
                println!("{}", content);
            }
            println!();
        }
    }

    Ok(())
}

fn git_section() -> Result<serde_json::Value> {
    let branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let status = Command::new("git")
        .args(["status", "--short"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "not a git repo".to_string());

    // Filter out build artifacts
    let changed: Vec<&str> = status.lines()
        .filter(|l| !l.is_empty())
        .filter(|l| !l.contains("/target/"))
        .filter(|l| !l.contains("node_modules"))
        .filter(|l| !l.contains(".fingerprint"))
        .collect();

    let display = if changed.is_empty() {
        "clean".to_string()
    } else if changed.len() <= 5 {
        changed.join("\n  ")
    } else {
        format!("{} files ({} shown)", changed.len(), 5)
    };

    Ok(serde_json::json!({
        "title": "Git",
        "content": format!("Branch: {}\nUncommitted: {}", branch, display),
        "branch": branch,
        "uncommitted": changed.len()
    }))
}

fn test_section() -> Result<serde_json::Value> {
    // Check if Cargo.toml exists
    let cargo_exists = Path::new("Cargo.toml").exists() || Path::new("workshop-cli/Cargo.toml").exists();

    if !cargo_exists {
        return Ok(serde_json::json!({
            "title": "Tests",
            "content": "No Rust project found",
            "count": 0
        }));
    }

    let output = Command::new("cargo")
        .args(["test", "--", "--list"])
        .current_dir("workshop-cli")
        .output()
        .ok();

    let (count, status) = match output {
        Some(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let test_count = stdout.lines().filter(|l| l.contains("::")).count();
            (test_count, if o.status.success() { "✅" } else { "❌" })
        }
        None => (0, "⚠️"),
    };

    Ok(serde_json::json!({
        "title": "Tests",
        "content": format!("{} {} tests", status, count),
        "count": count
    }))
}

fn workshop_section() -> Result<serde_json::Value> {
    let mut parts = Vec::new();

    // Count shavings in various possible locations
    let shavings_count = count_md_files("shavings")
        .or_else(|_| count_md_files("workshop-cli/shavings"))
        .or_else(|_| count_md_files("workshop/shavings"))
        .unwrap_or(0);
    if shavings_count > 0 {
        parts.push(format!("📝 {} shavings", shavings_count));
    }

    // Count specs
    let specs_count = count_md_files("specs")
        .or_else(|_| count_md_files("workshop-cli/specs"))
        .unwrap_or(0);
    if specs_count > 0 {
        parts.push(format!("📋 {} specs", specs_count));
    }

    // Check for .beads
    if Path::new(".beads").exists() {
        let beads_count = count_md_files(".beads/issues").unwrap_or(0);
        parts.push(format!("🔵 {} beads", beads_count));
    }

    // Check for LAB directories
    let lab_dirs = std::fs::read_dir(".")
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().starts_with("LAB-"))
                .count()
        })
        .unwrap_or(0);
    if lab_dirs > 0 {
        parts.push(format!("🔬 {} LAB-* dirs", lab_dirs));
    }

    // Check for workshop folder
    if Path::new("workshop").exists() {
        parts.push("🔧 workshop/".to_string());
    }

    let content = if parts.is_empty() {
        "No workshop structure found".to_string()
    } else {
        parts.join("\n")
    };

    Ok(serde_json::json!({
        "title": "Workshop",
        "content": content,
        "shavings": shavings_count,
        "specs": specs_count
    }))
}

fn activity_section() -> Result<serde_json::Value> {
    let log = Command::new("git")
        .args(["log", "--oneline", "-5"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "No git history".to_string());

    // Format commits nicely
    let commits: Vec<String> = log.lines()
        .take(5)
        .map(|l| format!("  {}", l))
        .collect();

    Ok(serde_json::json!({
        "title": "Recent Commits",
        "content": commits.join("\n"),
        "commits": commits
    }))
}

fn count_md_files(dir: &str) -> Result<usize> {
    use std::fs;

    let path = Path::new(dir);
    if !path.exists() {
        return Ok(0);
    }

    let count = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
        .count();

    Ok(count)
}
