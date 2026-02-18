//! Taint command - Security operations (S-01, S-02, SEC-01, SEC-03, SEC-04)

use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;

use crate::modules::security::taint::{TaintTracker, TaintLevel};

/// Default taint sources (paths that trigger taint)
const DEFAULT_SOURCES: &[&str] = &[
    ".secrets/",
    "*.env",
    "*.pem",
    "*.key",
    "credentials.*",
    "id_rsa*",
    "*.crt",
];

/// Default exfil sinks (commands blocked when tainted)
const DEFAULT_SINKS: &[&str] = &[
    "curl",
    "wget",
    "rsync",
    "scp",
    "nc",
    "telnet",
    "ftp",
    "ssh",
];

pub fn run(
    status: bool,
    mark: Option<String>,
    check: Option<String>,
    hook_read: Option<String>,
    hook_shell: Option<String>,
    robot: bool,
) -> Result<()> {
    // Load or create tracker
    let mut tracker = load_tracker()?;

    // Hook mode: output JSON and exit
    if let Some(path) = hook_read {
        return handle_hook_read(&tracker, &path);
    }

    if let Some(cmd) = hook_shell {
        return handle_hook_shell(&tracker, &cmd);
    }

    // Mark a path as tainted
    if let Some(path) = mark {
        let level = detect_taint_level(&path);
        tracker.add_taint(PathBuf::from(&path), level);
        save_tracker(&tracker)?;

        if robot {
            println!("{}", json!({
                "status": "marked",
                "path": path,
                "level": format!("{:?}", level)
            }));
        } else {
            println!("🛡️  Marked as tainted: {} (level: {:?})", path, level);
        }
        return Ok(());
    }

    // Check if command would be blocked
    if let Some(cmd) = check {
        let blocked = tracker.should_block(&cmd, DEFAULT_SINKS);
        let level = tracker.max_taint_level();

        if robot {
            println!("{}", json!({
                "blocked": blocked,
                "command": cmd,
                "taint_level": format!("{:?}", level),
                "tainted": tracker.is_tainted()
            }));
        } else {
            if blocked {
                println!("🚫 BLOCKED: '{}' (conversation tainted at {:?})", cmd, level);
            } else {
                println!("✅ Allowed: '{}' (taint level: {:?})", cmd, level);
            }
        }
        return Ok(());
    }

    // Status display
    if status || robot {
        let level = tracker.max_taint_level();
        let paths: Vec<_> = tracker.tainted_paths().iter().map(|p| p.to_string_lossy().to_string()).collect();

        if robot {
            println!("{}", json!({
                "status": "ok",
                "tainted": tracker.is_tainted(),
                "max_level": format!("{:?}", level),
                "tainted_paths": paths,
                "sources": DEFAULT_SOURCES,
                "sinks": DEFAULT_SINKS
            }));
        } else {
            println!("🛡️  Taint Status");
            println!("   Tainted: {}", tracker.is_tainted());
            println!("   Max level: {:?}", level);
            if !paths.is_empty() {
                println!("   Tainted paths:");
                for p in &paths {
                    println!("     - {}", p);
                }
            }
            println!();
            println!("   Sources (trigger taint):");
            for s in DEFAULT_SOURCES {
                println!("     - {}", s);
            }
            println!();
            println!("   Sinks (blocked if tainted):");
            for s in DEFAULT_SINKS {
                println!("     - {}", s);
            }
        }
        return Ok(());
    }

    // Default: show help
    println!("🛡️  Taint tracking commands:");
    println!("   workshop taint --status          Show current state");
    println!("   workshop taint --mark <path>     Mark path as tainted");
    println!("   workshop taint --check <cmd>     Check if command blocked");
    println!("   workshop taint --hook-read <p>   Hook mode: file read check (JSON)");
    println!("   workshop taint --hook-shell <c>  Hook mode: shell exec check (JSON)");

    Ok(())
}

/// Handle beforeFileRead hook - outputs JSON for Cursor
fn handle_hook_read(tracker: &TaintTracker, path: &str) -> Result<()> {
    let is_source = DEFAULT_SOURCES.iter().any(|pattern| {
        if pattern.ends_with('/') {
            path.starts_with(pattern.trim_end_matches('/'))
        } else if pattern.starts_with('*') {
            path.ends_with(pattern.trim_start_matches('*'))
        } else if pattern.ends_with('*') {
            path.starts_with(pattern.trim_end_matches('*'))
        } else {
            path.contains(pattern.trim_matches('*'))
        }
    });

    let level = if is_source {
        TaintLevel::High
    } else {
        tracker.max_taint_level()
    };

    // Always allow reads, but mark as tainted if source
    println!("{}", json!({
        "allow": true,
        "tainted": is_source || tracker.is_tainted(),
        "level": format!("{:?}", level),
        "path": path,
        "is_source": is_source
    }));

    Ok(())
}

/// Handle beforeShellExecution hook - outputs JSON for Cursor
fn handle_hook_shell(tracker: &TaintTracker, cmd: &str) -> Result<()> {
    let blocked = tracker.should_block(cmd, DEFAULT_SINKS);
    let level = tracker.max_taint_level();

    // Extract the base command
    let base_cmd = cmd.split_whitespace().next().unwrap_or(cmd);
    let is_sink = DEFAULT_SINKS.contains(&base_cmd);

    println!("{}", json!({
        "allow": !blocked,
        "blocked": blocked,
        "reason": if blocked { "Conversation tainted, network sink blocked" } else { "ok" },
        "command": cmd,
        "base_command": base_cmd,
        "is_sink": is_sink,
        "taint_level": format!("{:?}", level),
        "tainted": tracker.is_tainted()
    }));

    Ok(())
}

/// Detect taint level based on path pattern
fn detect_taint_level(path: &str) -> TaintLevel {
    if path.contains(".secrets/") || path.contains("id_rsa") {
        TaintLevel::Critical
    } else if path.ends_with(".pem") || path.ends_with(".key") {
        TaintLevel::Critical
    } else if path.contains("credentials") {
        TaintLevel::High
    } else if path.ends_with(".env") {
        TaintLevel::High
    } else {
        TaintLevel::Low
    }
}

/// Load tracker from state file
fn load_tracker() -> Result<TaintTracker> {
    let state_path = dirs::home_dir()
        .map(|h| h.join(".workshop/taint-state.json"));

    if let Some(path) = state_path {
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            if !content.is_empty() {
                return Ok(serde_json::from_str(&content)?);
            }
        }
    }

    Ok(TaintTracker::new())
}

/// Save tracker to state file
fn save_tracker(tracker: &TaintTracker) -> Result<()> {
    if let Some(home) = dirs::home_dir() {
        let dir = home.join(".workshop");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("taint-state.json");
        let content = serde_json::to_string_pretty(tracker)?;
        std::fs::write(&path, content)?;
    }
    Ok(())
}
