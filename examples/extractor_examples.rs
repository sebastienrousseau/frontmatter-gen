// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Extractor Examples
//!
//! This example demonstrates the functionality for extracting frontmatter in
//! YAML, TOML, and JSON formats from content. It covers various scenarios for
//! frontmatter extraction, format detection, and error handling.

#![allow(missing_docs)]

use frontmatter_gen::error::FrontmatterError;
use frontmatter_gen::extractor::{
    detect_format, extract_json_frontmatter, extract_raw_frontmatter,
};

/// Entry point for the FrontMatterGen extractor examples.
///
/// This function runs various examples demonstrating frontmatter extraction and
/// format detection for different scenarios in the FrontMatterGen library.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Extractor Examples\n");

    // Core functionality examples
    extract_yaml_example()?;
    extract_toml_example()?;
    extract_json_example()?;
    extract_json_deeply_nested_example()?;
    detect_format_example()?;

    // SSG-specific examples
    #[cfg(feature = "ssg")]
    run_ssg_examples()?;

    println!("\n🎉  All extractor examples completed successfully!");
    Ok(())
}

/// Demonstrates extracting YAML frontmatter from content.
fn extract_yaml_example() -> Result<(), FrontmatterError> {
    println!("🦀 YAML Frontmatter Extraction Example");
    println!("---------------------------------------------");
    let content = r#"---
title: Example
---
Content here"#;
    let result = extract_raw_frontmatter(content)?;
    println!("    ✅  Extracted frontmatter: {}\n", result.0);
    println!("    Remaining content: {}", result.1);
    Ok(())
}

/// Demonstrates extracting TOML frontmatter from content.
fn extract_toml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 TOML Frontmatter Extraction Example");
    println!("---------------------------------------------");
    let content = r#"+++
title = "Example"
+++
Content here"#;
    let result = extract_raw_frontmatter(content)?;
    println!("    ✅  Extracted frontmatter: {}\n", result.0);
    println!("    Remaining content: {}", result.1);
    Ok(())
}

/// Demonstrates extracting JSON frontmatter from content.
fn extract_json_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 JSON Frontmatter Extraction Example");
    println!("---------------------------------------------");
    let content = r#"{ "title": "Example" }
Content here"#;
    let result = extract_json_frontmatter(content)?;
    println!("    ✅  Extracted JSON frontmatter: {}\n", result);
    Ok(())
}

/// Demonstrates extracting deeply nested JSON frontmatter from content.
fn extract_json_deeply_nested_example() -> Result<(), FrontmatterError>
{
    println!("\n🦀 Deeply Nested JSON Frontmatter Example");
    println!("---------------------------------------------");
    let content = r#"{ "a": { "b": { "c": { "d": { "e": {} }}}}}
Content here"#;
    let result = extract_json_frontmatter(content)?;
    println!(
        "    ✅  Extracted deeply nested frontmatter: {}\n",
        result
    );
    Ok(())
}

/// Demonstrates detecting the format of frontmatter.
fn detect_format_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 Frontmatter Format Detection Example");
    println!("---------------------------------------------");
    let yaml = "title: Example";
    let toml = "title = \"Example\"";
    let json = "{ \"title\": \"Example\" }";
    println!(
        "    Detected format for YAML: {:?}",
        detect_format(yaml)?
    );
    println!(
        "    Detected format for TOML: {:?}",
        detect_format(toml)?
    );
    println!(
        "    Detected format for JSON: {:?}",
        detect_format(json)?
    );
    Ok(())
}

/// SSG-specific examples that are only available with the "ssg" feature
#[cfg(feature = "ssg")]
fn run_ssg_examples() -> Result<(), FrontmatterError> {
    println!("\n🦀 SSG-Specific Examples");
    println!("---------------------------------------------");

    // Example of extracting frontmatter with SSG-specific metadata
    let content = r#"---
title: My Page
layout: post
template: blog
date: 2025-01-01
---
Content here"#;

    let result = extract_raw_frontmatter(content)?;
    println!("    ✅  Extracted SSG frontmatter: {}\n", result.0);
    println!("    Remaining content: {}", result.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Core functionality tests
    #[test]
    fn test_yaml_extraction() -> Result<(), FrontmatterError> {
        let content = r#"---
title: Test
---
Content"#;
        let result = extract_raw_frontmatter(content)?;
        assert_eq!(result.0, "title: Test");
        Ok(())
    }

    #[test]
    fn test_toml_extraction() -> Result<(), FrontmatterError> {
        let content = r#"+++
title = "Test"
+++
Content"#;
        let result = extract_raw_frontmatter(content)?;
        assert_eq!(result.0, "title = \"Test\"");
        Ok(())
    }

    // SSG-specific tests
    #[cfg(feature = "ssg")]
    mod ssg_tests {
        use super::*;

        #[test]
        fn test_ssg_frontmatter() -> Result<(), FrontmatterError> {
            let content = r#"---
title: Test
template: post
---
Content"#;
            let result = extract_raw_frontmatter(content)?;
            assert!(result.0.contains("template: post"));
            Ok(())
        }
    }
}
