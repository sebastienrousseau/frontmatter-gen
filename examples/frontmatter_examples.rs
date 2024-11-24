// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Front Matter Examples
//!
//! This module demonstrates various use cases of front matter extraction using
//! the `frontmatter-gen` library, including:
//!
//! - Basic 404 page example
//! - YAML, TOML, and JSON front matter extraction
//! - Error handling for invalid formats
//! - Handling complex structures and nested content
//! - Advanced scenarios and edge cases
//!
//! ## Usage
//!
//! To run this example:
//!
//! ```bash
//! cargo run --features default --example frontmatter_examples
//! ```

use anyhow::{Context, Result};
use env_logger::{Builder, Env};
use frontmatter_gen::extract;
use log::{error, info};

/// Demonstrates extracting front matter for a 404 page.
///
/// This example highlights how front matter can be extracted and processed in a typical 404 page.
///
/// # Errors
/// Returns an error if front matter extraction fails.
fn example_404() -> Result<()> {
    info!("Running 404 page example");

    let env = Env::default().default_filter_or("warn");
    if let Err(e) = Builder::from_env(env).try_init() {
        error!("Logger already initialized: {}", e);
    }

    let content = r#"---
title: Not Found
---
<div style="text-align:center;">
    <img src="https://http.cat/images/404.jpg" alt="404 Not Found">
</div>
"#;

    let (frontmatter, remaining) = extract(content)
        .context("Failed to extract front matter from 404 page")?;

    println!("\n📄 404 Page Example:");
    println!("Front Matter:");
    println!("{:#?}", frontmatter);
    println!("\nContent:");
    println!("{}", remaining);

    Ok(())
}

/// Demonstrates extracting YAML front matter from content.
///
/// This example shows how to extract front matter written in YAML format.
///
/// # Errors
/// Returns an error if YAML front matter extraction fails.
fn example_yaml() -> Result<()> {
    info!("Testing YAML front matter");
    let content = r#"---
title: Example Page
date: 2024-11-24
tags:
  - rust
  - frontmatter
description: >
  A multi-line description
  spanning multiple lines.
---
# Main Content

This is the main content of the page."#;

    let (frontmatter, remaining) = extract(content)
        .context("Failed to extract YAML front matter")?;

    println!("\n📜 YAML Front Matter Example:");
    println!("Front Matter:");
    println!("{:#?}", frontmatter);
    println!("\nContent:");
    println!("{}", remaining);
    Ok(())
}

/// Demonstrates extracting TOML front matter from content.
///
/// This example shows how to extract front matter written in TOML format.
///
/// # Errors
/// Returns an error if TOML front matter extraction fails.
fn example_toml() -> Result<()> {
    info!("Testing TOML front matter");
    let content = r#"+++
title = "Example Page"
date = 2024-11-24
tags = ["rust", "frontmatter"]
[author]
name = "John Doe"
email = "john@example.com"
+++
# Main Content

This is the main content with TOML front matter."#;

    let (frontmatter, remaining) = extract(content)
        .context("Failed to extract TOML front matter")?;

    println!("\n📜 TOML Front Matter Example:");
    println!("Front Matter:");
    println!("{:#?}", frontmatter);
    println!("\nContent:");
    println!("{}", remaining);
    Ok(())
}

/// Demonstrates extracting JSON front matter from content.
///
/// This example shows how to extract front matter written in JSON format.
///
/// # Errors
/// Returns an error if JSON front matter extraction fails.
fn example_json() -> Result<()> {
    info!("Testing JSON front matter");
    let content = r#"{
    "title": "Example Page",
    "date": "2024-11-24",
    "tags": ["rust", "frontmatter"],
    "metadata": {
        "author": "John Doe",
        "version": 1.0
    }
}
# Main Content

This is the main content with JSON front matter."#;

    let (frontmatter, remaining) = extract(content)
        .context("Failed to extract JSON front matter")?;

    println!("\n📜 JSON Front Matter Example:");
    println!("Front Matter:");
    println!("{:#?}", frontmatter);
    println!("\nContent:");
    println!("{}", remaining);
    Ok(())
}

/// Main function demonstrating all examples.
///
/// This function sequentially runs all examples, covering YAML, TOML, JSON, and additional scenarios.
///
/// # Errors
/// Returns an error if any example fails.
pub fn main() -> Result<()> {
    println!("🧪 Front Matter Examples\n");

    // Run all examples
    example_404()?;
    example_yaml()?;
    example_toml()?;
    example_json()?;

    info!("All examples completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests extracting front matter from a 404 page.
    #[test]
    fn test_404_example() -> Result<()> {
        example_404()
    }

    /// Tests extracting YAML front matter.
    #[test]
    fn test_yaml_extraction() -> Result<()> {
        example_yaml()
    }

    /// Tests extracting TOML front matter.
    #[test]
    fn test_toml_extraction() -> Result<()> {
        example_toml()
    }

    /// Tests extracting JSON front matter.
    #[test]
    fn test_json_extraction() -> Result<()> {
        example_json()
    }

    /// Tests handling invalid front matter formats.
    #[test]
    fn test_invalid_formats() {
        // Invalid YAML
        let invalid_yaml = r#"---
invalid: : value
---"#;
        assert!(extract(invalid_yaml).is_err());

        // Invalid TOML
        let invalid_toml = r#"+++
invalid = = value
+++"#;
        assert!(extract(invalid_toml).is_err());

        // Invalid JSON
        let invalid_json = r#"{
    "invalid": : value
}"#;
        assert!(extract(invalid_json).is_err());
    }

    /// Tests handling complex structures in front matter.
    #[test]
    fn test_complex_structures() -> Result<()> {
        // Complex YAML
        let yaml = r#"---
nested:
  key: value
  array:
    - item1
    - item2
---"#;
        let (frontmatter, _) = extract(yaml)?;
        assert!(frontmatter.contains_key("nested"));

        // Complex TOML
        let toml = r#"+++
[table]
key = "value"
array = ["item1", "item2"]
+++"#;
        let (frontmatter, _) = extract(toml)?;
        assert!(frontmatter.contains_key("table"));

        // Complex JSON
        let json = r#"{
    "nested": {
        "key": "value",
        "array": ["item1", "item2"]
    }
}"#;
        let (frontmatter, _) = extract(json)?;
        assert!(frontmatter.contains_key("nested"));

        Ok(())
    }
}
