// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Parser Examples
//!
//! This example demonstrates how to use the FrontMatterGen parser to parse and
//! serialize frontmatter in YAML, TOML, and JSON formats.

#![allow(missing_docs)]

use frontmatter_gen::error::FrontmatterError;
use frontmatter_gen::{
    parser::parse, parser::to_string, Format, Frontmatter, Value,
};

/// Entry point for the FrontMatterGen parser examples.
///
/// This function runs various examples demonstrating frontmatter parsing and
/// serialization for different formats (YAML, TOML, and JSON).
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
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

    println!("\n🎉  All parser examples completed successfully!");

    Ok(())
}

/// Demonstrates parsing YAML frontmatter.
fn parse_yaml_example() -> Result<(), FrontmatterError> {
    println!("🦀 YAML Parsing Example");
    println!("---------------------------------------------");

    let yaml_content = "title: My Post\ndate: 2025-09-09\n";
    let frontmatter = parse(yaml_content, Format::Yaml)?;

    println!("    ✅  Parsed frontmatter: {:?}", frontmatter);
    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );

    Ok(())
}

/// Demonstrates parsing TOML frontmatter.
fn parse_toml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 TOML Parsing Example");
    println!("---------------------------------------------");

    let toml_content = "title = \"My Post\"\ndate = 2025-09-09\n";
    let frontmatter = parse(toml_content, Format::Toml)?;

    println!("    ✅  Parsed frontmatter: {:?}", frontmatter);
    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );

    Ok(())
}

/// Demonstrates parsing JSON frontmatter.
fn parse_json_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 JSON Parsing Example");
    println!("---------------------------------------------");

    let json_content = r#"{"title": "My Post", "date": "2025-09-09"}"#;
    let frontmatter = parse(json_content, Format::Json)?;

    println!("    ✅  Parsed frontmatter: {:?}", frontmatter);
    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );

    Ok(())
}

/// Creates a sample frontmatter for examples
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
fn serialize_to_yaml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 YAML Serialization Example");
    println!("---------------------------------------------");

    let frontmatter = create_sample_frontmatter();
    let yaml = to_string(&frontmatter, Format::Yaml)?;

    println!("    ✅  Serialized to YAML:\n{}", yaml);
    assert!(yaml.contains("title: My Post"));

    Ok(())
}

/// Demonstrates serializing frontmatter to TOML.
fn serialize_to_toml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 TOML Serialization Example");
    println!("---------------------------------------------");

    let frontmatter = create_sample_frontmatter();
    let toml = to_string(&frontmatter, Format::Toml)?;

    println!("    ✅  Serialized to TOML:\n{}", toml);
    assert!(toml.contains("title = \"My Post\""));

    Ok(())
}

/// Demonstrates serializing frontmatter to JSON.
fn serialize_to_json_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 JSON Serialization Example");
    println!("---------------------------------------------");

    let frontmatter = create_sample_frontmatter();
    let json = to_string(&frontmatter, Format::Json)?;

    println!("    ✅  Serialized to JSON:\n{}", json);
    assert!(json.contains("\"title\": \"My Post\""));

    Ok(())
}

/// SSG-specific examples that are only available with the "ssg" feature
#[cfg(feature = "ssg")]
fn ssg_parser_examples() -> Result<(), FrontmatterError> {
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

    // Core functionality tests
    #[test]
    fn test_basic_parsing() -> Result<(), FrontmatterError> {
        let yaml = "title: Test\n";
        let frontmatter = parse(yaml, Format::Yaml)?;
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test"
        );
        Ok(())
    }

    #[test]
    fn test_serialization() -> Result<(), FrontmatterError> {
        let frontmatter = create_sample_frontmatter();
        let yaml = to_string(&frontmatter, Format::Yaml)?;
        assert!(yaml.contains("title: My Post"));
        Ok(())
    }

    // SSG-specific tests
    #[cfg(feature = "ssg")]
    mod ssg_tests {
        use super::*;

        #[test]
        fn test_ssg_complex_frontmatter() -> Result<(), FrontmatterError>
        {
            let yaml = r#"
template: post
layout: blog
tags:
  - rust
  - ssg
draft: false
"#;
            let frontmatter = parse(yaml, Format::Yaml)?;
            assert_eq!(
                frontmatter.get("template").unwrap().as_str().unwrap(),
                "post"
            );
            assert_eq!(
                frontmatter.get("layout").unwrap().as_str().unwrap(),
                "blog"
            );
            assert!(!frontmatter
                .get("draft")
                .unwrap()
                .as_bool()
                .unwrap());
            Ok(())
        }
    }
}
