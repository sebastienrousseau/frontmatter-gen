// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Types Examples
//!
//! This example demonstrates the core types and functionality provided by the
//! FrontMatterGen crate, including the `Format` enum, the `Value` type, and the
//! `Frontmatter` struct.

#![allow(missing_docs)]

use frontmatter_gen::{Format, Frontmatter, Value};
use std::f64::consts::PI;

/// Entry point for the FrontMatterGen types examples.
///
/// This function runs various examples demonstrating how to work with frontmatter
/// types, including `Format`, `Value`, and `Frontmatter`.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Types Examples\n");

    format_examples()?;
    value_examples()?;
    frontmatter_examples()?;

    println!("\n🎉  All types examples completed successfully!");

    Ok(())
}

/// Demonstrates the usage of the `Format` enum.
fn format_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Format Enum Example");
    println!("---------------------------------------------");

    let default_format = Format::default();
    println!("    ✅  Default format: {:?}", default_format);

    let yaml_format = Format::Yaml;
    println!("    ✅  YAML format: {:?}", yaml_format);

    let toml_format = Format::Toml;
    println!("    ✅  TOML format: {:?}", toml_format);

    let json_format = Format::Json;
    println!("    ✅  JSON format: {:?}", json_format);

    Ok(())
}

/// Demonstrates the usage of the `Value` enum.
fn value_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Value Enum Example");
    println!("---------------------------------------------");

    let string_value = Value::String("Hello".to_string());
    println!("    ✅  String value: {:?}", string_value);

    let number_value = Value::Number(PI);
    println!("    ✅  Number value: {:?}", number_value);

    let bool_value = Value::Boolean(true);
    println!("    ✅  Boolean value: {:?}", bool_value);

    let array_value =
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    println!("    ✅  Array value: {:?}", array_value);

    let mut fm = Frontmatter::new();
    fm.insert("key".to_string(), Value::String("value".to_string()));
    let object_value = Value::Object(Box::new(fm.clone()));
    println!("    ✅  Object value: {:?}", object_value);

    Ok(())
}

/// Demonstrates the usage of the `Frontmatter` struct.
fn frontmatter_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Frontmatter Struct Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    frontmatter.insert("views".to_string(), Value::Number(100.0));

    println!("    ✅  Frontmatter with two entries: {:?}", frontmatter);

    let title = frontmatter.get("title").unwrap().as_str().unwrap();
    let views = frontmatter.get("views").unwrap().as_f64().unwrap();

    println!("    Title: {}", title);
    println!("    Views: {}", views);

    frontmatter.remove("views");
    println!(
        "    ✅  Frontmatter after removing 'views': {:?}",
        frontmatter
    );

    Ok(())
}
