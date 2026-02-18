//! Chartroom integration (W-05)
//!
//! Source: chartroom/

use anyhow::Result;
use std::path::PathBuf;

/// Chart type
#[derive(Debug, Clone, Copy)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
}

impl ChartType {
    pub fn to_str(&self) -> &'static str {
        match self {
            ChartType::Bar => "bar",
            ChartType::Line => "line",
            ChartType::Pie => "pie",
            ChartType::Scatter => "scatter",
        }
    }
}

/// Chart options
#[derive(Debug, Clone, Default)]
pub struct ChartOptions {
    pub title: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Create a chart with automatic alt-text generation
pub fn create_chart(
    data_path: &PathBuf,
    chart_type: ChartType,
    options: ChartOptions,
) -> Result<(PathBuf, String)> {
    let mut args = vec![
        "chartroom".to_string(),
        chart_type.to_str().to_string(),
        "--format".to_string(),
        "markdown".to_string(),
    ];

    if let Some(ref title) = options.title {
        args.push("--title".to_string());
        args.push(title.clone());
    }

    args.push(data_path.to_str().unwrap().to_string());

    let output = std::process::Command::new("uvx")
        .args(&args)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "chartroom failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse path and alt text from output
    let path = parse_chart_path(&stdout)?;
    let alt = parse_chart_alt(&stdout)?;

    Ok((path, alt))
}

fn parse_chart_path(output: &str) -> Result<PathBuf> {
    // Look for image path in markdown output
    for line in output.lines() {
        if line.starts_with("![") {
            if let Some(start) = line.find("](") {
                if let Some(end) = line[start + 2..].find(')') {
                    return Ok(PathBuf::from(&line[start + 2..start + 2 + end]));
                }
            }
        }
    }
    Err(anyhow::anyhow!("Could not parse chart path"))
}

fn parse_chart_alt(output: &str) -> Result<String> {
    // Look for alt text in markdown
    for line in output.lines() {
        if line.starts_with("![") {
            if let Some(end) = line[2..].find(']') {
                return Ok(line[2..2 + end].to_string());
            }
        }
    }
    Ok("Chart".to_string())
}
