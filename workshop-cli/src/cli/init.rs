//! Init command - Initialize a new workshop
//!
//! Uses O-01 through O-04 modules:
//! - O-01: Gum prompts (prompts.rs)
//! - O-02: Environment detection (detect.rs)
//! - O-03: Derive configuration (derive.rs)
//! - O-04: Generate workshop structure (generate.rs)

use anyhow::Result;
use std::path::PathBuf;

use crate::modules::setup::{
    prompts::GumPrompts,
    detect::{detect_environment_in, Environment},
    derive::{DerivedConfig, WorkType, SecurityLevel, Editor, CodeRefMode, Features},
    generate::generate_workshop,
};

pub fn run(
    path: String,
    non_interactive: bool,
    work_type: Option<String>,
    security: Option<String>,
    robot: bool,
) -> Result<()> {
    let target_path = PathBuf::from(&path);

    // O-02: Detect environment
    let env = detect_environment_in(&target_path);

    if robot {
        let json = serde_json::json!({
            "status": "success",
            "command": "init",
            "path": path,
            "environment": {
                "os": env.os,
                "has_gum": env.has_gum,
                "has_git": env.has_git,
                "project_type": env.project_type,
            }
        });
        println!("{}", serde_json::to_string(&json)?);
        return Ok(());
    }

    println!("🛠️  Workshop Init");
    println!("   Target: {}", path);
    println!();

    // Check for missing tools
    if !env.missing_tools.is_empty() {
        println!("❌ Missing required tools: {}", env.missing_tools.join(", "));
        return Ok(());
    }

    if !env.missing_recommended.is_empty() {
        println!("⚠️  Missing recommended tools: {}", env.missing_recommended.join(", "));
        println!("   (Some features may be limited)");
        println!();
    }

    // O-03: Derive configuration
    let config = if non_interactive {
        // Use defaults for non-interactive mode
        DerivedConfig {
            name: target_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("workshop")
                .to_string(),
            work_type: parse_work_type(work_type.as_deref()),
            code_refs: CodeRefMode::Optional,
            location: target_path.clone(),
            security_level: parse_security_level(security.as_deref()),
            editor: detect_editor(&env),
            create_git: true,
            features: Features::default(),
            environment: env.clone(),
        }
    } else {
        // O-01: Interactive prompts
        interactive_setup(&env, &target_path)?
    };

    println!("📋 Configuration:");
    println!("   Work type: {}", config.work_type);
    println!("   Security: {}", config.security_level);
    println!("   Editor: {}", config.editor);
    println!();

    // O-04: Generate workshop structure
    println!("📁 Creating workshop structure...");
    let result = generate_workshop(&config, &target_path)?;

    println!();
    println!("✅ Workshop initialized!");
    println!("   Created: {} items", result.created.len());
    println!("   Skipped: {} items (already exist)", result.skipped.len());

    if !result.created.is_empty() {
        println!();
        println!("   Created files:");
        for item in &result.created {
            println!("     + {}", item);
        }
    }

    println!();
    println!("   Next steps:");
    println!("     1. cd {}", path);
    println!("     2. Edit bench/identity.md with your project details");
    println!("     3. Run 'workshop cut <file>' to create your first shaving");

    Ok(())
}

/// Interactive setup using gum prompts
fn interactive_setup(env: &Environment, target_path: &PathBuf) -> Result<DerivedConfig> {
    let prompts = GumPrompts::new();

    prompts.header("🛠️  Workshop Setup")?;
    prompts.info("Let's configure your workshop.")?;
    println!();

    // Ask work type
    let work_type_str = if prompts.is_available() {
        prompts.select("What type of work?", &["software", "research", "writing"])?
    } else {
        println!("Work type [software/research/writing] (default: software): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        if choice.is_empty() { "software".to_string() } else { choice.to_string() }
    };
    let work_type = parse_work_type(Some(&work_type_str));

    // Ask security level
    let security_str = if prompts.is_available() {
        prompts.select("Security level?", &["none", "basic", "paranoid"])?
    } else {
        println!("Security level [none/basic/paranoid] (default: basic): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        if choice.is_empty() { "basic".to_string() } else { choice.to_string() }
    };
    let security_level = parse_security_level(Some(&security_str));

    // Ask about git
    let create_git = if prompts.is_available() {
        prompts.confirm("Initialize git repository?", true)?
    } else {
        println!("Initialize git? [Y/n] (default: Y): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        !input.trim().to_lowercase().starts_with('n')
    };

    Ok(DerivedConfig {
        name: target_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("workshop")
            .to_string(),
        work_type,
        code_refs: CodeRefMode::Optional,
        location: target_path.clone(),
        security_level,
        editor: detect_editor(env),
        create_git,
        features: Features::default(),
        environment: env.clone(),
    })
}

fn parse_work_type(s: Option<&str>) -> WorkType {
    match s.map(|s| s.to_lowercase()).as_deref() {
        Some("research") => WorkType::Research,
        Some("writing") => WorkType::Writing,
        _ => WorkType::Software,
    }
}

fn parse_security_level(s: Option<&str>) -> SecurityLevel {
    match s.map(|s| s.to_lowercase()).as_deref() {
        Some("basic") => SecurityLevel::Basic,
        Some("paranoid") => SecurityLevel::Paranoid,
        _ => SecurityLevel::None,
    }
}

fn detect_editor(env: &Environment) -> Editor {
    if env.cursor_installed {
        Editor::Cursor
    } else if which::which("claude-code").is_ok() || env.claude_code {
        Editor::ClaudeCode
    } else {
        Editor::Other
    }
}
