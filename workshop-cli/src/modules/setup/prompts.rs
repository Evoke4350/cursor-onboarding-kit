//! Gum-based interactive prompts (O-01)
//!
//! Beautiful terminal prompts using gum for the setup wizard.
//! Falls back to basic stdin if gum is not available.

use anyhow::{Result, bail};
use std::process::Command;

/// Gum prompt wrapper
pub struct GumPrompts {
    available: bool,
}

impl GumPrompts {
    pub fn new() -> Self {
        Self {
            available: which::which("gum").is_ok(),
        }
    }

    /// Check if gum is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Display a styled header
    pub fn header(&self, text: &str) -> Result<()> {
        if self.available {
            Command::new("gum")
                .args(["style", "--foreground", "99", "--bold", text])
                .status()?;
        } else {
            println!("\n{}\n", text);
        }
        Ok(())
    }

    /// Display info text
    pub fn info(&self, text: &str) -> Result<()> {
        if self.available {
            Command::new("gum")
                .args(["style", "--foreground", "240", text])
                .status()?;
        } else {
            println!("{}", text);
        }
        Ok(())
    }

    /// Ask for text input
    pub fn input(&self, prompt: &str, placeholder: Option<&str>) -> Result<String> {
        if self.available {
            let mut cmd = Command::new("gum");
            cmd.args(["input", "--prompt", prompt]);

            if let Some(p) = placeholder {
                cmd.args(["--placeholder", p]);
            }

            let output = cmd.output()?;
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if result.is_empty() {
                bail!("Input required")
            }
            Ok(result)
        } else {
            println!("{}: ", prompt);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            Ok(input.trim().to_string())
        }
    }

    /// Ask for text input with default value
    pub fn input_with_default(&self, prompt: &str, default: &str) -> Result<String> {
        if self.available {
            let output = Command::new("gum")
                .args([
                    "input",
                    "--prompt", prompt,
                    "--placeholder", default,
                    "--value", default,
                ])
                .output()?;

            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(if result.is_empty() { default.to_string() } else { result })
        } else {
            println!("{} [{}]: ", prompt, default);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            Ok(if trimmed.is_empty() { default.to_string() } else { trimmed.to_string() })
        }
    }

    /// Select from a list of options
    pub fn select(&self, prompt: &str, options: &[&str]) -> Result<String> {
        if self.available {
            let output = Command::new("gum")
                .args(["choose", "--header", prompt])
                .args(options)
                .output()?;

            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if result.is_empty() {
                bail!("Selection required")
            }
            Ok(result)
        } else {
            println!("{}", prompt);
            for (i, opt) in options.iter().enumerate() {
                println!("  {}. {}", i + 1, opt);
            }
            println!("Enter choice [1-{}]: ", options.len());

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let idx: usize = input.trim().parse()
                .map_err(|_| anyhow::anyhow!("Invalid number"))?;

            if idx < 1 || idx > options.len() {
                bail!("Invalid choice")
            }
            Ok(options[idx - 1].to_string())
        }
    }

    /// Yes/No confirmation
    pub fn confirm(&self, prompt: &str, default: bool) -> Result<bool> {
        if self.available {
            let default_str = if default { "Y/n" } else { "y/N" };
            let output = Command::new("gum")
                .args([
                    "confirm",
                    "--default", default_str,
                    prompt,
                ])
                .status()?;

            Ok(output.success())
        } else {
            let default_str = if default { "Y/n" } else { "y/N" };
            println!("{} [{}]: ", prompt, default_str);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let trimmed = input.trim().to_lowercase();
            if trimmed.is_empty() {
                Ok(default)
            } else {
                Ok(trimmed == "y" || trimmed == "yes")
            }
        }
    }

    /// Multi-select from options
    pub fn multi_select(&self, prompt: &str, options: &[&str]) -> Result<Vec<String>> {
        if self.available {
            let output = Command::new("gum")
                .args(["choose", "--header", prompt, "--no-limit"])
                .args(options)
                .output()?;

            let result = String::from_utf8_lossy(&output.stdout);
            Ok(result.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        } else {
            println!("{}", prompt);
            for (i, opt) in options.iter().enumerate() {
                println!("  {}. {}", i + 1, opt);
            }
            println!("Enter choices (comma-separated, e.g., 1,3,4): ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let selected: Result<Vec<String>, _> = input
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .filter(|&idx| idx >= 1 && idx <= options.len())
                .map(|idx| Ok(options[idx - 1].to_string()))
                .collect();

            selected
        }
    }

    /// Show a spinner while running a command
    pub fn spinner(&self, title: &str, command: &mut Command) -> Result<bool> {
        if self.available {
            let status = Command::new("gum")
                .args(["spin", "--title", title, "--"])
                .arg(command.get_program())
                .args(command.get_args())
                .status()?;

            Ok(status.success())
        } else {
            println!("{}...", title);
            let status = command.status()?;
            Ok(status.success())
        }
    }

    /// Write text with optional styling
    pub fn write(&self, text: &str, bold: bool, color: Option<&str>) -> Result<()> {
        if self.available {
            let mut args = vec!["style"];
            if bold {
                args.push("--bold");
            }
            if let Some(c) = color {
                args.extend_from_slice(&["--foreground", c]);
            }
            args.push(text);

            Command::new("gum").args(&args).status()?;
        } else {
            println!("{}", text);
        }
        Ok(())
    }
}

impl Default for GumPrompts {
    fn default() -> Self {
        Self::new()
    }
}

/// Workshop setup prompts
pub fn prompt_workshop_config(prompts: &GumPrompts) -> Result<WorkshopConfig> {
    // Welcome header
    prompts.header("🛠️  Workshop Setup")?;
    prompts.info("Ask me clarifying questions until you know what I want\nto build and walk me through the setup step by step.\n")?;

    // Phase 1: Detect (automatic)
    prompts.info("Phase 1: Detecting environment...")?;
    // Environment detection happens automatically

    // Phase 2: Understand
    prompts.header("\n📋 Phase 2: Understand")?;

    let work_type = prompts.select(
        "What type of work will you do in this workshop?",
        &["software", "research", "writing", "mixed"],
    )?;

    let security_level = prompts.select(
        "Security level for taint analysis?",
        &["none", "basic", "paranoid"],
    )?;

    let editor = prompts.select(
        "Primary editor?",
        &["cursor", "claude-code", "vscode", "other"],
    )?;

    // Phase 3: Derive
    prompts.header("\n🔧 Phase 3: Configure")?;

    let workshop_name = prompts.input_with_default(
        "Workshop name:",
        "my-workshop",
    )?;

    let create_git = prompts.confirm(
        "Initialize git repository?",
        true,
    )?;

    let features = prompts.multi_select(
        "Select features to enable:",
        &[
            "taint-analysis",
            "cursor-hooks",
            "knowledge-base",
            "showboat-docs",
        ],
    )?;

    // Phase 4: Confirm
    prompts.header("\n✅ Phase 4: Confirm")?;

    prompts.info(&format!(
        "Workshop: {}\nType: {}\nSecurity: {}\nEditor: {}\nGit: {}\nFeatures: {}",
        workshop_name,
        work_type,
        security_level,
        editor,
        if create_git { "yes" } else { "no" },
        features.join(", "),
    ))?;

    let confirm = prompts.confirm("Create workshop with these settings?", true)?;

    if !confirm {
        bail!("Setup cancelled")
    }

    Ok(WorkshopConfig {
        name: workshop_name,
        work_type,
        security_level,
        editor,
        create_git,
        features,
    })
}

/// Workshop configuration from prompts
#[derive(Debug, Clone)]
pub struct WorkshopConfig {
    pub name: String,
    pub work_type: String,
    pub security_level: String,
    pub editor: String,
    pub create_git: bool,
    pub features: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gum_prompts_creation() {
        let prompts = GumPrompts::new();
        // May or may not have gum, just check it doesn't crash
        assert!(prompts.is_available() || !prompts.is_available());
    }

    #[test]
    fn test_workshop_config() {
        let config = WorkshopConfig {
            name: "test".to_string(),
            work_type: "software".to_string(),
            security_level: "basic".to_string(),
            editor: "cursor".to_string(),
            create_git: true,
            features: vec!["taint-analysis".to_string()],
        };

        assert_eq!(config.name, "test");
        assert_eq!(config.features.len(), 1);
    }
}
