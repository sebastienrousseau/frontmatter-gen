// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Types Examples
//!
//! This example demonstrates the core types and functionality provided by the
//! FrontMatterGen crate, including:
//!
//! - `Format` enum for frontmatter formats
//! - `Value` type for flexible data representation
//! - `Frontmatter` struct for key-value storage
//!
//! ## Usage
//!
//! To run this example:
//!
//! ```bash
//! cargo run --features default --example types_examples
//! ```

use frontmatter_gen::{Format, Frontmatter, Value};
use std::f64::consts::PI;

/// Main function demonstrating the usage of FrontMatterGen types.
///
/// This function showcases examples for the `Format` enum, `Value` type, and
/// `Frontmatter` struct, along with additional SSG-specific examples.
///
/// # Errors
/// Returns an error if any of the examples fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Types Examples\n");

    // Core functionality examples
    format_examples()?;
    value_examples()?;
    frontmatter_examples()?;

    // SSG-specific examples
    #[cfg(feature = "ssg")]
    ssg_type_examples()?;

    println!("\n🎉 All types examples completed successfully!");
    Ok(())
}

/// Demonstrates the usage of the `Format` enum.
///
/// The `Format` enum defines the supported frontmatter formats: YAML, TOML, and JSON.
///
/// # Errors
/// Returns an error if an assertion fails.
fn format_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Format Enum Example");
    println!("---------------------------------------------");

    let default_format = Format::default();
    println!("    ✅ Default format: {:?}", default_format);

    let yaml_format = Format::Yaml;
    println!("    ✅ YAML format: {:?}", yaml_format);

    let toml_format = Format::Toml;
    println!("    ✅ TOML format: {:?}", toml_format);

    let json_format = Format::Json;
    println!("    ✅ JSON format: {:?}", json_format);

    // Basic assertions
    assert_eq!(Format::default(), Format::Json);
    assert_ne!(Format::Yaml, Format::Toml);

    Ok(())
}

/// Demonstrates the usage of the `Value` enum.
///
/// The `Value` enum represents different data types, such as strings, numbers,
/// booleans, arrays, and objects.
///
/// # Errors
/// Returns an error if an assertion fails.
fn value_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Value Enum Example");
    println!("---------------------------------------------");

    // String value example
    let string_value = Value::String("Hello".to_string());
    println!("    ✅ String value: {:?}", string_value);
    assert_eq!(string_value.as_str().unwrap(), "Hello");

    // Number value example
    let number_value = Value::Number(PI);
    println!("    ✅ Number value: {:?}", number_value);
    assert_eq!(number_value.as_f64().unwrap(), PI);

    // Boolean value example
    let bool_value = Value::Boolean(true);
    println!("    ✅ Boolean value: {:?}", bool_value);
    assert!(bool_value.as_bool().unwrap());

    // Array value example
    let array_value =
        Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    println!("    ✅ Array value: {:?}", array_value);
    assert_eq!(array_value.array_len().unwrap(), 2);

    // Object value example
    let mut fm = Frontmatter::new();
    let _ = fm
        .insert("key".to_string(), Value::String("value".to_string()));
    let object_value = Value::Object(Box::new(fm.clone()));
    println!("    ✅ Object value: {:?}", object_value);
    assert!(object_value.is_object());

    Ok(())
}

/// Demonstrates the usage of the `Frontmatter` struct.
///
/// The `Frontmatter` struct is a key-value store for metadata.
///
/// # Errors
/// Returns an error if an assertion fails.
fn frontmatter_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Frontmatter Struct Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();

    // Insert and retrieve values
    let _ = frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    let _ =
        frontmatter.insert("views".to_string(), Value::Number(100.0));
    println!("    ✅ Frontmatter with two entries: {:?}", frontmatter);

    let title = frontmatter.get("title").unwrap().as_str().unwrap();
    let views = frontmatter.get("views").unwrap().as_f64().unwrap();
    println!("    Title: {}", title);
    println!("    Views: {}", views);

    // Test removal
    let _ = frontmatter.remove("views");
    println!(
        "    ✅ Frontmatter after removing 'views': {:?}",
        frontmatter
    );
    assert!(frontmatter.get("views").is_none());
    assert_eq!(frontmatter.len(), 1);

    Ok(())
}

/// SSG-specific examples for the `Frontmatter` struct.
///
/// This example demonstrates how to add and manipulate metadata commonly used
/// in static site generators (SSGs).
///
/// # Errors
/// Returns an error if any operation fails.
#[cfg(feature = "ssg")]
fn ssg_type_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 SSG-Specific Types Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();

    // Add SSG-specific metadata
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

    // Add nested metadata
    let mut metadata = Frontmatter::new();
    let _ = metadata.insert(
        "author".to_string(),
        Value::String("John Doe".to_string()),
    );
    let _ = metadata.insert(
        "category".to_string(),
        Value::String("Programming".to_string()),
    );
    let _ = frontmatter.insert(
        "metadata".to_string(),
        Value::Object(Box::new(metadata)),
    );

    println!("    ✅ SSG Frontmatter:\n{:#?}", frontmatter);

    // Demonstrate type checking and access
    if let Some(tags) =
        frontmatter.get("tags").and_then(Value::as_array)
    {
        println!("\n    Tags:");
        for tag in tags {
            if let Some(tag_str) = tag.as_str() {
                println!("    - {}", tag_str);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic operations with `Value`.
    #[test]
    fn test_value_types() {
        let string_val = Value::String("test".to_string());
        assert!(string_val.is_string());
        assert_eq!(string_val.as_str().unwrap(), "test");

        let num_val = Value::Number(42.0);
        assert!(num_val.is_number());
        assert_eq!(num_val.as_f64().unwrap(), 42.0);
    }

    /// Tests basic operations with `Frontmatter`.
    #[test]
    fn test_frontmatter_operations() {
        let mut fm = Frontmatter::new();
        fm.insert(
            "test".to_string(),
            Value::String("value".to_string()),
        );

        assert!(fm.contains_key("test"));
        assert_eq!(fm.get("test").unwrap().as_str().unwrap(), "value");

        fm.remove("test");
        assert!(!fm.contains_key("test"));
    }

    /// Tests SSG-specific operations.
    #[cfg(feature = "ssg")]
    #[test]
    fn test_ssg_metadata() {
        let mut fm = Frontmatter::new();
        fm.insert(
            "template".to_string(),
            Value::String("post".to_string()),
        );
        fm.insert("draft".to_string(), Value::Boolean(false));
        fm.insert(
            "tags".to_string(),
            Value::Array(vec![Value::String("rust".to_string())]),
        );

        assert_eq!(
            fm.get("template").unwrap().as_str().unwrap(),
            "post"
        );
        assert!(!fm.get("draft").unwrap().as_bool().unwrap());
        assert_eq!(fm.get("tags").unwrap().array_len().unwrap(), 1);
    }
}
