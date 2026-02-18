//! Workshop generation (O-04)
//!
//! Port from: features/*.md
//!
//! Phase 5 of setup: Generate workshop files and structure.
//! Creates all directories, config files, and initial templates.

use super::DerivedConfig;
use anyhow::Result;
use std::path::Path;

/// Generate workshop structure from configuration
pub fn generate_workshop(config: &DerivedConfig, path: &Path) -> Result<GenerateResult> {
    let mut created = Vec::new();
    let mut skipped = Vec::new();

    // Create folder structure
    let dirs = [
        "bench",
        "shavings",
        "sawdust/sessions",
        "sawdust/state",
        "sawdust/audit",
        ".workshop/tools/core",
        ".workshop/policy",
        ".workshop/templates",
    ];

    for dir in dirs {
        let full_path = path.join(dir);
        if !full_path.exists() {
            std::fs::create_dir_all(&full_path)?;
            created.push(format!("dir:{}", dir));
        } else {
            skipped.push(format!("dir:{}", dir));
        }
    }

    // Generate identity.md
    let identity_path = path.join("bench/identity.md");
    if !identity_path.exists() {
        let identity = generate_identity(config);
        std::fs::write(&identity_path, identity)?;
        created.push("file:bench/identity.md".to_string());
    } else {
        skipped.push("file:bench/identity.md".to_string());
    }

    // Generate methodology.md
    let method_path = path.join("bench/methodology.md");
    if !method_path.exists() {
        let method = generate_methodology(config);
        std::fs::write(&method_path, method)?;
        created.push("file:bench/methodology.md".to_string());
    }

    // Generate CLAUDE.md if editor is cursor or claude-code
    if config.editor == super::derive::Editor::Cursor
        || config.editor == super::derive::Editor::ClaudeCode
    {
        let claude_path = path.join("CLAUDE.md");
        if !claude_path.exists() {
            let claude = generate_claude_md(config);
            std::fs::write(&claude_path, claude)?;
            created.push("file:CLAUDE.md".to_string());
        }
    }

    // Generate .cursorignore if cursor editor
    if config.editor == super::derive::Editor::Cursor {
        let cursorignore = path.join(".cursorignore");
        if !cursorignore.exists() {
            std::fs::write(&cursorignore, generate_cursorignore())?;
            created.push("file:.cursorignore".to_string());
        }
    }

    // Generate policy files if security enabled
    if config.security_level != super::derive::SecurityLevel::None {
        generate_policy_files(path, &mut created)?;
    }

    // Generate state files
    let task_path = path.join("sawdust/state/current-task.md");
    if !task_path.exists() {
        std::fs::write(&task_path, "")?;
        created.push("file:sawdust/state/current-task.md".to_string());
    }

    let taint_path = path.join("sawdust/state/taint-state.txt");
    if !taint_path.exists() {
        std::fs::write(&taint_path, "TAINT_STATE: clean\n")?;
        created.push("file:sawdust/state/taint-state.txt".to_string());
    }

    // Initialize git if requested
    if config.create_git && !path.join(".git").exists() {
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()
            .ok();
        created.push("git:init".to_string());
    }

    Ok(GenerateResult { created, skipped })
}

/// Result of workshop generation
#[derive(Debug, Clone)]
pub struct GenerateResult {
    /// Files/directories created
    pub created: Vec<String>,
    /// Files/directories skipped (already existed)
    pub skipped: Vec<String>,
}

fn generate_identity(config: &DerivedConfig) -> String {
    let work_type = match config.work_type {
        super::derive::WorkType::Software => "software engineering",
        super::derive::WorkType::Research => "research",
        super::derive::WorkType::Writing => "writing",
        super::derive::WorkType::Mixed => "mixed work",
    };

    let features: Vec<&str> = [
        config.features.taint_analysis.then_some("taint-analysis"),
        config.features.cursor_hooks.then_some("cursor-hooks"),
        config.features.knowledge_base.then_some("knowledge-base"),
        config.features.showboat_docs.then_some("showboat"),
    ]
    .into_iter()
    .flatten()
    .collect();

    format!(
        r#"---
created: {}
name: {}
work_type: {}
security: {}
editor: {}
features: [{}]
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

## Guiding Principle

Ask me clarifying questions until you know what I want to build
and walk me through the setup step by step.
"#,
        chrono::Local::now().format("%Y-%m-%d"),
        config.name,
        config.work_type,
        config.security_level,
        config.editor,
        features.join(", "),
        work_type
    )
}

fn generate_methodology(config: &DerivedConfig) -> String {
    let code_section = if config.code_refs != super::derive::CodeRefMode::Disabled {
        r#"
## Code References

When extracting insights from code:
1. Include the file path and line numbers
2. Preserve the programming language context
3. Link to the original source
"#
    } else {
        ""
    };

    format!(
        r#"---
type: methodology
---

# 5 Cs Methodology

## Capture

Zero-friction capture to inbox. Don't organize, just capture.

## Cut

Extract atomic insights. One idea per shaving.
{}

## Carve

Find connections. Search across all shavings.
Link related ideas together.

## Chamfer

Update older notes with new context.
Keep knowledge current.

## Check

Validate everything. Run adversarial tests.
Ensure security and quality.
"#,
        code_section
    )
}

fn generate_claude_md(config: &DerivedConfig) -> String {
    format!(
        r#"# {}

{}

## Security

{}

## Commands

- `workshop cut <file>` - Extract insight
- `workshop carve <query>` - Search
- `workshop chamfer <shaving> <context>` - Update
- `workshop check` - Validate
"#,
        config.name,
        if config.features.taint_analysis {
            "Taint analysis is ENABLED. Protected paths will be monitored.\nData exfiltration to sinks will be blocked."
        } else {
            "Standard security mode."
        },
        match config.security_level {
            super::derive::SecurityLevel::None => "No taint analysis.",
            super::derive::SecurityLevel::Basic => "Basic taint analysis for secrets.",
            super::derive::SecurityLevel::Paranoid => "Maximum security. All data flows tracked.",
        },
    )
}

fn generate_cursorignore() -> String {
    r#"# Secrets
.secrets/
*.env
*.pem
*.key
credentials.*

# Workshop internal
.workshop/policy/

# Session data
sawdust/sessions/
"#
    .to_string()
}

fn generate_policy_files(path: &Path, created: &mut Vec<String>) -> Result<()> {
    let sources = r#"# Protected paths
# Add your sensitive files here

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

  - pattern: ".netrc"
    taint: critical
    description: "Netrc file"

  - pattern: "_netrc"
    taint: critical
    description: "Netrc file (Windows)"
"#;

    let sinks = r#"# Exfiltration sinks
# Commands that could leak data

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

  - command: "aws"
    block_if_tainted: true
    reason: "AWS CLI can upload data"

  - command: "gcloud"
    block_if_tainted: true
    reason: "GCloud CLI can upload data"
"#;

    let sources_path = path.join(".workshop/policy/sources.yaml");
    if !sources_path.exists() {
        std::fs::write(&sources_path, sources)?;
        created.push("file:.workshop/policy/sources.yaml".to_string());
    }

    let sinks_path = path.join(".workshop/policy/sinks.yaml");
    if !sinks_path.exists() {
        std::fs::write(&sinks_path, sinks)?;
        created.push("file:.workshop/policy/sinks.yaml".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::derive::{WorkType, SecurityLevel, Editor, Features};
    use super::super::detect::Environment;
    use tempfile::TempDir;

    fn test_config() -> DerivedConfig {
        DerivedConfig {
            name: "test-workshop".to_string(),
            work_type: WorkType::Software,
            code_refs: super::super::derive::CodeRefMode::Required,
            location: std::env::current_dir().unwrap(),
            security_level: SecurityLevel::Basic,
            editor: Editor::Cursor,
            create_git: false,
            features: Features {
                taint_analysis: true,
                cursor_hooks: false,
                knowledge_base: false,
                showboat_docs: false,
            },
            environment: Environment::default(),
        }
    }

    #[test]
    fn test_generate_workshop() {
        let temp_dir = TempDir::new().unwrap();
        let config = test_config();

        let result = generate_workshop(&config, temp_dir.path()).unwrap();

        assert!(!result.created.is_empty());
        assert!(temp_dir.path().join("bench").exists());
        assert!(temp_dir.path().join("shavings").exists());
        assert!(temp_dir.path().join("bench/identity.md").exists());
    }

    #[test]
    fn test_generate_identity() {
        let config = test_config();
        let identity = generate_identity(&config);

        assert!(identity.contains("test-workshop"));
        assert!(identity.contains("software"));
    }

    #[test]
    fn test_generate_methodology() {
        let config = test_config();
        let method = generate_methodology(&config);

        assert!(method.contains("5 Cs"));
        assert!(method.contains("Cut"));
        assert!(method.contains("Carve"));
    }

    #[test]
    fn test_generate_policy_files() {
        let temp_dir = TempDir::new().unwrap();
        // Create the directory structure first
        std::fs::create_dir_all(temp_dir.path().join(".workshop/policy")).unwrap();
        let mut created = Vec::new();

        generate_policy_files(temp_dir.path(), &mut created).unwrap();

        assert!(temp_dir.path().join(".workshop/policy/sources.yaml").exists());
        assert!(temp_dir.path().join(".workshop/policy/sinks.yaml").exists());
    }
}
