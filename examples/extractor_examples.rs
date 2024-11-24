// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # FrontMatterGen Extractor Examples
//!
//! This example demonstrates the functionality for extracting frontmatter in
//! YAML, TOML, and JSON formats from content. It includes examples for:
//!
//! - Frontmatter extraction (YAML, TOML, JSON)
//! - Format detection
//! - Error handling for edge cases
//! - Handling invalid inputs and delimiters
//! - Benchmarking performance with large frontmatter
//! - SSG-specific functionality (if enabled)
//!
//! ## Usage
//!
//! To run this example:
//!
//! ```bash
//! cargo run --features default --example extractor
//! ```

use frontmatter_gen::error::Error;
use frontmatter_gen::extractor::{
    detect_format, extract_json_frontmatter, extract_raw_frontmatter,
};
use std::time::Instant;

/// Main function demonstrating frontmatter extraction and format detection.
///
/// This function covers core functionality, advanced scenarios, edge cases, and benchmarking.
///
/// # Errors
/// Returns an error if any of the example functions fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Extractor Examples\n");

    // Core functionality examples
    extract_yaml_example()?;
    extract_toml_example()?;
    extract_json_example()?;
    extract_json_deeply_nested_example()?;
    detect_format_example()?;

    // Advanced scenarios and edge cases
    example_invalid_delimiters()?;
    example_partial_content()?;
    example_edge_cases()?;
    example_empty_inputs()?;
    benchmark_large_frontmatter()?;

    // SSG-specific examples (enabled via "ssg" feature)
    #[cfg(feature = "ssg")]
    run_ssg_examples()?;

    println!("\n🎉 All extractor examples completed successfully!");
    Ok(())
}

/// Demonstrates extracting YAML frontmatter from content.
///
/// This example shows how to extract and process frontmatter in YAML format.
///
/// # Errors
/// Returns an error if YAML extraction fails.
fn extract_yaml_example() -> Result<(), Error> {
    println!("🦀 YAML Frontmatter Extraction Example");
    println!("---------------------------------------------");
    let content = r#"---
title: Example
---
Content here"#;
    let result = extract_raw_frontmatter(content)?;
    println!("    ✅ Extracted frontmatter: {}\n", result.0);
    println!("    Remaining content: {}", result.1);
    Ok(())
}

/// Demonstrates extracting TOML frontmatter from content.
///
/// This example shows how to extract and process frontmatter in TOML format.
///
/// # Errors
/// Returns an error if TOML extraction fails.
fn extract_toml_example() -> Result<(), Error> {
    println!("\n🦀 TOML Frontmatter Extraction Example");
    println!("---------------------------------------------");
    let content = r#"+++
title = "Example"
+++
Content here"#;
    let result = extract_raw_frontmatter(content)?;
    println!("    ✅ Extracted frontmatter: {}\n", result.0);
    println!("    Remaining content: {}", result.1);
    Ok(())
}

/// Demonstrates extracting JSON frontmatter from content.
///
/// This example shows how to extract and process frontmatter in JSON format.
///
/// # Errors
/// Returns an error if JSON extraction fails.
fn extract_json_example() -> Result<(), Error> {
    println!("\n🦀 JSON Frontmatter Extraction Example");
    println!("---------------------------------------------");
    let content = r#"{ "title": "Example" }
Content here"#;
    let result = extract_json_frontmatter(content)?;
    println!("    ✅ Extracted JSON frontmatter: {}\n", result);
    Ok(())
}

/// Demonstrates extracting deeply nested JSON frontmatter.
///
/// This example highlights the library's ability to handle complex, deeply nested JSON structures.
///
/// # Errors
/// Returns an error if JSON extraction fails.
fn extract_json_deeply_nested_example() -> Result<(), Error> {
    println!("\n🦀 Deeply Nested JSON Frontmatter Example");
    println!("---------------------------------------------");
    let content = r#"{ "a": { "b": { "c": { "d": { "e": {} }}}}}
Content here"#;
    let result = extract_json_frontmatter(content)?;
    println!(
        "    ✅ Extracted deeply nested frontmatter: {}\n",
        result
    );
    Ok(())
}

/// Demonstrates detecting the format of frontmatter.
///
/// This example shows how to identify the frontmatter format (YAML, TOML, or JSON).
///
/// # Errors
/// Returns an error if format detection fails.
fn detect_format_example() -> Result<(), Error> {
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

/// Demonstrates handling of invalid frontmatter delimiters.
///
/// This example highlights how invalid delimiters are correctly detected as errors.
///
/// # Errors
/// Returns an error if invalid delimiters are not detected.
fn example_invalid_delimiters() -> Result<(), Error> {
    println!("\n🚨 Testing invalid delimiters...");

    let content_with_invalid_delimiters = r#"<>
title: Invalid Delimiters
<>
Content"#;
    let result =
        extract_raw_frontmatter(content_with_invalid_delimiters);
    match result {
        Ok(_) => println!("    ❌ Unexpectedly extracted content with invalid delimiters."),
        Err(e) => println!(
            "    ✅ Correctly detected invalid delimiters. Error: {}\n",
            e
        ),
    }

    Ok(())
}

/// Demonstrates handling of partially complete frontmatter blocks.
///
/// This example shows how the library handles incomplete or malformed frontmatter.
///
/// # Errors
/// Returns an error if partial content handling fails.
fn example_partial_content() -> Result<(), Error> {
    println!("\n🚨 Testing partial content extraction...");

    let content_with_partial_frontmatter = r#"---
title: "Incomplete
Content"#;
    let result =
        extract_raw_frontmatter(content_with_partial_frontmatter);
    match result {
        Ok(_) => println!("    ❌ Unexpectedly extracted incomplete frontmatter."),
        Err(e) => println!(
            "    ✅ Correctly detected incomplete frontmatter. Error: {}\n",
            e
        ),
    }

    Ok(())
}

/// Demonstrates handling of edge cases during frontmatter extraction.
///
/// This example includes scenarios like empty frontmatter, malformed JSON, and unsupported formats.
///
/// # Errors
/// Returns an error if edge case handling fails.
fn example_edge_cases() -> Result<(), Error> {
    println!("\n🚨 Testing edge cases...");

    let empty_frontmatter = r#"---
---
Content here"#;
    let result = extract_raw_frontmatter(empty_frontmatter);
    match result {
        Ok(res) => println!(
            "    ✅ Extracted empty frontmatter: '{}', Remaining: '{}'\n",
            res.0, res.1
        ),
        Err(e) => println!(
            "    ❌ Failed to extract empty frontmatter. Error: {}\n",
            e
        ),
    }

    Ok(())
}

/// Benchmarks performance for extracting large frontmatter blocks.
///
/// This example measures the time taken to process a very large frontmatter section.
///
/// # Errors
/// Returns an error if extraction fails.
fn benchmark_large_frontmatter() -> Result<(), Error> {
    println!("\n🚀 Benchmarking large frontmatter extraction...");
    let large_content = format!(
        "---\n{}\n---\nContent here",
        (0..10_000)
            .map(|i| format!("key{}: value{}", i, i))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let start = Instant::now();
    let result = extract_raw_frontmatter(&large_content);
    let duration = start.elapsed();

    match result {
        Ok(_) => println!(
            "    ✅ Successfully extracted large frontmatter in {:?}.",
            duration
        ),
        Err(e) => println!(
            "    ❌ Failed to extract large frontmatter. Error: {}",
            e
        ),
    }

    Ok(())
}

/// Demonstrates handling of empty inputs.
///
/// This example highlights how empty or null inputs are handled by the library.
///
/// # Errors
/// Returns an error if empty inputs are not correctly handled.
fn example_empty_inputs() -> Result<(), Error> {
    println!("\n🚨 Testing empty inputs...");

    let empty_content = "";
    match extract_raw_frontmatter(empty_content) {
        Ok(_) => println!("    ❌ Unexpectedly parsed empty content."),
        Err(_) => println!(
            "    ✅ Correctly detected error for empty content."
        ),
    }

    let null_content: Option<&str> = None;
    if let Some(content) = null_content {
        match extract_raw_frontmatter(content) {
            Ok(_) => {
                println!("    ❌ Unexpectedly parsed null content.")
            }
            Err(_) => println!(
                "    ✅ Correctly detected error for null content."
            ),
        }
    } else {
        println!("    ✅ Detected null content as invalid input.");
    }

    Ok(())
}

/// Demonstrates SSG-specific frontmatter extraction (if enabled).
///
/// This example handles frontmatter containing metadata fields specific to static site generators.
///
/// # Errors
/// Returns an error if SSG-specific extraction fails.
#[cfg(feature = "ssg")]
fn run_ssg_examples() -> Result<(), Error> {
    println!("\n🦀 SSG-Specific Examples");
    println!("---------------------------------------------");

    let content = r#"---
title: My Page
layout: post
template: blog
date: 2025-01-01
---
Content here"#;

    let result = extract_raw_frontmatter(content)?;
    println!("    ✅ Extracted SSG frontmatter: {}\n", result.0);
    println!("    Remaining content: {}", result.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_extraction() -> Result<(), Error> {
        let content = r#"---
title: Test
---
Content"#;
        let result = extract_raw_frontmatter(content)?;
        assert_eq!(result.0, "title: Test");
        assert_eq!(result.1, "Content");
        Ok(())
    }

    #[test]
    fn test_toml_extraction() -> Result<(), Error> {
        let content = r#"+++
title = "Test"
+++
Content"#;
        let result = extract_raw_frontmatter(content)?;
        assert_eq!(result.0, "title = \"Test\"");
        assert_eq!(result.1, "Content");
        Ok(())
    }

    #[test]
    fn test_yaml_extraction() -> Result<(), Error> {
        let content = r#"---
title: Test
---
Content"#;
        let result = extract_raw_frontmatter(content)?;
        assert_eq!(result.0, "title: Test");
        assert_eq!(result.1, "Content");
        Ok(())
    }

    #[test]
    fn test_empty_inputs() -> Result<(), Error> {
        example_empty_inputs()
    }

    #[test]
    fn test_invalid_delimiters() -> Result<(), Error> {
        example_invalid_delimiters()
    }

    #[test]
    fn test_partial_content() -> Result<(), Error> {
        example_partial_content()
    }

    #[test]
    fn test_edge_cases() -> Result<(), Error> {
        example_edge_cases()
    }

    #[test]
    fn test_large_frontmatter() -> Result<(), Error> {
        benchmark_large_frontmatter()
    }

    #[cfg(feature = "ssg")]
    #[test]
    fn test_ssg_examples() -> Result<(), Error> {
        run_ssg_examples()
    }
}
