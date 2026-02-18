//! Showboat CLI integration (W-01)
//!
//! Source: showboat/main.go

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Showboat document wrapper
pub struct ShowboatDoc {
    pub path: PathBuf,
    pub uuid: String,
}

impl ShowboatDoc {
    /// Initialize a new showboat document
    pub fn init(path: &Path, title: &str) -> Result<Self> {
        let output = Command::new("uvx")
            .args(["showboat", "init", path.to_str().unwrap(), title])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "showboat init failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Extract UUID from output
        let stdout = String::from_utf8_lossy(&output.stdout);
        let uuid = stdout
            .lines()
            .find(|l| l.contains("uuid:"))
            .map(|l| l.split(':').last().unwrap_or("unknown").trim().to_string())
            .unwrap_or_default();

        Ok(Self {
            path: path.to_path_buf(),
            uuid,
        })
    }

    /// Add a note to the document
    pub fn note(&self, text: &str) -> Result<()> {
        let output = Command::new("uvx")
            .args([
                "showboat",
                "note",
                self.path.to_str().unwrap(),
                text,
            ])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "showboat note failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Execute a command and capture output
    pub fn exec(&self, lang: &str, code: &str) -> Result<String> {
        let output = Command::new("uvx")
            .args([
                "showboat",
                "exec",
                self.path.to_str().unwrap(),
                lang,
                code,
            ])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
