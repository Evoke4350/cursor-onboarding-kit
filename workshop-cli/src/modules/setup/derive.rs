//! Configuration derivation (O-03)
//!
//! Port from: claude-md.md:116-125
//!
//! Phase 2 of setup: Understand user needs and derive configuration.
//! Converts WorkshopConfig from prompts into DerivedConfig for generation.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use super::prompts::WorkshopConfig;
use super::detect::Environment;

/// Work type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WorkType {
    #[default]
    Software,
    Research,
    Writing,
    Mixed,
}

impl std::fmt::Display for WorkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkType::Software => write!(f, "software"),
            WorkType::Research => write!(f, "research"),
            WorkType::Writing => write!(f, "writing"),
            WorkType::Mixed => write!(f, "mixed"),
        }
    }
}

/// Code reference mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CodeRefMode {
    #[default]
    Optional,
    Required,
    Disabled,
}

impl std::fmt::Display for CodeRefMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeRefMode::Optional => write!(f, "optional"),
            CodeRefMode::Required => write!(f, "required"),
            CodeRefMode::Disabled => write!(f, "disabled"),
        }
    }
}

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    #[default]
    Basic,
    None,
    Paranoid,
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityLevel::Basic => write!(f, "basic"),
            SecurityLevel::None => write!(f, "none"),
            SecurityLevel::Paranoid => write!(f, "paranoid"),
        }
    }
}

/// Editor type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Editor {
    #[default]
    Cursor,
    ClaudeCode,
    VSCode,
    Other,
}

impl std::fmt::Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Editor::Cursor => write!(f, "cursor"),
            Editor::ClaudeCode => write!(f, "claude-code"),
            Editor::VSCode => write!(f, "vscode"),
            Editor::Other => write!(f, "other"),
        }
    }
}

/// Feature flags derived from config
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Features {
    pub taint_analysis: bool,
    pub cursor_hooks: bool,
    pub knowledge_base: bool,
    pub showboat_docs: bool,
}

/// Derived configuration from user answers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedConfig {
    /// Workshop name
    pub name: String,
    /// Work type
    pub work_type: WorkType,
    /// Code reference mode
    pub code_refs: CodeRefMode,
    /// Workshop location
    pub location: PathBuf,
    /// Security level
    pub security_level: SecurityLevel,
    /// Primary editor
    pub editor: Editor,
    /// Initialize git
    pub create_git: bool,
    /// Feature flags
    pub features: Features,
    /// Environment detected
    pub environment: Environment,
}

impl DerivedConfig {
    /// Create default config with environment
    pub fn new(env: Environment) -> Self {
        Self {
            name: "my-workshop".to_string(),
            work_type: WorkType::default(),
            code_refs: CodeRefMode::default(),
            location: std::env::current_dir().unwrap_or_default(),
            security_level: SecurityLevel::default(),
            editor: Editor::default(),
            create_git: true,
            features: Features::default(),
            environment: env,
        }
    }

    /// Derive configuration from WorkshopConfig (from prompts)
    pub fn from_workshop_config(config: WorkshopConfig, env: Environment) -> Self {
        let work_type = match config.work_type.as_str() {
            "research" => WorkType::Research,
            "writing" => WorkType::Writing,
            "mixed" => WorkType::Mixed,
            _ => WorkType::Software,
        };

        let security_level = match config.security_level.as_str() {
            "none" => SecurityLevel::None,
            "paranoid" => SecurityLevel::Paranoid,
            _ => SecurityLevel::Basic,
        };

        let editor = match config.editor.as_str() {
            "claude-code" => Editor::ClaudeCode,
            "vscode" => Editor::VSCode,
            "other" => Editor::Other,
            _ => Editor::Cursor,
        };

        let features = Features {
            taint_analysis: config.features.contains(&"taint-analysis".to_string()),
            cursor_hooks: config.features.contains(&"cursor-hooks".to_string()),
            knowledge_base: config.features.contains(&"knowledge-base".to_string()),
            showboat_docs: config.features.contains(&"showboat-docs".to_string()),
        };

        // Derive code_refs from work type
        let code_refs = match work_type {
            WorkType::Software => CodeRefMode::Required,
            WorkType::Mixed => CodeRefMode::Optional,
            _ => CodeRefMode::Disabled,
        };

        Self {
            name: config.name,
            work_type,
            code_refs,
            location: std::env::current_dir().unwrap_or_default(),
            security_level,
            editor,
            create_git: config.create_git,
            features,
            environment: env,
        }
    }

    /// Derive configuration from question answers (legacy)
    pub fn from_answers(answers: std::collections::HashMap<String, String>) -> Self {
        let env = Environment::default(); // Will be empty
        let mut config = Self::new(env);

        if let Some(wt) = answers.get("work_type") {
            config.work_type = match wt.to_lowercase().as_str() {
                "research" => WorkType::Research,
                "writing" => WorkType::Writing,
                "mixed" => WorkType::Mixed,
                _ => WorkType::Software,
            };
        }

        if let Some(cr) = answers.get("code_refs") {
            config.code_refs = match cr.to_lowercase().as_str() {
                "required" | "yes" => CodeRefMode::Required,
                "disabled" | "no" => CodeRefMode::Disabled,
                _ => CodeRefMode::Optional,
            };
        }

        if let Some(loc) = answers.get("location") {
            config.location = PathBuf::from(loc);
        }

        if let Some(sec) = answers.get("security") {
            config.security_level = match sec.to_lowercase().as_str() {
                "none" => SecurityLevel::None,
                "paranoid" => SecurityLevel::Paranoid,
                _ => SecurityLevel::Basic,
            };
        }

        if let Some(name) = answers.get("name") {
            config.name = name.clone();
        }

        config
    }

    /// Get summary for display
    pub fn summary(&self) -> String {
        let features: Vec<&str> = [
            self.features.taint_analysis.then_some("taint"),
            self.features.cursor_hooks.then_some("hooks"),
            self.features.knowledge_base.then_some("knowledge"),
            self.features.showboat_docs.then_some("showboat"),
        ]
        .into_iter()
        .flatten()
        .collect();

        let features_str = if features.is_empty() {
            "none".to_string()
        } else {
            features.join(", ")
        };

        format!(
            "Workshop: {}\n\
             Type: {}\n\
             Security: {}\n\
             Editor: {}\n\
             Code refs: {}\n\
             Git: {}\n\
             Features: {}",
            self.name,
            self.work_type,
            self.security_level,
            self.editor,
            self.code_refs,
            if self.create_git { "yes" } else { "no" },
            features_str,
        )
    }
}

impl Default for DerivedConfig {
    fn default() -> Self {
        Self::new(Environment::default())
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            home_exists: true,
            has_gum: false,
            has_ripgrep: false,
            has_uvx: false,
            has_git: false,
            cursor_installed: false,
            cursor_db_exists: false,
            claude_code: false,
            project_type: None,
            missing_tools: vec![],
            missing_recommended: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_workshop_config() {
        let config = WorkshopConfig {
            name: "test-workshop".to_string(),
            work_type: "software".to_string(),
            security_level: "basic".to_string(),
            editor: "cursor".to_string(),
            create_git: true,
            features: vec!["taint-analysis".to_string(), "knowledge-base".to_string()],
        };

        let env = Environment::default();
        let derived = DerivedConfig::from_workshop_config(config, env);

        assert_eq!(derived.name, "test-workshop");
        assert_eq!(derived.work_type, WorkType::Software);
        assert_eq!(derived.security_level, SecurityLevel::Basic);
        assert!(derived.features.taint_analysis);
        assert!(derived.features.knowledge_base);
        assert!(!derived.features.cursor_hooks);
    }

    #[test]
    fn test_from_answers() {
        let mut answers = std::collections::HashMap::new();
        answers.insert("work_type".to_string(), "research".to_string());
        answers.insert("security".to_string(), "paranoid".to_string());
        answers.insert("name".to_string(), "my-research".to_string());

        let config = DerivedConfig::from_answers(answers);

        assert_eq!(config.work_type, WorkType::Research);
        assert_eq!(config.security_level, SecurityLevel::Paranoid);
        assert_eq!(config.name, "my-research");
    }

    #[test]
    fn test_work_type_display() {
        assert_eq!(WorkType::Software.to_string(), "software");
        assert_eq!(WorkType::Research.to_string(), "research");
    }

    #[test]
    fn test_summary() {
        let config = DerivedConfig::default();
        let summary = config.summary();
        assert!(summary.contains("Workshop:"));
        assert!(summary.contains("Type:"));
    }
}
