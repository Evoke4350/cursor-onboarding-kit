//! Check command - Validate everything (P-04)

use anyhow::Result;
use std::path::PathBuf;
use crate::modules::pipeline::{CheckResult, CheckConfig, CheckStatus, run_checks};

pub fn run(adversarial: bool, verify: bool, robot: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let workshop_dir = find_workshop_root(&cwd);

    let config = CheckConfig {
        adversarial,
        verify_showboat: verify,
        check_links: true,
        check_stale: true,
        stale_days: 30,
    };

    let result = run_checks(&workshop_dir, &config)?;

    if robot {
        output_json(&result)?;
    } else {
        output_human(&result);
    }

    // Exit with error code if issues found
    if result.issues_found > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Find workshop root (directory with shavings/ or .workshop/)
fn find_workshop_root(cwd: &PathBuf) -> PathBuf {
    let mut current = cwd.clone();

    loop {
        if current.join("shavings").exists() || current.join(".workshop").exists() {
            return current;
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return cwd.clone(),
        }
    }
}

fn output_human(result: &CheckResult) {
    println!("✅ Check: Workshop validation");
    println!();
    println!("   Files checked: {}", result.files_checked);
    println!("   Issues found:  {}", result.issues_found);
    println!("   Health score:  {}/100", result.health_score);
    println!();

    for check in &result.checks {
        let icon = match check.status {
            CheckStatus::Pass => "✓",
            CheckStatus::Warning => "⚠",
            CheckStatus::Fail => "✗",
            CheckStatus::Skipped => "○",
        };

        println!("   {} {}: {}", icon, check.name, check.message);

        if check.status != CheckStatus::Pass && !check.affected_files.is_empty() {
            for file in &check.affected_files {
                println!("      - {}", file.display());
            }
        }
    }

    println!();

    if result.issues_found > 0 {
        println!("❌ Validation failed with {} issue(s)", result.issues_found);
    } else {
        println!("✅ All checks passed!");
    }
}

fn output_json(result: &CheckResult) -> Result<()> {
    let json = serde_json::json!({
        "status": if result.issues_found > 0 { "fail" } else { "pass" },
        "command": "check",
        "files_checked": result.files_checked,
        "issues_found": result.issues_found,
        "health_score": result.health_score,
        "checks": result.checks.iter().map(|c| serde_json::json!({
            "name": c.name,
            "status": format!("{:?}", c.status),
            "message": c.message,
            "affected_files": c.affected_files.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>(),
        })).collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
