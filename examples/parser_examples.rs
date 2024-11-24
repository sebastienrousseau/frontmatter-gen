// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Parser Examples
//!
//! This example demonstrates how to use the FrontMatterGen parser to parse and
//! serialize frontmatter in YAML, TOML, and JSON formats.
//!
//! ## Features
//!
//! - Parsing frontmatter in YAML, TOML, and JSON formats
//! - Serializing frontmatter to YAML, TOML, and JSON formats
//! - Roundtrip parsing and serialization validation
//! - Error handling for invalid inputs
//! - SSG-specific examples (enabled via the "ssg" feature)
//!
//! ## Usage
//!
//! To run this example:
//!
//! ```bash
//! cargo run --features default --example parser_examples
//! ```

use frontmatter_gen::{
    error::Error,
    parser::{parse, to_string},
    Format, Frontmatter, Value,
};

/// Main function demonstrating frontmatter parsing and serialization.
///
/// This function covers parsing and serializing YAML, TOML, and JSON frontmatter,
/// with additional examples for static site generators (SSG) if enabled.
///
/// # Errors
/// Returns an error if any of the examples fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Parser Examples\n");

    // Core functionality examples
    parse_yaml_example()?;
    parse_toml_example()?;
    parse_json_example()?;
    serialize_to_yaml_example()?;
    serialize_to_toml_example()?;
    serialize_to_json_example()?;

    // SSG-specific examples
    #[cfg(feature = "ssg")]
    ssg_parser_examples()?;

    println!("\n🎉 All parser examples completed successfully!");
    Ok(())
}

/// Demonstrates parsing YAML frontmatter.
///
/// This example shows how to parse YAML content into a Frontmatter struct.
///
/// # Errors
/// Returns an error if parsing fails.
fn parse_yaml_example() -> Result<(), Error> {
    println!("🦀 YAML Parsing Example");
    println!("---------------------------------------------");

    let yaml_content = "title: My Post\ndate: 2025-09-09\n";
    let frontmatter = parse(yaml_content, Format::Yaml)?;

    println!("    ✅ Parsed frontmatter: {:?}", frontmatter);
    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );

    Ok(())
}

/// Demonstrates parsing TOML frontmatter.
///
/// This example shows how to parse TOML content into a Frontmatter struct.
///
/// # Errors
/// Returns an error if parsing fails.
fn parse_toml_example() -> Result<(), Error> {
    println!("\n🦀 TOML Parsing Example");
    println!("---------------------------------------------");

    let toml_content = "title = \"My Post\"\ndate = 2025-09-09\n";
    let frontmatter = parse(toml_content, Format::Toml)?;

    println!("    ✅ Parsed frontmatter: {:?}", frontmatter);
    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );

    Ok(())
}

/// Demonstrates parsing JSON frontmatter.
///
/// This example shows how to parse JSON content into a Frontmatter struct.
///
/// # Errors
/// Returns an error if parsing fails.
fn parse_json_example() -> Result<(), Error> {
    println!("\n🦀 JSON Parsing Example");
    println!("---------------------------------------------");

    let json_content = r#"{"title": "My Post", "date": "2025-09-09"}"#;
    let frontmatter = parse(json_content, Format::Json)?;

    println!("    ✅ Parsed frontmatter: {:?}", frontmatter);
    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );

    Ok(())
}

/// Helper function to create sample frontmatter.
///
/// This function creates a Frontmatter struct with predefined data.
///
/// # Returns
/// Returns a sample `Frontmatter` instance.
fn create_sample_frontmatter() -> Frontmatter {
    let mut frontmatter = Frontmatter::new();
    let _ = frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    let _ = frontmatter.insert(
        "date".to_string(),
        Value::String("2025-09-09".to_string()),
    );
    frontmatter
}

/// Demonstrates serializing frontmatter to YAML.
///
/// This example shows how to serialize a Frontmatter struct to YAML format.
///
/// # Errors
/// Returns an error if serialization fails.
fn serialize_to_yaml_example() -> Result<(), Error> {
    println!("\n🦀 YAML Serialization Example");
    println!("---------------------------------------------");

    let frontmatter = create_sample_frontmatter();
    let yaml = to_string(&frontmatter, Format::Yaml)?;

    println!("    ✅ Serialized to YAML:\n{}", yaml);
    assert!(yaml.contains("title: My Post"));

    Ok(())
}

/// Demonstrates serializing frontmatter to TOML.
///
/// This example shows how to serialize a Frontmatter struct to TOML format.
///
/// # Errors
/// Returns an error if serialization fails.
fn serialize_to_toml_example() -> Result<(), Error> {
    println!("\n🦀 TOML Serialization Example");
    println!("---------------------------------------------");

    let frontmatter = create_sample_frontmatter();
    let toml = to_string(&frontmatter, Format::Toml)?;

    println!("    ✅ Serialized to TOML:\n{}", toml);
    assert!(toml.contains("title = \"My Post\""));

    Ok(())
}

/// Demonstrates serializing frontmatter to JSON.
///
/// This example shows how to serialize a Frontmatter struct to JSON format.
///
/// # Errors
/// Returns an error if serialization fails.
fn serialize_to_json_example() -> Result<(), Error> {
    println!("\n🦀 JSON Serialization Example");
    println!("---------------------------------------------");

    let frontmatter = create_sample_frontmatter();
    let json = to_string(&frontmatter, Format::Json)?;

    println!("    ✅ Serialized to JSON:\n{}", json);
    assert!(json.contains("\"title\": \"My Post\""));

    Ok(())
}

/// Demonstrates SSG-specific frontmatter parsing and serialization.
///
/// This example shows how to process frontmatter for SSGs, including additional
/// metadata fields.
///
/// # Errors
/// Returns an error if parsing or serialization fails.
#[cfg(feature = "ssg")]
fn ssg_parser_examples() -> Result<(), Error> {
    println!("\n🦀 SSG-Specific Parser Examples");
    println!("---------------------------------------------");

    // Create a complex frontmatter with SSG-specific fields
    let mut frontmatter = Frontmatter::new();
    let _ = frontmatter.insert(
        "title".to_string(),
        Value::String("My Blog Post".to_string()),
    );
    let _ = frontmatter.insert(
        "template".to_string(),
        Value::String("post".to_string()),
    );
    let _ = frontmatter.insert(
        "layout".to_string(),
        Value::String("blog".to_string()),
    );
    let _ =
        frontmatter.insert("draft".to_string(), Value::Boolean(false));
    let _ = frontmatter.insert(
        "tags".to_string(),
        Value::Array(vec![
            Value::String("rust".to_string()),
            Value::String("ssg".to_string()),
        ]),
    );

    // Demonstrate parsing and serializing in all formats
    println!("\n    Converting SSG frontmatter to all formats:");
    for format in [Format::Yaml, Format::Toml, Format::Json] {
        let serialized = to_string(&frontmatter, format)?;
        println!("\n    {} format:", format);
        println!("{}", serialized);

        // Verify roundtrip
        let parsed = parse(&serialized, format)?;
        assert_eq!(
            parsed.get("template").unwrap().as_str().unwrap(),
            "post"
        );
        assert_eq!(
            parsed.get("layout").unwrap().as_str().unwrap(),
            "blog"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic YAML parsing.
    #[test]
    fn test_basic_parsing() -> Result<(), Error> {
        let yaml = "title: Test\n";
        let frontmatter = parse(yaml, Format::Yaml)?;
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test"
        );
        Ok(())
    }

    /// Tests YAML serialization.
    #[test]
    fn test_serialization() -> Result<(), Error> {
        let frontmatter = create_sample_frontmatter();
        let yaml = to_string(&frontmatter, Format::Yaml)?;
        assert!(yaml.contains("title: My Post"));
        Ok(())
    }

    /// Tests roundtrip parsing and serialization.
    #[test]
    fn test_roundtrip() -> Result<(), Error> {
        let frontmatter = create_sample_frontmatter();
        let yaml = to_string(&frontmatter, Format::Yaml)?;
        let parsed = parse(&yaml, Format::Yaml)?;
        assert_eq!(
            parsed.get("title").unwrap().as_str().unwrap(),
            "My Post"
        );
        Ok(())
    }
}
