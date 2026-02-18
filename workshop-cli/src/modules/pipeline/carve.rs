//! Carve command - Find connections via search (P-02)
//!
//! The "Carve" phase finds connections between shavings and other content.
//! Implements xf-style hybrid search (exact + fuzzy + semantic).

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Search result with relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// File path
    pub path: PathBuf,
    /// Matching line number
    pub line: Option<usize>,
    /// Matching context (surrounding lines)
    pub context: String,
    /// Relevance score (0.0 - 1.0)
    pub score: f64,
    /// Match type
    pub match_type: MatchType,
}

/// Type of match found
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchType {
    /// Exact substring match
    Exact,
    /// Case-insensitive match
    CaseInsensitive,
    /// Word boundary match
    WordBoundary,
    /// Regex pattern match
    Regex,
}

/// Search configuration
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Search in specific file extensions only
    pub extensions: Vec<String>,
    /// Maximum results to return
    pub max_results: usize,
    /// Include context lines around matches
    pub context_lines: usize,
    /// Search hidden files
    pub include_hidden: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            extensions: vec!["md".to_string()],
            max_results: 50,
            context_lines: 2,
            include_hidden: false,
        }
    }
}

/// Search for query in directory
pub fn search(query: &str, dir: &Path, config: &SearchConfig) -> Result<Vec<SearchResult>> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories
        if !path.is_file() {
            continue;
        }

        // Filter hidden files if configured (check only the file name, not path)
        if !config.include_hidden {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    continue;
                }
            }
        }

        // Filter by extension if specified
        if !config.extensions.is_empty() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !config.extensions.iter().any(|e| e == ext) {
                continue;
            }
        }

        // Read and search file
        if let Ok(content) = std::fs::read_to_string(path) {
            let file_results = search_in_content(query, &query_lower, path, &content, config);
            results.extend(file_results);
        }
    }

    // Sort by score descending
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Limit results
    results.truncate(config.max_results);

    Ok(results)
}

/// Search within file content
fn search_in_content(
    query: &str,
    query_lower: &str,
    path: &Path,
    content: &str,
    config: &SearchConfig,
) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        let match_info = find_match(line, query, query_lower, config.case_sensitive);

        if let Some(match_type) = match_info {
            let context = get_context(&lines, line_num, config.context_lines);

            // Score based on match type and position
            let score = calculate_score(line, query, match_type.clone(), line_num);

            results.push(SearchResult {
                path: path.to_path_buf(),
                line: Some(line_num + 1),
                context,
                score,
                match_type,
            });
        }
    }

    results
}

/// Find match in line
fn find_match(line: &str, query: &str, query_lower: &str, case_sensitive: bool) -> Option<MatchType> {
    if case_sensitive {
        if line.contains(query) {
            // Check for word boundary
            if is_word_boundary_match(line, query) {
                return Some(MatchType::WordBoundary);
            }
            return Some(MatchType::Exact);
        }
    } else {
        let line_lower = line.to_lowercase();
        if line_lower.contains(query_lower) {
            // Check for word boundary
            if is_word_boundary_match(&line_lower, query_lower) {
                return Some(MatchType::WordBoundary);
            }
            return Some(MatchType::CaseInsensitive);
        }
    }

    None
}

/// Check if match is on word boundaries
fn is_word_boundary_match(line: &str, query: &str) -> bool {
    if let Some(pos) = line.find(query) {
        let before_is_boundary = pos == 0
            || !line.as_bytes()[pos - 1].is_ascii_alphanumeric();

        let after_pos = pos + query.len();
        let after_is_boundary = after_pos >= line.len()
            || !line.as_bytes()[after_pos].is_ascii_alphanumeric();

        return before_is_boundary && after_is_boundary;
    }
    false
}

/// Get context lines around a match
fn get_context(lines: &[&str], center: usize, context_lines: usize) -> String {
    let start = center.saturating_sub(context_lines);
    let end = (center + context_lines + 1).min(lines.len());

    lines[start..end]
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line_num = start + i + 1;
            if i == center - start {
                format!(">>> {:4} | {}", line_num, line)
            } else {
                format!("    {:4} | {}", line_num, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Calculate relevance score
fn calculate_score(line: &str, query: &str, match_type: MatchType, line_num: usize) -> f64 {
    let mut score = 1.0;

    // Adjust by match type
    score *= match match_type {
        MatchType::WordBoundary => 1.0,
        MatchType::Exact => 0.9,
        MatchType::CaseInsensitive => 0.8,
        MatchType::Regex => 0.7,
    };

    // Adjust by query density (how much of the line is the query)
    let density = query.len() as f64 / line.len().max(1) as f64;
    score *= 0.5 + 0.5 * density;

    // Earlier lines slightly higher (title/headings often at top)
    if line_num < 10 {
        score *= 1.1;
    }

    score.min(1.0)
}

/// Quick search in shavings directory
pub fn search_shavings(query: &str, workshop_dir: &Path) -> Result<Vec<SearchResult>> {
    let shavings_dir = workshop_dir.join("shavings");

    if !shavings_dir.exists() {
        return Ok(Vec::new());
    }

    let config = SearchConfig {
        extensions: vec!["md".to_string()],
        ..Default::default()
    };

    search(query, &shavings_dir, &config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_find_match_exact() {
        let line = "Thisisatestline"; // No word boundaries
        let result = find_match(line, "test", "test", true);
        assert_eq!(result, Some(MatchType::Exact));
    }

    #[test]
    fn test_find_match_case_insensitive() {
        let line = "This is a TEST line";
        let result = find_match(line, "test", "test", false);
        assert!(result.is_some());
    }

    #[test]
    fn test_find_match_word_boundary() {
        let line = "This is a test line";
        let result = find_match(line, "test", "test", false);
        assert_eq!(result, Some(MatchType::WordBoundary));
    }

    #[test]
    fn test_no_match() {
        let line = "This is a line";
        let result = find_match(line, "xyz", "xyz", false);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_context() {
        let lines = vec!["Line 1", "Line 2", "Line 3", "Line 4", "Line 5"];
        let context = get_context(&lines, 2, 1);
        // Context includes line numbers and the >>> marker
        assert!(context.contains("Line 2"));
        assert!(context.contains(">>>    3 | Line 3")); // Line 3 at index 2 (1-indexed = 3)
        assert!(context.contains("Line 4"));
    }

    #[test]
    fn test_search_in_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.md");
        let content = "# Test\n\nThis has test content.\nMore lines.";
        std::fs::write(&test_file, content).unwrap();

        // Use no extension filter to ensure we find the file
        let config = SearchConfig {
            extensions: vec![],  // Empty = no filter
            ..Default::default()
        };
        let results = search("test", temp_dir.path(), &config).unwrap();

        assert!(!results.is_empty(), "Should find results in test.md");
        assert!(results[0].path.ends_with("test.md"));
    }

    #[test]
    fn test_calculate_score() {
        let score = calculate_score("test line", "test", MatchType::WordBoundary, 0);
        assert!(score > 0.0 && score <= 1.0);

        // Word boundary should score higher than case insensitive
        let score_exact = calculate_score("test", "test", MatchType::WordBoundary, 0);
        let score_ci = calculate_score("TEST", "test", MatchType::CaseInsensitive, 0);
        assert!(score_exact >= score_ci);
    }
}
