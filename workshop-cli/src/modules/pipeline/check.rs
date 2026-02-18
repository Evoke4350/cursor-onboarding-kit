//! Check command - Validate everything (P-04)
//!
//! The "Check" phase validates shavings, links, and overall workshop health.
//! Supports adversarial testing for security validation.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Check result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Total files checked
    pub files_checked: usize,
    /// Files with issues
    pub issues_found: usize,
    /// Individual check results
    pub checks: Vec<IndividualCheck>,
    /// Overall health score (0-100)
    pub health_score: u8,
}

/// Individual check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualCheck {
    /// Check name
    pub name: String,
    /// Check status
    pub status: CheckStatus,
    /// Message or details
    pub message: String,
    /// Affected files
    pub affected_files: Vec<PathBuf>,
}

/// Check status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
    Skipped,
}

/// Configuration for check operation
#[derive(Debug, Clone)]
pub struct CheckConfig {
    /// Run adversarial tests
    pub adversarial: bool,
    /// Verify showboat documents
    pub verify_showboat: bool,
    /// Check for broken links
    pub check_links: bool,
    /// Check for stale content
    pub check_stale: bool,
    /// Days threshold for stale content
    pub stale_days: u64,
}

impl Default for CheckConfig {
    fn default() -> Self {
        Self {
            adversarial: false,
            verify_showboat: false,
            check_links: true,
            check_stale: true,
            stale_days: 30,
        }
    }
}

/// Run all checks on workshop
pub fn run_checks(dir: &Path, config: &CheckConfig) -> Result<CheckResult> {
    let mut checks = Vec::new();
    let mut issues = 0;
    let mut files_checked = 0;

    // Check 1: Shavings structure
    let shavings_check = check_shavings_structure(dir)?;
    if shavings_check.status == CheckStatus::Fail {
        issues += 1;
    }
    files_checked += shavings_check.affected_files.len();
    checks.push(shavings_check);

    // Check 2: Markdown format
    let md_check = check_markdown_format(dir)?;
    if md_check.status == CheckStatus::Fail {
        issues += 1;
    }
    files_checked += md_check.affected_files.len();
    checks.push(md_check);

    // Check 3: Broken internal links
    if config.check_links {
        let link_check = check_broken_links(dir)?;
        if link_check.status == CheckStatus::Fail {
            issues += 1;
        }
        files_checked += link_check.affected_files.len();
        checks.push(link_check);
    }

    // Check 4: Stale content
    if config.check_stale {
        let stale_check = check_stale_content(dir, config.stale_days)?;
        if stale_check.status == CheckStatus::Warning {
            // Stale is a warning, not failure
        }
        files_checked += stale_check.affected_files.len();
        checks.push(stale_check);
    }

    // Check 5: Adversarial tests (if enabled)
    if config.adversarial {
        let adv_check = run_adversarial_tests(dir)?;
        if adv_check.status == CheckStatus::Fail {
            issues += 1;
        }
        checks.push(adv_check);
    }

    // Calculate health score
    let total_checks = checks.len();
    let passing = checks.iter().filter(|c| c.status == CheckStatus::Pass).count();
    let warnings = checks.iter().filter(|c| c.status == CheckStatus::Warning).count();

    let health_score = if total_checks > 0 {
        let base = (passing as f64 / total_checks as f64) * 100.0;
        let warning_penalty = (warnings as f64 / total_checks as f64) * 20.0;
        (base - warning_penalty).max(0.0) as u8
    } else {
        100
    };

    Ok(CheckResult {
        files_checked,
        issues_found: issues,
        checks,
        health_score,
    })
}

/// Check shavings directory structure
fn check_shavings_structure(dir: &Path) -> Result<IndividualCheck> {
    let shavings_dir = dir.join("shavings");
    let mut affected_files = Vec::new();

    if !shavings_dir.exists() {
        return Ok(IndividualCheck {
            name: "Shavings Directory".to_string(),
            status: CheckStatus::Fail,
            message: "No shavings directory found".to_string(),
            affected_files: vec![],
        });
    }

    // Check for markdown files
    let md_count = WalkDir::new(&shavings_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .count();

    if md_count == 0 {
        return Ok(IndividualCheck {
            name: "Shavings Directory".to_string(),
            status: CheckStatus::Warning,
            message: "Shavings directory exists but contains no markdown files".to_string(),
            affected_files: vec![shavings_dir],
        });
    }

    affected_files.push(shavings_dir);

    Ok(IndividualCheck {
        name: "Shavings Directory".to_string(),
        status: CheckStatus::Pass,
        message: format!("Found {} shavings", md_count),
        affected_files,
    })
}

/// Check markdown formatting
fn check_markdown_format(dir: &Path) -> Result<IndividualCheck> {
    let mut affected_files = Vec::new();
    let mut issues = 0;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let path = entry.path();
        if let Ok(content) = std::fs::read_to_string(path) {
            // Check for common markdown issues
            if !content.starts_with('#') {
                issues += 1;
                affected_files.push(path.to_path_buf());
            }
        }
    }

    if issues > 0 {
        Ok(IndividualCheck {
            name: "Markdown Format".to_string(),
            status: CheckStatus::Warning,
            message: format!("{} files missing title header", issues),
            affected_files,
        })
    } else {
        Ok(IndividualCheck {
            name: "Markdown Format".to_string(),
            status: CheckStatus::Pass,
            message: "All markdown files properly formatted".to_string(),
            affected_files,
        })
    }
}

/// Check for broken internal links
fn check_broken_links(dir: &Path) -> Result<IndividualCheck> {
    let mut affected_files = Vec::new();
    let mut broken_count = 0;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let path = entry.path();
        if let Ok(content) = std::fs::read_to_string(path) {
            // Find markdown links: [text](path)
            for line in content.lines() {
                if line.contains("](") {
                    // Extract link target (simplified)
                    if let Some(start) = line.find("](") {
                        if let Some(end) = line[start + 2..].find(')') {
                            let link = &line[start + 2..start + 2 + end];
                            // Check if it's a relative link
                            if !link.starts_with("http") && !link.starts_with("#") {
                                let target = path.parent()
                                    .unwrap_or(dir)
                                    .join(link);
                                if !target.exists() {
                                    broken_count += 1;
                                    affected_files.push(path.to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if broken_count > 0 {
        Ok(IndividualCheck {
            name: "Internal Links".to_string(),
            status: CheckStatus::Fail,
            message: format!("{} broken links found", broken_count),
            affected_files,
        })
    } else {
        Ok(IndividualCheck {
            name: "Internal Links".to_string(),
            status: CheckStatus::Pass,
            message: "All internal links valid".to_string(),
            affected_files,
        })
    }
}

/// Check for stale content
fn check_stale_content(dir: &Path, stale_days: u64) -> Result<IndividualCheck> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut affected_files = Vec::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let stale_threshold = now - (stale_days * 24 * 60 * 60);

    let mut stale_count = 0;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let path = entry.path();
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let mtime = modified
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                if mtime < stale_threshold {
                    stale_count += 1;
                    affected_files.push(path.to_path_buf());
                }
            }
        }
    }

    if stale_count > 0 {
        Ok(IndividualCheck {
            name: "Stale Content".to_string(),
            status: CheckStatus::Warning,
            message: format!("{} files older than {} days", stale_count, stale_days),
            affected_files,
        })
    } else {
        Ok(IndividualCheck {
            name: "Stale Content".to_string(),
            status: CheckStatus::Pass,
            message: "All content is fresh".to_string(),
            affected_files,
        })
    }
}

/// Run adversarial security tests
fn run_adversarial_tests(dir: &Path) -> Result<IndividualCheck> {
    // Check for sensitive files that should not be in shavings
    let sensitive_patterns = [
        ".env",
        "credentials",
        "secret",
        "password",
        "api_key",
        "private_key",
    ];

    let mut affected_files = Vec::new();
    let mut violations = 0;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let path_str = path.to_string_lossy().to_lowercase();

        for pattern in &sensitive_patterns {
            if path_str.contains(pattern) {
                violations += 1;
                affected_files.push(path.to_path_buf());
                break;
            }
        }
    }

    if violations > 0 {
        Ok(IndividualCheck {
            name: "Adversarial Security".to_string(),
            status: CheckStatus::Fail,
            message: format!("{} potential security violations found", violations),
            affected_files,
        })
    } else {
        Ok(IndividualCheck {
            name: "Adversarial Security".to_string(),
            status: CheckStatus::Pass,
            message: "No security violations detected".to_string(),
            affected_files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_check_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config = CheckConfig::default();
        let result = run_checks(temp_dir.path(), &config).unwrap();

        assert!(result.health_score < 100);
    }

    #[test]
    fn test_check_with_shavings() {
        let temp_dir = TempDir::new().unwrap();
        let shavings = temp_dir.path().join("shavings");
        std::fs::create_dir(&shavings).unwrap();
        std::fs::write(
            shavings.join("test.md"),
            "# Test Shaving\n\nContent here."
        ).unwrap();

        let config = CheckConfig::default();
        let result = run_checks(temp_dir.path(), &config).unwrap();

        assert!(result.health_score > 50);
    }

    #[test]
    fn test_check_markdown_format() {
        let temp_dir = TempDir::new().unwrap();
        let shavings = temp_dir.path().join("shavings");
        std::fs::create_dir(&shavings).unwrap();

        // File without header
        std::fs::write(shavings.join("bad.md"), "No header here").unwrap();

        let check = check_markdown_format(temp_dir.path()).unwrap();
        assert_eq!(check.status, CheckStatus::Warning);
    }

    #[test]
    fn test_check_broken_links() {
        let temp_dir = TempDir::new().unwrap();
        let shavings = temp_dir.path().join("shavings");
        std::fs::create_dir(&shavings).unwrap();

        // File with broken link
        std::fs::write(
            shavings.join("link.md"),
            "# Test\n\nSee [missing](./nonexistent.md)"
        ).unwrap();

        let check = check_broken_links(temp_dir.path()).unwrap();
        assert_eq!(check.status, CheckStatus::Fail);
    }

    #[test]
    fn test_adversarial_check() {
        let temp_dir = TempDir::new().unwrap();
        let shavings = temp_dir.path().join("shavings");
        std::fs::create_dir(&shavings).unwrap();

        // Sensitive file
        std::fs::write(shavings.join("credentials.md"), "# Oops").unwrap();

        let check = run_adversarial_tests(temp_dir.path()).unwrap();
        assert_eq!(check.status, CheckStatus::Fail);
    }
}
