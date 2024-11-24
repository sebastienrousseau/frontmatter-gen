// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Library Examples
//!
//! This example demonstrates the core functionality of the FrontMatterGen library,
//! including:
//!
//! - Frontmatter extraction from content
//! - Conversion of frontmatter to various formats (YAML, JSON, TOML)
//! - Error handling for invalid frontmatter or formats
//! - SSG-specific examples for static site generators (if enabled)
//!
//! ## Usage
//!
//! To run this example:
//!
//! ```bash
//! cargo run --features default --example library_examples
//! ```

use frontmatter_gen::error::Error;
use frontmatter_gen::{extract, to_format, Format, Frontmatter, Value};

/// Main function demonstrating frontmatter extraction and conversion examples.
///
/// This function covers the core functionality and SSG-specific scenarios (if enabled).
///
/// # Errors
/// Returns an error if any of the examples fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Library Examples\n");

    // Core functionality examples
    extract_example()?;
    to_format_example()?;

    // Advanced scenarios and SSG-specific examples
    #[cfg(feature = "ssg")]
    ssg_examples()?;

    println!("\n🎉 All library examples completed successfully!");
    Ok(())
}

/// Demonstrates extracting frontmatter from content.
///
/// This example extracts YAML frontmatter from content and processes it.
///
/// # Errors
/// Returns an error if extraction fails.
fn extract_example() -> Result<(), Error> {
    println!("🦀 Frontmatter Extraction Example");
    println!("---------------------------------------------");

    let content = r#"---
title: My Post
date: 2025-09-09
tags:
  - rust
  - example
---
Content here"#;

    let (frontmatter, remaining_content) = extract(content)?;
    println!("    ✅ Extracted frontmatter: {:?}", frontmatter);
    println!("    Remaining content: {}", remaining_content);

    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );
    assert_eq!(remaining_content, "Content here");

    Ok(())
}

/// Demonstrates converting frontmatter to different formats.
///
/// This example converts frontmatter to YAML, JSON, and TOML formats.
///
/// # Errors
/// Returns an error if conversion fails.
fn to_format_example() -> Result<(), Error> {
    println!("\n🦀 Frontmatter Conversion Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    let _ = frontmatter.insert("title".to_string(), "My Post".into());
    let _ = frontmatter.insert("date".to_string(), "2025-09-09".into());

    // Use Value::Array for tags
    let tags = vec![Value::from("rust"), Value::from("example")];
    let _ = frontmatter.insert("tags".to_string(), Value::Array(tags));

    let yaml = to_format(&frontmatter, Format::Yaml)?;
    println!("    ✅ Converted frontmatter to YAML:\n{}", yaml);

    let json = to_format(&frontmatter, Format::Json)?;
    println!("    ✅ Converted frontmatter to JSON:\n{}", json);

    let toml = to_format(&frontmatter, Format::Toml)?;
    println!("    ✅ Converted frontmatter to TOML:\n{}", toml);

    assert!(yaml.contains("title: My Post"));
    assert!(yaml.contains("date: '2025-09-09'"));
    assert!(yaml.contains("tags:\n- rust\n- example"));

    // Debugging output for JSON string
    println!("    Debug: JSON output is:\n{}", json);
    assert!(json.contains("\"title\":\"My Post\"")); // Updated assertion for JSON format
    assert!(json.contains("\"date\":\"2025-09-09\""));
    assert!(json.contains("\"tags\":[\"rust\",\"example\"]"));

    assert!(toml.contains("title = \"My Post\""));
    assert!(toml.contains("date = \"2025-09-09\""));
    assert!(toml.contains("tags = [\"rust\", \"example\"]"));

    Ok(())
}

/// Demonstrates SSG-specific frontmatter processing (if enabled).
///
/// This example extracts SSG-specific frontmatter and converts it to multiple formats.
///
/// # Errors
/// Returns an error if extraction or conversion fails.
#[cfg(feature = "ssg")]
fn ssg_examples() -> Result<(), Error> {
    println!("\n🦀 SSG-Specific Frontmatter Examples");
    println!("---------------------------------------------");

    let content = r#"---
title: My Blog Post
date: 2025-09-09
template: blog
layout: post
tags:
  - rust
  - ssg
---
# Blog Content Here"#;

    let (frontmatter, remaining_content) = extract(content)?;
    println!("    ✅ Extracted SSG frontmatter: {:?}", frontmatter);
    println!("    Remaining content: {}", remaining_content);

    let yaml = to_format(&frontmatter, Format::Yaml)?;
    let toml = to_format(&frontmatter, Format::Toml)?;
    let json = to_format(&frontmatter, Format::Json)?;

    println!("\n    Converted Formats:");
    println!("    YAML:\n{}", yaml);
    println!("    TOML:\n{}", toml);
    println!("    JSON:\n{}", json);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic frontmatter extraction.
    #[test]
    fn test_basic_extraction() -> Result<(), Error> {
        let content = r#"---
title: Test
---
Content"#;
        let (frontmatter, content) = extract(content)?;
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test"
        );
        assert_eq!(content, "Content");
        Ok(())
    }

    /// Tests frontmatter conversion to different formats.
    #[test]
    fn test_format_conversion() -> Result<(), Error> {
        let mut frontmatter = Frontmatter::new();
        frontmatter.insert("title".to_string(), "Test".into());

        let yaml = to_format(&frontmatter, Format::Yaml)?;
        assert!(yaml.contains("title: Test"));

        let json = to_format(&frontmatter, Format::Json)?;
        assert!(json.contains("\"title\": \"Test\""));

        let toml = to_format(&frontmatter, Format::Toml)?;
        assert!(toml.contains("title = \"Test\""));

        Ok(())
    }

    /// Tests SSG-specific frontmatter processing.
    #[cfg(feature = "ssg")]
    #[test]
    fn test_ssg_metadata() -> Result<(), Error> {
        let content = r#"---
title: Test
template: post
layout: blog
tags:
  - rust
  - ssg
---
Content"#;
        let (frontmatter, _) = extract(content)?;
        assert!(frontmatter.get("template").is_some());
        assert!(frontmatter.get("layout").is_some());
        assert!(frontmatter.get("tags").is_some());
        Ok(())
    }
}
