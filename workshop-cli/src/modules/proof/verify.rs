//! Document verification (W-03)
//!
//! Source: showboat/verify.go

use anyhow::Result;
use std::path::Path;

/// Verify a showboat document
pub fn verify_document(path: &Path) -> Result<VerifyResult> {
    let output = std::process::Command::new("uvx")
        .args(["showboat", "verify", path.to_str().unwrap()])
        .output()?;

    let success = output.status.success();
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let error = if success {
        None
    } else {
        Some(String::from_utf8_lossy(&output.stderr).to_string())
    };

    Ok(VerifyResult {
        path: path.to_path_buf(),
        success,
        output: output_str,
        error,
    })
}

/// Verification result
#[derive(Debug)]
pub struct VerifyResult {
    pub path: std::path::PathBuf,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// Verify all shavings in a directory
pub fn verify_all(shavings_dir: &Path) -> Result<Vec<VerifyResult>> {
    let mut results = Vec::new();

    if !shavings_dir.exists() {
        return Ok(results);
    }

    for entry in std::fs::read_dir(shavings_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "md").unwrap_or(false) {
            results.push(verify_document(&path)?);
        }
    }

    Ok(results)
}
