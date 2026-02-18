//! Cut command - Extract atomic insight (P-01)
//!
//! The "Cut" phase extracts atomic insights from source material.
//! A "shaving" is a single, self-contained insight extracted from raw content.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Extracted atomic insight (shaving)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shaving {
    /// Unique ID (UUID)
    pub id: String,
    /// Source file path
    pub source: PathBuf,
    /// Source line range (start, end)
    pub source_lines: Option<(usize, usize)>,
    /// Extracted text content
    pub text: String,
    /// Summary/title (one line)
    pub summary: String,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Optional code reference with language
    pub code_ref: Option<CodeReference>,
}

/// Code reference extracted from source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReference {
    /// Programming language
    pub language: String,
    /// Code snippet
    pub code: String,
    /// Function/class name if applicable
    pub symbol: Option<String>,
}

/// Configuration for cut operation
#[derive(Debug, Clone)]
pub struct CutConfig {
    /// Include code references
    pub with_code_ref: bool,
    /// Maximum text length (truncate if longer)
    pub max_text_length: usize,
    /// Auto-detect language from extension
    pub auto_detect_language: bool,
}

impl Default for CutConfig {
    fn default() -> Self {
        Self {
            with_code_ref: false,
            max_text_length: 2000,
            auto_detect_language: true,
        }
    }
}

/// Extract a shaving from source content
pub fn extract_shaving(
    source: &Path,
    content: &str,
    selection: Option<(usize, usize)>,
    config: &CutConfig,
) -> Result<Shaving> {
    let text = match selection {
        Some((start, end)) => {
            let lines: Vec<&str> = content.lines().collect();
            if start == 0 && end == 0 {
                content.to_string()
            } else if start <= end && end <= lines.len() {
                lines[start..=end.min(lines.len() - 1)].join("\n")
            } else {
                bail!("Invalid line range: {}-{}", start, end)
            }
        }
        None => content.to_string(),
    };

    let text = if text.len() > config.max_text_length {
        format!("{}...\n[truncated]", &text[..config.max_text_length])
    } else {
        text
    };

    // Generate summary from first meaningful line
    let summary = generate_summary(&text);

    // Extract code reference if requested
    let code_ref = if config.with_code_ref {
        extract_code_ref(source, &text, config.auto_detect_language)
    } else {
        None
    };

    // Extract tags from content
    let tags = extract_tags(&text);

    Ok(Shaving {
        id: uuid::Uuid::new_v4().to_string(),
        source: source.to_path_buf(),
        source_lines: selection,
        text,
        summary,
        tags,
        created_at: Utc::now(),
        code_ref,
    })
}

/// Generate a one-line summary from content
fn generate_summary(text: &str) -> String {
    let first_line = text
        .lines()
        .find(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("#")
                && !trimmed.starts_with("/*")
                && !trimmed.starts_with("*")
        })
        .unwrap_or("Untitled insight");

    // Truncate to reasonable length
    let summary = first_line.trim();
    if summary.len() > 80 {
        format!("{}...", &summary[..77])
    } else {
        summary.to_string()
    }
}

/// Extract code reference from content
fn extract_code_ref(
    source: &Path,
    text: &str,
    auto_detect: bool,
) -> Option<CodeReference> {
    let language = if auto_detect {
        detect_language(source)
    } else {
        "text".to_string()
    };

    // Check if content looks like code
    let code_indicators = [
        ("fn ", "rust"),
        ("function ", "javascript"),
        ("def ", "python"),
        ("class ", "object-oriented"),
        ("impl ", "rust"),
        ("pub ", "rust"),
        ("const ", "multi"),
        ("let ", "multi"),
        ("import ", "multi"),
        ("use ", "rust"),
    ];

    let is_code = code_indicators.iter().any(|(indicator, _)| text.contains(indicator));

    if !is_code && language == "text" {
        return None;
    }

    // Try to extract symbol name
    let symbol = extract_symbol(text);

    Some(CodeReference {
        language,
        code: text.to_string(),
        symbol,
    })
}

/// Detect programming language from file extension
fn detect_language(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => "rust".to_string(),
        Some("py") => "python".to_string(),
        Some("js") => "javascript".to_string(),
        Some("ts") => "typescript".to_string(),
        Some("go") => "go".to_string(),
        Some("java") => "java".to_string(),
        Some("md") => "markdown".to_string(),
        Some("json") => "json".to_string(),
        Some("yaml") | Some("yml") => "yaml".to_string(),
        Some("toml") => "toml".to_string(),
        _ => "text".to_string(),
    }
}

/// Extract function/class name from code
fn extract_symbol(text: &str) -> Option<String> {
    // Try common patterns
    let patterns = [
        (r"fn\s+(\w+)", 1),
        (r"function\s+(\w+)", 1),
        (r"def\s+(\w+)", 1),
        (r"class\s+(\w+)", 1),
        (r"struct\s+(\w+)", 1),
        (r"impl\s+(\w+)", 1),
        (r"pub\s+\w+\s+(\w+)", 1),
    ];

    for (pattern, group) in patterns {
        if let Ok(re) = regex_lite_match(pattern, text) {
            return Some(re);
        }
    }

    None
}

/// Simple regex-like match for symbol extraction (no regex crate needed)
fn regex_lite_match(pattern: &str, text: &str) -> Result<String> {
    // Very simple pattern matching for common cases
    if pattern.starts_with("fn ") {
        for line in text.lines() {
            if line.contains("fn ") {
                if let Some(start) = line.find("fn ") {
                    let rest = &line[start + 3..];
                    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !name.is_empty() {
                        return Ok(name);
                    }
                }
            }
        }
    } else if pattern.starts_with("def ") {
        for line in text.lines() {
            if line.contains("def ") {
                if let Some(start) = line.find("def ") {
                    let rest = &line[start + 4..];
                    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !name.is_empty() {
                        return Ok(name);
                    }
                }
            }
        }
    } else if pattern.starts_with("class ") {
        for line in text.lines() {
            if line.contains("class ") {
                if let Some(start) = line.find("class ") {
                    let rest = &line[start + 6..];
                    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if !name.is_empty() {
                        return Ok(name);
                    }
                }
            }
        }
    }
    bail!("No match found")
}

/// Extract tags from content (hashtags and keywords)
fn extract_tags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();

    // Find #hashtags
    for word in text.split_whitespace() {
        if word.starts_with('#') && word.len() > 1 {
            let tag = word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_string();
            if !tag.is_empty() {
                tags.push(format!("#{}", tag.to_lowercase()));
            }
        }
    }

    tags.sort();
    tags.dedup();
    tags
}

/// Save shaving to file
pub fn save_shaving(shaving: &Shaving, output_dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    let filename = format!(
        "{}-{}.md",
        shaving.created_at.format("%Y%m%d-%H%M%S"),
        sanitize_filename(&shaving.summary)
    );

    let path = output_dir.join(filename);

    let content = format!(
        "# {}\n\n{}\n\n---\n\n**Source:** `{}`\n**Created:** {}\n{}\n",
        shaving.summary,
        shaving.text,
        shaving.source.display(),
        shaving.created_at.to_rfc3339(),
        if shaving.tags.is_empty() {
            String::new()
        } else {
            format!("**Tags:** {}\n", shaving.tags.join(" "))
        }
    );

    std::fs::write(&path, content)?;
    Ok(path)
}

/// Sanitize string for use in filename
fn sanitize_filename(s: &str) -> String {
    let sanitized: String = s
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();

    let sanitized = sanitized.trim_matches('-').to_string();
    if sanitized.len() > 50 {
        sanitized[..50].to_string()
    } else {
        sanitized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_shaving_basic() {
        let content = "# Test Header\n\nThis is the content.\n\n#test";
        let config = CutConfig::default();
        let shaving = extract_shaving(
            Path::new("test.md"),
            content,
            None,
            &config,
        ).unwrap();

        assert!(shaving.text.contains("content"));
        assert!(!shaving.tags.is_empty());
    }

    #[test]
    fn test_extract_shaving_with_selection() {
        let content = "Line 1\nLine 2\nLine 3\nLine 4";
        let config = CutConfig::default();
        let shaving = extract_shaving(
            Path::new("test.txt"),
            content,
            Some((1, 2)),
            &config,
        ).unwrap();

        assert!(shaving.text.contains("Line 2"));
        assert!(shaving.text.contains("Line 3"));
        assert!(!shaving.text.contains("Line 1"));
    }

    #[test]
    fn test_generate_summary() {
        let text = "   \n// Comment\nActual content here\nMore stuff";
        let summary = generate_summary(text);
        assert!(summary.contains("Actual"));
    }

    #[test]
    fn test_extract_tags() {
        let text = "Some text #rust #code and #testing";
        let tags = extract_tags(text);
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"#rust".to_string()));
    }

    #[test]
    fn test_detect_language() {
        assert_eq!(detect_language(Path::new("test.rs")), "rust");
        assert_eq!(detect_language(Path::new("test.py")), "python");
        assert_eq!(detect_language(Path::new("test.unknown")), "text");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Hello World!"), "Hello-World");
        assert_eq!(sanitize_filename("a".repeat(100).as_str()).len(), 50);
    }
}
