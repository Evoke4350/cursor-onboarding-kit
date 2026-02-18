//! Environment detection (O-02)
//!
//! Port from: claude-md.md:66-90
//!
//! Phase 1 of setup: Detect available tools, project type,
//! and any missing dependencies.

use std::path::Path;
use serde::{Deserialize, Serialize};

/// Detected environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Operating system
    pub os: String,
    /// Architecture
    pub arch: String,
    /// Home directory exists
    pub home_exists: bool,
    /// Gum available for prompts
    pub has_gum: bool,
    /// Ripgrep available for search
    pub has_ripgrep: bool,
    /// UVX available for Python tools
    pub has_uvx: bool,
    /// Git available
    pub has_git: bool,
    /// Cursor IDE installed
    pub cursor_installed: bool,
    /// Cursor database exists
    pub cursor_db_exists: bool,
    /// Claude Code detected
    pub claude_code: bool,
    /// Detected project type
    pub project_type: Option<String>,
    /// Missing required tools
    pub missing_tools: Vec<String>,
    /// Missing recommended tools
    pub missing_recommended: Vec<String>,
}

/// Detect available tools and environment
pub fn detect_environment() -> Environment {
    detect_environment_in(Path::new("."))
}

/// Detect environment in specific directory
pub fn detect_environment_in(dir: &Path) -> Environment {
    let mut missing_required = Vec::new();
    let mut missing_recommended = Vec::new();

    // Check required tools
    let has_git = which::which("git").is_ok();
    if !has_git {
        missing_required.push("git".to_string());
    }

    // Check recommended tools
    let has_gum = which::which("gum").is_ok();
    if !has_gum {
        missing_recommended.push("gum".to_string());
    }

    let has_ripgrep = which::which("rg").is_ok();
    if !has_ripgrep {
        missing_recommended.push("ripgrep".to_string());
    }

    let has_uvx = which::which("uvx").is_ok();
    if !has_uvx {
        missing_recommended.push("uvx".to_string());
    }

    let cursor_installed = detect_cursor();
    let cursor_db_exists = detect_cursor_db();

    // Detect project type
    let project_type = detect_project_type(dir);

    Environment {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        home_exists: dirs::home_dir().is_some(),
        has_gum,
        has_ripgrep,
        has_uvx,
        has_git,
        cursor_installed,
        cursor_db_exists,
        claude_code: detect_claude_code(),
        project_type,
        missing_tools: missing_required,
        missing_recommended,
    }
}

/// Detect if Cursor IDE is installed
fn detect_cursor() -> bool {
    dirs::data_dir()
        .map(|d| d.join("Cursor").exists())
        .unwrap_or(false)
}

/// Detect if Cursor SQLite database exists
fn detect_cursor_db() -> bool {
    dirs::data_dir()
        .map(|d| d.join("Cursor/User/globalStorage/state.vscdb").exists())
        .unwrap_or(false)
}

/// Detect if running in Claude Code
fn detect_claude_code() -> bool {
    // Check for Claude Code environment indicators
    std::env::var("CLAUDE_CODE").is_ok()
        || std::env::var("ANTHROPIC_API_KEY").is_ok()
        || std::env::current_exe()
            .map(|p| p.to_string_lossy().contains("claude"))
            .unwrap_or(false)
}

/// Detect project type from directory contents
fn detect_project_type(dir: &Path) -> Option<String> {
    // Check for various project indicators
    let indicators = [
        ("Cargo.toml", "rust"),
        ("package.json", "node"),
        ("pyproject.toml", "python"),
        ("requirements.txt", "python"),
        ("go.mod", "go"),
        ("pom.xml", "java"),
        ("build.gradle", "java"),
        ("Gemfile", "ruby"),
        ("composer.json", "php"),
        (".git", "git"),
        ("Makefile", "make"),
        ("flake.nix", "nix"),
    ];

    for (file, project_type) in indicators {
        if dir.join(file).exists() {
            return Some(project_type.to_string());
        }
    }

    // Check for markdown-heavy projects (docs)
    let md_count = std::fs::read_dir(dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
                .count()
        })
        .unwrap_or(0);

    if md_count >= 3 {
        return Some("docs".to_string());
    }

    None
}

impl Environment {
    /// Check if environment is ready for workshop
    pub fn is_ready(&self) -> bool {
        self.missing_tools.is_empty()
    }

    /// Get environment info as string
    pub fn summary(&self) -> String {
        let mut parts = vec![format!("OS: {}/{}", self.os, self.arch)];

        let tools = [
            ("git", self.has_git),
            ("gum", self.has_gum),
            ("rg", self.has_ripgrep),
            ("uvx", self.has_uvx),
            ("cursor", self.cursor_installed),
        ];

        for (name, has) in tools {
            if has {
                parts.push(format!("{} ✓", name));
            } else {
                parts.push(format!("{} ✗", name));
            }
        }

        parts.join(", ")
    }

    /// Get missing tools message
    pub fn missing_message(&self) -> Option<String> {
        if self.missing_tools.is_empty() && self.missing_recommended.is_empty() {
            return None;
        }

        let mut msg = String::new();

        if !self.missing_tools.is_empty() {
            msg.push_str("Required tools missing: ");
            msg.push_str(&self.missing_tools.join(", "));
            msg.push('\n');
        }

        if !self.missing_recommended.is_empty() {
            msg.push_str("Recommended tools missing: ");
            msg.push_str(&self.missing_recommended.join(", "));
        }

        Some(msg)
    }

    /// Get install hints for missing tools
    pub fn install_hints(&self) -> Vec<String> {
        let mut hints = Vec::new();

        if !self.has_git {
            hints.push("git: brew install git".to_string());
        }
        if !self.has_gum {
            hints.push("gum: brew install gum".to_string());
        }
        if !self.has_ripgrep {
            hints.push("ripgrep: brew install ripgrep".to_string());
        }
        if !self.has_uvx {
            hints.push("uvx: brew install uv".to_string());
        }

        hints
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_environment() {
        let env = detect_environment();
        assert!(!env.os.is_empty());
        // Git should be available in most dev environments
    }

    #[test]
    fn test_detect_project_type_rust() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("Cargo.toml"), "").unwrap();

        let env = detect_environment_in(temp_dir.path());
        assert_eq!(env.project_type, Some("rust".to_string()));
    }

    #[test]
    fn test_detect_project_type_node() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("package.json"), "{}").unwrap();

        let env = detect_environment_in(temp_dir.path());
        assert_eq!(env.project_type, Some("node".to_string()));
    }

    #[test]
    fn test_detect_project_type_python() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("pyproject.toml"), "").unwrap();

        let env = detect_environment_in(temp_dir.path());
        assert_eq!(env.project_type, Some("python".to_string()));
    }

    #[test]
    fn test_missing_message() {
        let mut env = Environment {
            os: "macos".to_string(),
            arch: "aarch64".to_string(),
            home_exists: true,
            has_gum: false,
            has_ripgrep: true,
            has_uvx: false,
            has_git: true,
            cursor_installed: true,
            cursor_db_exists: true,
            claude_code: true,
            project_type: None,
            missing_tools: vec![],
            missing_recommended: vec!["gum".to_string()],
        };

        let msg = env.missing_message();
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("gum"));
    }

    #[test]
    fn test_summary() {
        let env = detect_environment();
        let summary = env.summary();
        assert!(summary.contains("OS:"));
    }
}
