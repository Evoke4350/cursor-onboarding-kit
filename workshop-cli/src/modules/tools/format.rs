//! Tool definition format (T-01)
//!
//! Port from: templates.md:12-49

use serde::{Deserialize, Serialize};

/// Tool definition with YAML frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// YAML frontmatter
    pub frontmatter: ToolFrontmatter,
    /// Markdown body
    pub body: String,
}

/// YAML frontmatter for tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFrontmatter {
    /// Tool name
    pub name: String,
    /// Description
    pub description: String,
    /// Category
    pub category: String,
    /// Required tools
    #[serde(default)]
    pub requires: Vec<String>,
    /// Performance budget
    #[serde(default)]
    pub performance: Option<PerformanceBudget>,
}

/// Performance budget specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBudget {
    pub target_ms: u64,
    pub warning_ms: u64,
    pub panic_ms: u64,
}

/// Parse tool from markdown with YAML frontmatter
pub fn parse_tool(content: &str) -> anyhow::Result<ToolDefinition> {
    let content = content.trim();

    // Check for frontmatter delimiters
    if !content.starts_with("---\n") {
        return Err(anyhow::anyhow!("Missing YAML frontmatter"));
    }

    // Find closing delimiter
    let end_idx = content[4..]
        .find("\n---\n")
        .map(|i| i + 4)
        .ok_or_else(|| anyhow::anyhow!("Unclosed frontmatter"))?;

    let frontmatter_str = &content[4..end_idx];
    let body = content[end_idx + 5..].to_string();

    let frontmatter: ToolFrontmatter = serde_yaml::from_str(frontmatter_str)?;

    Ok(ToolDefinition { frontmatter, body })
}
