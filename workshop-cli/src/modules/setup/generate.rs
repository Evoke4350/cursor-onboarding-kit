//! Workshop generation (O-04)
//!
//! Port from: features/*.md

use super::DerivedConfig;
use anyhow::Result;
use std::path::Path;

/// Generate workshop structure from configuration
pub fn generate_workshop(config: &DerivedConfig, path: &Path) -> Result<()> {
    // Create folder structure
    std::fs::create_dir_all(path.join("bench"))?;
    std::fs::create_dir_all(path.join("shavings"))?;
    std::fs::create_dir_all(path.join("sawdust/sessions"))?;
    std::fs::create_dir_all(path.join("sawdust/state"))?;
    std::fs::create_dir_all(path.join("sawdust/audit"))?;
    std::fs::create_dir_all(path.join(".workshop/tools/core"))?;
    std::fs::create_dir_all(path.join(".workshop/policy"))?;
    std::fs::create_dir_all(path.join(".workshop/templates"))?;

    // Generate identity.md
    let identity = generate_identity(config);
    std::fs::write(path.join("bench/identity.md"), identity)?;

    // Generate policy if security enabled
    if config.security_level != super::derive::SecurityLevel::None {
        generate_policy_files(path)?;
    }

    // Generate state files
    std::fs::write(path.join("sawdust/state/current-task.md"), "")?;
    std::fs::write(path.join("sawdust/state/taint-state.txt"), "TAINT_STATE: clean\n")?;

    Ok(())
}

fn generate_identity(config: &DerivedConfig) -> String {
    let work_type = match config.work_type {
        super::derive::WorkType::Software => "software engineering",
        super::derive::WorkType::Research => "research",
        super::derive::WorkType::Writing => "writing",
    };

    format!(
        r#"---
created: {}
work_type: {:?}
code_refs: {:?}
---

# Workshop Identity

This workshop helps with {}.

## Methodology

Extract insights, find connections, update older work.

## Commands

- `/cut` - Extract atomic insight
- `/carve` - Find connections
- `/chamfer` - Update older work
- `/check` - Validate everything
"#,
        chrono::Local::now().format("%Y-%m-%d"),
        config.work_type,
        config.code_refs,
        work_type
    )
}

fn generate_policy_files(path: &Path) -> Result<()> {
    let sources = r#"# Protected paths
sources:
  - pattern: ".secrets/*"
    taint: critical
    description: "Secrets directory"

  - pattern: "*.env"
    taint: high
    description: "Environment files"

  - pattern: "*.pem"
    taint: critical
    description: "Certificate files"

  - pattern: "*.key"
    taint: critical
    description: "Private key files"

  - pattern: "credentials.*"
    taint: critical
    description: "Credential files"
"#;

    let sinks = r#"# Exfiltration sinks
sinks:
  - command: "curl"
    block_if_tainted: true
    reason: "HTTP client can exfiltrate data"

  - command: "wget"
    block_if_tainted: true
    reason: "HTTP client can exfiltrate data"

  - command: "rsync"
    block_if_tainted: true
    reason: "File sync can exfiltrate data"

  - command: "scp"
    block_if_tainted: true
    reason: "Secure copy can exfiltrate data"

  - command: "nc"
    block_if_tainted: true
    reason: "Netcat can exfiltrate data"
"#;

    std::fs::write(path.join(".workshop/policy/sources.yaml"), sources)?;
    std::fs::write(path.join(".workshop/policy/sinks.yaml"), sinks)?;

    Ok(())
}
