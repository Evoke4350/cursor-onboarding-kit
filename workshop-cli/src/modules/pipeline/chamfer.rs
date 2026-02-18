//! Chamfer command - Update older notes with new context (P-03)
//!
//! The "Chamfer" phase updates existing shavings with new context,
//! keeping notes current as understanding evolves.

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use chrono::{DateTime, Utc};

/// Chamfer operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChamferResult {
    /// Original shaving path
    pub path: PathBuf,
    /// Whether update was successful
    pub updated: bool,
    /// New context added
    pub context_added: String,
    /// Timestamp of update
    pub updated_at: DateTime<Utc>,
    /// Number of references found
    pub references: usize,
}

/// Configuration for chamfer operation
#[derive(Debug, Clone)]
pub struct ChamferConfig {
    /// Append context rather than inline
    pub append_mode: bool,
    /// Marker for context section
    pub context_marker: String,
    /// Auto-find related shavings
    pub auto_link: bool,
}

impl Default for ChamferConfig {
    fn default() -> Self {
        Self {
            append_mode: true,
            context_marker: "## Context".to_string(),
            auto_link: true,
        }
    }
}

/// Update a shaving with new context
pub fn update_shaving(
    shaving_path: &Path,
    context: &str,
    config: &ChamferConfig,
) -> Result<ChamferResult> {
    if !shaving_path.exists() {
        bail!("Shaving not found: {}", shaving_path.display());
    }

    let content = fs::read_to_string(shaving_path)?;

    // Check if context section already exists
    let has_context_section = content.contains(&config.context_marker);

    let new_content = if config.append_mode {
        if has_context_section {
            // Append to existing context section
            append_to_context_section(&content, context, &config.context_marker)
        } else {
            // Add new context section
            format!(
                "{}\n\n{}\n\n{}\n",
                content.trim_end(),
                config.context_marker,
                context
            )
        }
    } else {
        // Inline mode: prepend context as a note
        format!(
            "> **Update {}**: {}\n\n{}",
            Utc::now().format("%Y-%m-%d"),
            context,
            content
        )
    };

    fs::write(shaving_path, &new_content)?;

    Ok(ChamferResult {
        path: shaving_path.to_path_buf(),
        updated: true,
        context_added: context.to_string(),
        updated_at: Utc::now(),
        references: 0, // TODO: Implement auto-link
    })
}

/// Append context to existing context section
fn append_to_context_section(content: &str, new_context: &str, marker: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    let mut found_context = false;
    let mut inserted = false;

    for (i, line) in lines.iter().enumerate() {
        result.push_str(line);
        result.push('\n');

        if line.starts_with(marker) && !found_context {
            found_context = true;
            // Check if next line is empty or a heading
            let next_is_heading = lines.get(i + 1)
                .map(|l| l.starts_with('#'))
                .unwrap_or(true);

            if !next_is_heading {
                // Add newline and context
                result.push('\n');
                result.push_str(new_context);
                result.push('\n');
                inserted = true;
            }
        }
    }

    if found_context && !inserted {
        // Context section was at end of file
        result.push('\n');
        result.push_str(new_context);
        result.push('\n');
    }

    result
}

/// Find shavings that might need updating based on search
pub fn find_stale_shavings(
    dir: &Path,
    query: &str,
    max_age_days: u64,
) -> Result<Vec<PathBuf>> {
    use super::carve::{search, SearchConfig};

    let config = SearchConfig {
        extensions: vec!["md".to_string()],
        ..Default::default()
    };

    let results = search(query, dir, &config)?;

    let cutoff = Utc::now() - chrono::Duration::days(max_age_days as i64);

    let stale: Vec<PathBuf> = results
        .into_iter()
        .filter_map(|r| {
            // Check file modification time
            if let Ok(metadata) = fs::metadata(&r.path) {
                if let Ok(modified) = metadata.modified() {
                    let modified_dt: DateTime<Utc> = modified.into();
                    if modified_dt < cutoff {
                        return Some(r.path);
                    }
                }
            }
            None
        })
        .collect();

    Ok(stale)
}

/// Add cross-reference link between shavings
pub fn add_cross_reference(
    source: &Path,
    target: &Path,
    link_text: &str,
) -> Result<()> {
    let content = fs::read_to_string(source)?;
    let target_name = target.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let link = format!(
        "\n\n---\n**See also:** [{}]({})",
        link_text,
        target.display()
    );

    // Check if link already exists
    if content.contains(&target.display().to_string()) {
        return Ok(());
    }

    fs::write(source, format!("{}{}", content.trim_end(), link))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_update_shaving_append_new_section() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("test.md");
        fs::write(&file, "# Test\n\nSome content.").unwrap();

        let config = ChamferConfig::default();
        let result = update_shaving(&file, "New context", &config).unwrap();

        assert!(result.updated);
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("## Context"));
        assert!(content.contains("New context"));
    }

    #[test]
    fn test_update_shaving_append_existing() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("test.md");
        fs::write(&file, "# Test\n\nSome content.\n\n## Context\n\nOld context.").unwrap();

        let config = ChamferConfig::default();
        let result = update_shaving(&file, "New context", &config).unwrap();

        assert!(result.updated);
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("Old context"));
        assert!(content.contains("New context"));
    }

    #[test]
    fn test_append_to_context_section() {
        let content = "# Title\n\nBody\n\n## Context\n\nOld";
        let result = append_to_context_section(content, "New", "## Context");
        assert!(result.contains("Old"));
        assert!(result.contains("New"));
    }

    #[test]
    fn test_nonexistent_file() {
        let config = ChamferConfig::default();
        let result = update_shaving(Path::new("/nonexistent/file.md"), "context", &config);
        assert!(result.is_err());
    }
}
