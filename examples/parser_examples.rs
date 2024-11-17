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
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Parser Examples\n");

    parse_yaml_example()?;
    parse_toml_example()?;
    parse_json_example()?;
    serialize_to_yaml_example()?;
    serialize_to_toml_example()?;
    serialize_to_json_example()?;

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

    Ok(())
}

/// Demonstrates parsing TOML frontmatter.
fn parse_toml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 TOML Parsing Example");
    println!("---------------------------------------------");

    let toml_content = "title = \"My Post\"\ndate = 2025-09-09\n";
    let frontmatter = parse(toml_content, Format::Toml)?;

    println!("    ✅  Parsed frontmatter: {:?}", frontmatter);

    Ok(())
}

/// Demonstrates parsing JSON frontmatter.
fn parse_json_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 JSON Parsing Example");
    println!("---------------------------------------------");

    let json_content = r#"{"title": "My Post", "date": "2025-09-09"}"#;
    let frontmatter = parse(json_content, Format::Json)?;

    println!("    ✅  Parsed frontmatter: {:?}", frontmatter);

    Ok(())
}

/// Demonstrates serializing frontmatter to YAML.
fn serialize_to_yaml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 YAML Serialization Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    frontmatter.insert(
        "date".to_string(),
        Value::String("2025-09-09".to_string()),
    );

    let yaml = to_string(&frontmatter, Format::Yaml)?;

    println!("    ✅  Serialized to YAML:\n{}", yaml);

    Ok(())
}

/// Demonstrates serializing frontmatter to TOML.
fn serialize_to_toml_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 TOML Serialization Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    frontmatter.insert(
        "date".to_string(),
        Value::String("2025-09-09".to_string()),
    );

    let toml = to_string(&frontmatter, Format::Toml)?;

    println!("    ✅  Serialized to TOML:\n{}", toml);

    Ok(())
}

/// Demonstrates serializing frontmatter to JSON.
fn serialize_to_json_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 JSON Serialization Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    frontmatter.insert(
        "date".to_string(),
        Value::String("2025-09-09".to_string()),
    );

    let json = to_string(&frontmatter, Format::Json)?;

    println!("    ✅  Serialized to JSON:\n{}", json);

    Ok(())
}
