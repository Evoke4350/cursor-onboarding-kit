//! Feature block templates (T-02)
//!
//! Port from: templates.md:17-82

use std::collections::HashMap;

/// Composable feature block
#[derive(Debug, Clone)]
pub struct FeatureBlock {
    pub name: String,
    pub template: String,
    pub variables: HashMap<String, String>,
}

impl FeatureBlock {
    /// Create a new feature block
    pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            template: template.into(),
            variables: HashMap::new(),
        }
    }

    /// Set a variable
    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Render the template with variables
    pub fn render(&self) -> String {
        let mut result = self.template.clone();
        for (key, value) in &self.variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
}

/// Core tool templates
pub fn core_templates() -> HashMap<String, FeatureBlock> {
    let mut templates = HashMap::new();

    templates.insert(
        "cut".to_string(),
        FeatureBlock::new("cut", include_cut_template()),
    );

    templates.insert(
        "carve".to_string(),
        FeatureBlock::new("carve", include_carve_template()),
    );

    templates.insert(
        "chamfer".to_string(),
        FeatureBlock::new("chamfer", include_chamfer_template()),
    );

    templates.insert(
        "check".to_string(),
        FeatureBlock::new("check", include_check_template()),
    );

    templates
}

fn include_cut_template() -> String {
    r#"---
name: cut
description: Extract atomic insight from source
category: core
requires: [ripgrep]
performance:
  target_ms: 1
  warning_ms: 10
  panic_ms: 50
---

# /cut - Extract Insight

## Usage
```bash
workshop cut <source> [--with-code-ref] [--output <path>]
```

## Description
Extracts an atomic insight from the specified source file.
Creates a new shaving in `shavings/` with the extracted content.

## Options
- `--with-code-ref` - Include code references (file:line)
- `--output <path>` - Specify output path

## Example
```bash
workshop cut src/main.rs --with-code-ref
```
"#.to_string()
}

fn include_carve_template() -> String {
    r#"---
name: carve
description: Find connections via search
category: core
requires: [ripgrep]
performance:
  target_ms: 75
  warning_ms: 200
  panic_ms: 500
---

# /carve - Find Connections

## Usage
```bash
workshop carve <query> [--domain <domain>]
```

## Description
Searches all shavings for connections to the query.
Uses ripgrep for fast lexical search.

## Example
```bash
workshop carve "authentication"
```
"#.to_string()
}

fn include_chamfer_template() -> String {
    r#"---
name: chamfer
description: Update older shavings with new context
category: core
performance:
  target_ms: 5
  warning_ms: 20
  panic_ms: 100
---

# /chamfer - Update Work

## Usage
```bash
workshop chamfer <shaving> <context>
```

## Description
Updates an older shaving with new context or information.

## Example
```bash
workshop chamfer shavings/2026-02-18-auth.md "Updated: now uses JWT"
```
"#.to_string()
}

fn include_check_template() -> String {
    r#"---
name: check
description: Validate everything
category: core
performance:
  target_ms: 10
  warning_ms: 50
  panic_ms: 200
---

# /check - Validate

## Usage
```bash
workshop check [--adversarial] [--verify]
```

## Description
Validates workshop state, runs health checks.

## Options
- `--adversarial` - Run adversarial test suite
- `--verify` - Run showboat verify on all shavings

## Example
```bash
workshop check --verify
```
"#.to_string()
}
