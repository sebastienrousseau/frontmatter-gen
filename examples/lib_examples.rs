// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Library Examples
//!
//! This example demonstrates the core functionality of the FrontMatterGen library,
//! including frontmatter extraction and conversion to various formats.

#![allow(missing_docs)]

use frontmatter_gen::error::FrontmatterError;
use frontmatter_gen::{extract, to_format, Format, Frontmatter};

/// Entry point for the FrontMatterGen library examples.
///
/// This function runs various examples demonstrating frontmatter extraction and
/// conversion for different scenarios in the FrontMatterGen library.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Library Examples\n");

    extract_example()?;
    to_format_example()?;

    println!("\n🎉  All library examples completed successfully!");

    Ok(())
}

/// Demonstrates extracting frontmatter from content.
fn extract_example() -> Result<(), FrontmatterError> {
    println!("🦀 Frontmatter Extraction Example");
    println!("---------------------------------------------");

    let yaml_content = r#"---
title: My Post
date: 2024-11-16
---
Content here"#;

    let (frontmatter, remaining_content) = extract(yaml_content)?;
    println!("    ✅  Extracted frontmatter: {:?}", frontmatter);
    println!("    Remaining content: {}", remaining_content);

    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );
    assert_eq!(remaining_content, "Content here");

    Ok(())
}

/// Demonstrates converting frontmatter to a specific format.
fn to_format_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 Frontmatter Conversion Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    frontmatter.insert("title".to_string(), "My Post".into());
    frontmatter.insert("date".to_string(), "2024-11-16".into());

    let yaml = to_format(&frontmatter, Format::Yaml)?;
    println!("    ✅  Converted frontmatter to YAML:\n{}", yaml);

    let json = to_format(&frontmatter, Format::Json)?;
    println!("    ✅  Converted frontmatter to JSON:\n{}", json);

    assert!(yaml.contains("title: My Post"));
    assert!(yaml.contains("date: '2024-11-16'"));
    assert!(json.contains("\"title\": \"My Post\""));
    assert!(json.contains("\"date\": \"2024-11-16\""));

    Ok(())
}
