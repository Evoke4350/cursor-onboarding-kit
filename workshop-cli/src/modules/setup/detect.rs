//! Environment detection (O-02)
//!
//! Port from: claude-md.md:66-90

use std::collections::HashMap;

/// Detected environment
#[derive(Debug, Clone)]
pub struct Environment {
    pub os: String,
    pub has_gum: bool,
    pub has_ripgrep: bool,
    pub has_uvx: bool,
    pub cursor_installed: bool,
    pub claude_code: bool,
}

/// Detect available tools and environment
pub fn detect_environment() -> Environment {
    Environment {
        os: std::env::consts::OS.to_string(),
        has_gum: which::which("gum").is_ok(),
        has_ripgrep: which::which("rg").is_ok(),
        has_uvx: which::which("uvx").is_ok(),
        cursor_installed: detect_cursor(),
        claude_code: true, // We're running in Claude Code
    }
}

fn detect_cursor() -> bool {
    dirs::data_dir()
        .map(|d| d.join("Cursor").exists())
        .unwrap_or(false)
}

impl Environment {
    /// Get environment info as string
    pub fn summary(&self) -> String {
        let mut parts = vec![format!("OS: {}", self.os)];

        if self.has_gum {
            parts.push("gum ✓".to_string());
        }
        if self.has_ripgrep {
            parts.push("ripgrep ✓".to_string());
        }
        if self.has_uvx {
            parts.push("uvx ✓".to_string());
        }
        if self.cursor_installed {
            parts.push("cursor ✓".to_string());
        }

        parts.join(", ")
    }
}
