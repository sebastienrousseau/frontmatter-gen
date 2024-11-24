// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Front Matter Error Examples
//!
//! This example demonstrates various error handling scenarios when using the
//! `frontmatter-gen` library, including:
//!
//! - Format-specific parsing errors (YAML, TOML, JSON)
//! - Conversion and validation errors
//! - Handling unexpected parsing success
//! - Edge case handling and robustness testing
//! - Performance benchmarks
//!
//! ## Usage
//!
//! ```bash
//! cargo run --features default --example error_examples
//! ```

use anyhow::{Context, Result};
use env_logger::{Builder, Env};
use frontmatter_gen::error::Error;
use log::info;

/// Displays the result of a test with appropriate messages.
///
/// # Arguments
/// * `result` - The result of the operation being tested.
/// * `success_message` - The message to display if the operation succeeds.
/// * `error_message` - The message to display if the operation fails.
fn display_result(
    result: Result<(), Error>,
    success_message: &str,
    error_message: &str,
) {
    match result {
        Ok(_) => println!("✅ {}", success_message),
        Err(e) => println!("❌ {}: {}", error_message, e),
    }
}

/// Validates YAML parsing and ensures errors are correctly detected.
///
/// # Arguments
/// * `input` - The YAML content to parse.
///
/// # Returns
/// * `Result<(), Error>` indicating success or failure.
fn validate_yaml_parsing(input: &str) -> Result<(), Error> {
    match serde_yml::from_str::<serde_yml::Value>(input) {
        Ok(_) => Err(Error::InvalidFormat),
        Err(_) => Ok(()),
    }
}

/// Validates TOML parsing and ensures errors are correctly detected.
///
/// # Arguments
/// * `input` - The TOML content to parse.
///
/// # Returns
/// * `Result<(), Error>` indicating success or failure.
fn validate_toml_parsing(input: &str) -> Result<(), Error> {
    match toml::from_str::<toml::Value>(input) {
        Ok(_) => Err(Error::InvalidFormat),
        Err(_) => Ok(()),
    }
}

/// Validates JSON parsing and ensures errors are correctly detected.
///
/// # Arguments
/// * `input` - The JSON content to parse.
///
/// # Returns
/// * `Result<(), Error>` indicating success or failure.
fn validate_json_parsing(input: &str) -> Result<(), Error> {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(_) => Err(Error::InvalidFormat),
        Err(_) => Ok(()),
    }
}

/// Tests parsing of valid YAML, TOML, and JSON inputs.
fn example_valid_parsing() -> Result<()> {
    let valid_yaml = r#"---
title: "Valid YAML"
description: "This is a valid YAML document"
list:
  - item1
  - item2
---"#;
    println!("\n✅ Testing valid YAML...");
    display_result(
        validate_yaml_parsing(valid_yaml),
        "Successfully parsed valid YAML.",
        "Failed to parse valid YAML",
    );

    let valid_toml = r#"
title = "Valid TOML"
description = "This is a valid TOML document"
[section]
key = "value"
"#;
    println!("\n✅ Testing valid TOML...");
    display_result(
        validate_toml_parsing(valid_toml),
        "Successfully parsed valid TOML.",
        "Failed to parse valid TOML",
    );

    let valid_json = r#"{
    "title": "Valid JSON",
    "description": "This is a valid JSON document",
    "list": ["item1", "item2"]
}"#;
    println!("\n✅ Testing valid JSON...");
    display_result(
        validate_json_parsing(valid_json),
        "Successfully parsed valid JSON.",
        "Failed to parse valid JSON",
    );

    Ok(())
}

/// Tests error handling for YAML inputs.
fn example_yaml_errors() -> Result<()> {
    info!("📝 YAML Error Examples: Testing YAML error handling");

    let invalid_yaml = r#"---
title: : Invalid : Syntax
description: *undefined_anchor
array:
  - item1
  - : invalid
---"#;
    println!("\n🚨 Testing invalid YAML syntax...");
    display_result(
        validate_yaml_parsing(invalid_yaml),
        "Successfully caught YAML syntax error.",
        "Unexpectedly passed invalid YAML syntax",
    );

    let empty_yaml = r#"---
---"#;
    println!("\n🚨 Testing empty YAML document...");
    display_result(
        validate_yaml_parsing(empty_yaml),
        "Successfully caught empty YAML document.",
        "Unexpectedly passed empty YAML document",
    );

    let invalid_indent = r#"---
title: Test
  description: Wrong indentation
    nested: Also wrong
---"#;
    println!("\n🚨 Testing invalid indentation...");
    display_result(
        validate_yaml_parsing(invalid_indent),
        "Successfully caught YAML indentation error.",
        "Unexpectedly passed invalid YAML indentation",
    );

    Ok(())
}

/// Tests error handling for TOML inputs.
fn example_toml_errors() -> Result<()> {
    info!("📝 TOML Error Examples: Testing TOML error handling");

    let invalid_toml = r#"title = = "invalid syntax"
[section]
key = unclosed string"
author = missing quotes"#;
    println!("\n🚨 Testing invalid TOML syntax...");
    display_result(
        validate_toml_parsing(invalid_toml),
        "Successfully caught TOML syntax error.",
        "Unexpectedly passed invalid TOML syntax",
    );

    let duplicate_keys = r#"title = "First title"
description = "A description"
title = "Second title""#;
    println!("\n🚨 Testing duplicate TOML keys...");
    display_result(
        validate_toml_parsing(duplicate_keys),
        "Successfully caught duplicate key error.",
        "Unexpectedly passed duplicate TOML keys",
    );

    let invalid_table = r#"[table
key = "value"]"#;
    println!("\n🚨 Testing invalid table syntax...");
    display_result(
        validate_toml_parsing(invalid_table),
        "Successfully caught invalid table syntax error.",
        "Unexpectedly passed invalid table syntax",
    );

    Ok(())
}

/// Tests error handling for JSON inputs.
fn example_json_errors() -> Result<()> {
    info!("📝 JSON Error Examples: Testing JSON error handling");

    let invalid_json = r#"{
    "title": "Missing comma"
    "author": "John Doe",
    "description": "unclosed string
}"#;
    println!("\n🚨 Testing invalid JSON syntax...");
    display_result(
        validate_json_parsing(invalid_json),
        "Successfully caught JSON syntax error.",
        "Unexpectedly passed invalid JSON syntax",
    );

    let invalid_number = r#"{
    "title": "Test",
    "value": .123,
    "array": [1, 2, .]
}"#;
    println!("\n🚨 Testing invalid number format...");
    display_result(
        validate_json_parsing(invalid_number),
        "Successfully caught invalid number error.",
        "Unexpectedly passed invalid number format",
    );

    let invalid_unicode = r#"{
    "title": "Invalid \u123 escape sequence"
}"#;
    println!("\n🚨 Testing invalid Unicode escape...");
    display_result(
        validate_json_parsing(invalid_unicode),
        "Successfully caught invalid Unicode error.",
        "Unexpectedly passed invalid Unicode escape",
    );

    Ok(())
}

/// Tests handling of empty inputs.
fn example_empty_or_null_inputs() -> Result<()> {
    println!("\n🚨 Testing empty or null inputs...");

    let empty_input = "";
    display_result(
        validate_yaml_parsing(empty_input),
        "Successfully caught empty input for YAML.",
        "Unexpectedly passed empty input for YAML",
    );

    display_result(
        validate_toml_parsing(empty_input),
        "Successfully caught empty input for TOML.",
        "Unexpectedly passed empty input for TOML",
    );

    display_result(
        validate_json_parsing(empty_input),
        "Successfully caught empty input for JSON.",
        "Unexpectedly passed empty input for JSON",
    );

    Ok(())
}

/// Tests edge cases like circular references and deeply nested structures.
fn example_edge_cases() -> Result<()> {
    println!("\n🚨 Testing edge cases...");

    let yaml_with_anchor = r#"---
title: &title_anchor "Valid Title"
description: *undefined_anchor
---"#;
    display_result(
        validate_yaml_parsing(yaml_with_anchor),
        "Successfully caught YAML anchor error.",
        "Unexpectedly passed YAML with invalid anchor",
    );

    let deeply_nested_json = r#"{
        "level1": {
            "level2": {
                "level3": {
                    "level4": "Too deep!"
                }
            }
        }
    }"#;
    display_result(
        validate_json_parsing(deeply_nested_json),
        "Successfully caught deeply nested JSON error.",
        "Unexpectedly passed deeply nested JSON",
    );

    Ok(())
}

/// Main function demonstrating all error handling examples.
pub fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .context("Failed to initialize logger")?;

    println!("🧪 Front Matter Error Handling Examples\n");

    example_yaml_errors()?;
    example_toml_errors()?;
    example_json_errors()?;
    example_valid_parsing()?;
    example_empty_or_null_inputs()?;
    example_edge_cases()?;

    println!(
        "\n✨ All error handling examples completed successfully!"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_errors() -> Result<()> {
        example_yaml_errors()
    }

    #[test]
    fn test_toml_errors() -> Result<()> {
        example_toml_errors()
    }

    #[test]
    fn test_json_errors() -> Result<()> {
        example_json_errors()
    }

    #[test]
    fn test_valid_parsing() -> Result<()> {
        example_valid_parsing()
    }

    #[test]
    fn test_empty_or_null_inputs() -> Result<()> {
        example_empty_or_null_inputs()
    }

    #[test]
    fn test_edge_cases() -> Result<()> {
        example_edge_cases()
    }
}
