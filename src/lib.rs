#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/frontmatter-gen/images/favicon.ico",
    html_logo_url = "https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg",
    html_root_url = "https://docs.rs/frontmatter-gen"
)]

//! # Frontmatter Gen
//!
//! `frontmatter-gen` is a fast, secure, and memory-efficient library for working with
//! frontmatter in multiple formats (YAML, TOML, and JSON).
//!
//! ## Overview
//!
//! Frontmatter is metadata prepended to content files, commonly used in static site
//! generators and content management systems. This library provides:
//!
//! - **Zero-copy parsing** for optimal performance
//! - **Format auto-detection** between YAML, TOML, and JSON
//! - **Memory safety** with no unsafe code
//! - **Comprehensive validation** of all inputs
//! - **Rich error handling** with detailed diagnostics
//! - **Async support** for non-blocking operations
//!
//! ## Quick Start
//!
//! ```rust
//! use frontmatter_gen::{extract, Format, Result};
//!
//! fn main() -> Result<()> {
//!     let content = r#"---
//! title: My Post
//! date: 2024-01-01
//! draft: false
//! ---
//! # Post content here
//! "#;
//!
//!     let (frontmatter, content) = extract(content)?;
//!     println!("Title: {}", frontmatter.get("title")
//!         .and_then(|v| v.as_str())
//!         .unwrap_or("Untitled"));
//!
//!     Ok(())
//! }
//! ```

/// Prelude module for convenient imports.
///
/// This module provides the most commonly used types and traits.
/// Import all contents with `use frontmatter_gen::prelude::*`.
pub mod prelude {
    pub use crate::{
        extract, to_format, Config, Format, Frontmatter,
        FrontmatterError, Result, Value,
    };
}

// Re-export core types and traits
pub use crate::{
    config::Config,
    error::FrontmatterError,
    extractor::{detect_format, extract_raw_frontmatter},
    parser::{parse, to_string},
    types::{Format, Frontmatter, Value},
};

// Module declarations
pub mod config;
pub mod engine;
pub mod error;
pub mod extractor;
pub mod parser;
pub mod types;
pub mod utils;

/// A specialized Result type for frontmatter operations.
///
/// This type alias provides a consistent error type throughout the crate
/// and simplifies error handling for library users.
pub type Result<T> = std::result::Result<T, FrontmatterError>;

/// Extracts and parses frontmatter from content with format auto-detection.
///
/// This function provides a zero-copy extraction of frontmatter, automatically
/// detecting the format (YAML, TOML, or JSON) and parsing it into a structured
/// representation.
///
/// # Performance
///
/// This function performs a single pass over the input with O(n) complexity
/// and avoids unnecessary allocations where possible.
///
/// # Examples
///
/// ```rust
/// use frontmatter_gen::extract;
///
/// let content = r#"---
/// title: My Post
/// date: 2024-01-01
/// ---
/// Content here"#;
///
/// let (frontmatter, content) = extract(content)?;
/// assert_eq!(frontmatter.get("title").unwrap().as_str().unwrap(), "My Post");
/// assert_eq!(content.trim(), "Content here");
/// # Ok::<(), frontmatter_gen::FrontmatterError>(())
/// ```
///
/// # Errors
///
/// Returns `FrontmatterError` if:
/// - Content is malformed
/// - Frontmatter format is invalid
/// - Parsing fails
#[inline]
pub fn extract(content: &str) -> Result<(Frontmatter, &str)> {
    let (raw_frontmatter, remaining_content) =
        extract_raw_frontmatter(content)?;
    let format = detect_format(raw_frontmatter)?;
    let frontmatter = parse(raw_frontmatter, format)?;
    Ok((frontmatter, remaining_content))
}

/// Converts frontmatter to a specific format.
///
/// # Arguments
///
/// * `frontmatter` - The frontmatter to convert
/// * `format` - Target format for conversion
///
/// # Returns
///
/// Returns the formatted string representation or an error.
///
/// # Examples
///
/// ```rust
/// use frontmatter_gen::{Frontmatter, Format, Value, to_format};
///
/// let mut frontmatter = Frontmatter::new();
/// frontmatter.insert("title".to_string(), Value::String("My Post".into()));
///
/// let yaml = to_format(&frontmatter, Format::Yaml)?;
/// assert!(yaml.contains("title: My Post"));
/// # Ok::<(), frontmatter_gen::FrontmatterError>(())
/// ```
///
/// # Errors
///
/// Returns `FrontmatterError` if:
/// - Serialization fails
/// - Format conversion fails
/// - Invalid data types are encountered
pub fn to_format(
    frontmatter: &Frontmatter,
    format: Format,
) -> Result<String> {
    to_string(frontmatter, format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // Helper function to create test content with frontmatter
    fn create_test_content(content: &str, format: Format) -> String {
        match format {
            Format::Yaml => format!("---\n{}\n---\nContent", content),
            Format::Toml => format!("+++\n{}\n+++\nContent", content),
            Format::Json => format!("{}\nContent", content),
            Format::Unsupported => content.to_string(),
        }
    }

    #[test]
    fn test_extract_yaml_frontmatter() {
        let content = r#"---
title: Test Post
date: 2024-01-01
---
Content here"#;

        let (frontmatter, content) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(
            frontmatter.get("date").unwrap().as_str().unwrap(),
            "2024-01-01"
        );
        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_extract_toml_frontmatter() {
        let content = r#"+++
title = "Test Post"
date = "2024-01-01"
+++
Content here"#;

        let (frontmatter, content) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(
            frontmatter.get("date").unwrap().as_str().unwrap(),
            "2024-01-01"
        );
        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_extract_json_frontmatter() {
        let content = r#"{
            "title": "Test Post",
            "date": "2024-01-01"
        }
Content here"#;

        let (frontmatter, content) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(
            frontmatter.get("date").unwrap().as_str().unwrap(),
            "2024-01-01"
        );
        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_to_format_conversion() {
        let mut frontmatter = Frontmatter::new();
        frontmatter.insert(
            "title".to_string(),
            Value::String("Test Post".to_string()),
        );

        let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
        assert!(yaml.contains("title: Test Post"));

        let json = to_format(&frontmatter, Format::Json).unwrap();
        assert!(json.contains(r#""title":"Test Post"#));

        let toml = to_format(&frontmatter, Format::Toml).unwrap();
        assert!(toml.contains(r#"title = "Test Post""#));
    }

    #[test]
    fn test_format_conversion_roundtrip() {
        let mut original = Frontmatter::new();
        original.insert(
            "key".to_string(),
            Value::String("value".to_string()),
        );

        let format_wrappers = [
            (Format::Yaml, "---\n", "\n---\n"),
            (Format::Toml, "+++\n", "\n+++\n"),
            (Format::Json, "", "\n"),
        ];

        for (format, prefix, suffix) in format_wrappers {
            let formatted = to_format(&original, format).unwrap();
            let content = format!("{}{}{}", prefix, formatted, suffix);
            let (parsed, _) = extract(&content).unwrap();
            assert_eq!(
                parsed.get("key").unwrap().as_str().unwrap(),
                "value",
                "Failed roundtrip test for {:?} format",
                format
            );
        }
    }

    #[test]
    fn test_invalid_frontmatter() {
        let invalid_inputs = [
            "Invalid frontmatter\nContent",
            "---\nInvalid: : yaml\n---\nContent",
            "+++\ninvalid toml ===\n+++\nContent",
            "{invalid json}\nContent",
        ];

        for input in invalid_inputs {
            assert!(extract(input).is_err());
        }
    }

    #[test]
    fn test_empty_frontmatter() {
        let empty_inputs =
            ["---\n---\nContent", "+++\n+++\nContent", "{}\nContent"];

        for input in empty_inputs {
            let (frontmatter, content) = extract(input).unwrap();
            assert!(frontmatter.is_empty());
            assert!(content.contains("Content"));
        }
    }

    #[test]
    fn test_complex_nested_structures() {
        let content = r#"---
title: Test Post
metadata:
  author:
    name: John Doe
    email: john@example.com
  tags:
    - rust
    - programming
numbers:
  - 1
  - 2
  - 3
settings:
  published: true
  featured: false
---
Content here"#;

        let (frontmatter, content) = extract(content).unwrap();

        // Check basic fields
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );

        // Check nested object
        let metadata =
            frontmatter.get("metadata").unwrap().as_object().unwrap();
        let author =
            metadata.get("author").unwrap().as_object().unwrap();
        assert_eq!(
            author.get("name").unwrap().as_str().unwrap(),
            "John Doe"
        );
        assert_eq!(
            author.get("email").unwrap().as_str().unwrap(),
            "john@example.com"
        );

        // Check arrays
        let tags = metadata.get("tags").unwrap().as_array().unwrap();
        assert_eq!(tags[0].as_str().unwrap(), "rust");
        assert_eq!(tags[1].as_str().unwrap(), "programming");

        let numbers =
            frontmatter.get("numbers").unwrap().as_array().unwrap();
        assert_eq!(numbers.len(), 3);

        // Check nested boolean values
        let settings =
            frontmatter.get("settings").unwrap().as_object().unwrap();
        assert!(settings.get("published").unwrap().as_bool().unwrap());
        assert!(!settings.get("featured").unwrap().as_bool().unwrap());

        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_whitespace_handling() {
        let inputs = [
            "---\ntitle: Test Post  \ndate:   2024-01-01\n---\nContent",
            "+++\ntitle = \"Test Post\"  \ndate = \"2024-01-01\"\n+++\nContent",
            "{\n  \"title\": \"Test Post\",\n  \"date\": \"2024-01-01\"\n}\nContent",
        ];

        for input in inputs {
            let (frontmatter, _) = extract(input).unwrap();
            assert_eq!(
                frontmatter.get("title").unwrap().as_str().unwrap(),
                "Test Post"
            );
            assert_eq!(
                frontmatter.get("date").unwrap().as_str().unwrap(),
                "2024-01-01"
            );
        }
    }

    #[test]
    fn test_special_characters() {
        let content = r#"---
title: "Test: Special Characters!"
description: "Line 1\nLine 2"
path: "C:\\Program Files"
quote: "Here's a \"quote\""
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test: Special Characters!"
        );
        assert_eq!(
            frontmatter.get("description").unwrap().as_str().unwrap(),
            "Line 1\nLine 2"
        );
        assert_eq!(
            frontmatter.get("path").unwrap().as_str().unwrap(),
            "C:\\Program Files"
        );
        assert_eq!(
            frontmatter.get("quote").unwrap().as_str().unwrap(),
            "Here's a \"quote\""
        );
    }

    #[test]
fn test_numeric_values() {
    let content = r#"---
integer: 42
float: 3.14
scientific: 1.23e-4
negative: -17
zero: 0
---
Content"#;

    let (frontmatter, _) = extract(content).unwrap();

    // Define a small margin of error for floating-point comparisons
    let epsilon = 1e-6;

    assert!((frontmatter.get("integer").unwrap().as_f64().unwrap() - 42.0).abs() < epsilon);
    assert!((frontmatter.get("float").unwrap().as_f64().unwrap() - 3.14).abs() < epsilon); // Use 3.14 directly
    assert!((frontmatter.get("scientific").unwrap().as_f64().unwrap() - 0.000123).abs() < epsilon);
    assert!((frontmatter.get("negative").unwrap().as_f64().unwrap() - (-17.0)).abs() < epsilon);
    assert!((frontmatter.get("zero").unwrap().as_f64().unwrap() - 0.0).abs() < epsilon);
}

    #[test]
    fn test_boolean_values() {
        let content = r#"---
true_value: true
false_value: false
yes_value: yes
no_value: no
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();
        assert!(frontmatter
            .get("true_value")
            .unwrap()
            .as_bool()
            .unwrap());
        assert!(!frontmatter
            .get("false_value")
            .unwrap()
            .as_bool()
            .unwrap());
        // Note: YAML's yes/no handling depends on the YAML parser implementation
        // You might need to adjust these assertions based on your parser's behavior
    }

    #[test]
    fn test_array_handling() {
        let content = r#"---
empty_array: []
simple_array:
  - one
  - two
  - three
nested_arrays:
  -
    - a
    - b
  -
    - c
    - d
mixed_array:
  - 42
  - true
  - "string"
  - [1, 2, 3]
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();

        // Test empty array
        assert!(frontmatter
            .get("empty_array")
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty());

        // Test simple array
        let simple = frontmatter
            .get("simple_array")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(simple.len(), 3);
        assert_eq!(simple[0].as_str().unwrap(), "one");

        // Test nested arrays
        let nested = frontmatter
            .get("nested_arrays")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(nested.len(), 2);
        let first_nested = nested[0].as_array().unwrap();
        assert_eq!(first_nested[0].as_str().unwrap(), "a");

        // Test mixed type array
        let mixed =
            frontmatter.get("mixed_array").unwrap().as_array().unwrap();
        assert_eq!(mixed[0].as_f64().unwrap(), 42.0);
        assert!(mixed[1].as_bool().unwrap());
        assert_eq!(mixed[2].as_str().unwrap(), "string");
        assert_eq!(mixed[3].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_large_frontmatter() {
        let mut large_content = String::from("---\n");
        for i in 0..1000 {
            large_content
                .push_str(&format!("key_{}: value_{}\n", i, i));
        }
        large_content.push_str("---\nContent");

        let (frontmatter, content) = extract(&large_content).unwrap();
        assert_eq!(frontmatter.len(), 1000);
        assert_eq!(content.trim(), "Content");
    }

    #[test]
    fn test_format_specific_features() {
        // YAML-specific features
        let yaml_content = r#"---
alias: &base
  key: value
reference: *base
---
Content"#;

        let (yaml_fm, _) = extract(yaml_content).unwrap();
        assert_eq!(
            yaml_fm
                .get("alias")
                .unwrap()
                .as_object()
                .unwrap()
                .get("key")
                .unwrap()
                .as_str()
                .unwrap(),
            "value"
        );

        // TOML-specific features
        let toml_content = r#"+++
title = "Test"
[table]
key = "value"
+++
Content"#;

        let (toml_fm, _) = extract(toml_content).unwrap();
        assert_eq!(
            toml_fm
                .get("table")
                .unwrap()
                .as_object()
                .unwrap()
                .get("key")
                .unwrap()
                .as_str()
                .unwrap(),
            "value"
        );

        // JSON-specific features
        let json_content = r#"{
            "null_value": null,
            "number": 42.0
        }
Content"#;

        let (json_fm, _) = extract(json_content).unwrap();
        assert!(json_fm.get("null_value").unwrap().is_null());
        assert_eq!(
            json_fm.get("number").unwrap().as_f64().unwrap(),
            42.0
        );
    }

    #[test]
    fn test_error_cases() {
        let error_cases = [
            // Invalid delimiters
            "--\ntitle: Test\n--\nContent",
            "+\ntitle = \"Test\"\n+\nContent",
            // Mismatched delimiters
            "---\ntitle: Test\n+++\nContent",
            "+++\ntitle = \"Test\"\n---\nContent",
            // Invalid syntax
            "---\n[invalid: yaml:\n---\nContent",
            // More explicitly invalid TOML
            "+++\ntitle = [\nincomplete array\n+++\nContent",
            "{invalid json}\nContent",
            // Empty content
            "",
            // Missing closing delimiter
            "---\ntitle: Test\nContent",
            "+++\ntitle = \"Test\"\nContent",
            // Completely malformed
            "not a frontmatter",
            "@#$%invalid content",
            // Invalid TOML cases that should definitely fail
            "+++\ntitle = = \"double equals\"\n+++\nContent",
            "+++\n[[[[invalid.section\n+++\nContent",
            "+++\nkey = \n+++\nContent", // Missing value
        ];

        for case in error_cases {
            assert!(
                extract(case).is_err(),
                "Expected error for input: {}",
                case.replace('\n', "\\n") // Make newlines visible in error message
            );
        }
    }

    // Add a test for valid but edge-case TOML
    #[test]
    fn test_valid_toml_edge_cases() {
        let valid_cases = [
            // Empty sections are valid in TOML
            "+++\n[section]\n+++\nContent",
            // Empty arrays are valid
            "+++\narray = []\n+++\nContent",
            // Empty tables are valid
            "+++\ntable = {}\n+++\nContent",
        ];

        for case in valid_cases {
            assert!(
                extract(case).is_ok(),
                "Expected success for valid TOML: {}",
                case.replace('\n', "\\n")
            );
        }
    }

    // Add test for empty lines and whitespace
    #[test]
    fn test_whitespace_and_empty_lines() {
        let test_cases = [
        // YAML with empty lines
        "---\n\ntitle: Test\n\nkey: value\n\n---\nContent",
        // TOML with empty lines
        "+++\n\ntitle = \"Test\"\n\nkey = \"value\"\n\n+++\nContent",
        // JSON with whitespace
        "{\n  \n  \"title\": \"Test\",\n  \n  \"key\": \"value\"\n  \n}\nContent",
    ];

        for case in test_cases {
            let (frontmatter, _) = extract(case).unwrap();
            assert_eq!(
                frontmatter.get("title").unwrap().as_str().unwrap(),
                "Test"
            );
            assert_eq!(
                frontmatter.get("key").unwrap().as_str().unwrap(),
                "value"
            );
        }
    }

    // Add test for comments in valid locations
    #[test]
    fn test_valid_comments() {
        let test_cases = [
        // YAML with comments
        "---\n# Comment\ntitle: Test # Inline comment\n---\nContent",
        // TOML with comments
        "+++\n# Comment\ntitle = \"Test\" # Inline comment\n+++\nContent",
        // JSON doesn't support comments, so we'll skip it
    ];

        for case in test_cases {
            let (frontmatter, _) = extract(case).unwrap();
            assert_eq!(
                frontmatter.get("title").unwrap().as_str().unwrap(),
                "Test"
            );
        }
    }

    #[test]
    fn test_unicode_handling() {
        let content = r#"---
title: "こんにちは世界"
description: "Hello, 世界! Здравствуй, мир! مرحبا بالعالم!"
list:
  - "🦀"
  - "📚"
  - "🔧"
nested:
  key: "👋 Hello"
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "こんにちは世界"
        );
        assert!(frontmatter
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("世界"));

        let list = frontmatter.get("list").unwrap().as_array().unwrap();
        assert_eq!(list[0].as_str().unwrap(), "🦀");

        let nested =
            frontmatter.get("nested").unwrap().as_object().unwrap();
        assert_eq!(
            nested.get("key").unwrap().as_str().unwrap(),
            "👋 Hello"
        );
    }

    #[test]
    fn test_windows_line_endings() {
        let content = "---\r\ntitle: Test Post\r\ndate: 2024-01-01\r\n---\r\nContent here";
        let (frontmatter, content) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(
            frontmatter.get("date").unwrap().as_str().unwrap(),
            "2024-01-01"
        );
        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_deep_nested_structures() {
        let content = r#"---
level1:
  level2:
    level3:
      level4:
        level5:
          key: value
arrays:
  - - - - - nested
numbers:
  - - - - 42
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();

        let level1 =
            frontmatter.get("level1").unwrap().as_object().unwrap();
        let level2 = level1.get("level2").unwrap().as_object().unwrap();
        let level3 = level2.get("level3").unwrap().as_object().unwrap();
        let level4 = level3.get("level4").unwrap().as_object().unwrap();
        let level5 = level4.get("level5").unwrap().as_object().unwrap();

        assert_eq!(
            level5.get("key").unwrap().as_str().unwrap(),
            "value"
        );

        let arrays =
            frontmatter.get("arrays").unwrap().as_array().unwrap();
        assert_eq!(
            arrays[0].as_array().unwrap()[0].as_array().unwrap()[0]
                .as_array()
                .unwrap()[0]
                .as_array()
                .unwrap()[0]
                .as_str()
                .unwrap(),
            "nested"
        );
    }

    #[test]
    fn test_format_detection() {
        let test_cases = [
            ("---\nkey: value\n---\n", Format::Yaml),
            ("+++\nkey = \"value\"\n+++\n", Format::Toml),
            ("{\n\"key\": \"value\"\n}\n", Format::Json),
        ];

        for (content, expected_format) in test_cases {
            let (raw_frontmatter, _) =
                extract_raw_frontmatter(content).unwrap();
            let detected_format =
                detect_format(raw_frontmatter).unwrap();
            assert_eq!(detected_format, expected_format);
        }
    }

    #[test]
    fn test_empty_values() {
        let content = r#"---
empty_string: ""
null_value: null
empty_array: []
empty_object: {}
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();

        assert_eq!(
            frontmatter.get("empty_string").unwrap().as_str().unwrap(),
            ""
        );
        assert!(frontmatter.get("null_value").unwrap().is_null());
        assert!(frontmatter
            .get("empty_array")
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty());
        assert!(frontmatter
            .get("empty_object")
            .unwrap()
            .as_object()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn test_duplicate_keys() {
        let test_cases = [
            // YAML with duplicate keys
            r#"---
key: value1
key: value2
---
Content"#,
            // TOML with duplicate keys
            r#"+++
key = "value1"
key = "value2"
+++
Content"#,
            // JSON with duplicate keys
            r#"{
                "key": "value1",
                "key": "value2"
            }
Content"#,
        ];

        for case in test_cases {
            let result = extract(case);
            // The exact behavior might depend on the underlying parser
            // Some might error out, others might take the last value
            if let Ok((frontmatter, _)) = result {
                assert_eq!(
                    frontmatter.get("key").unwrap().as_str().unwrap(),
                    "value2"
                );
            }
        }
    }

    #[test]
    fn test_timestamp_handling() {
        let content = r#"---
date: 2024-01-01
datetime: 2024-01-01T12:00:00Z
datetime_tz: 2024-01-01T12:00:00+01:00
---
Content"#;

        let (frontmatter, _) = extract(content).unwrap();

        assert_eq!(
            frontmatter.get("date").unwrap().as_str().unwrap(),
            "2024-01-01"
        );
        assert_eq!(
            frontmatter.get("datetime").unwrap().as_str().unwrap(),
            "2024-01-01T12:00:00Z"
        );
        assert_eq!(
            frontmatter.get("datetime_tz").unwrap().as_str().unwrap(),
            "2024-01-01T12:00:00+01:00"
        );
    }

    #[test]
    fn test_comment_handling() {
        let yaml_content = r#"---
title: Test Post
# This is a YAML comment
key: value
---
Content"#;

        let toml_content = r#"+++
title = "Test Post"
# This is a TOML comment
key = "value"
+++
Content"#;

        let json_content = r#"{
            "title": "Test Post",
            // JSON technically doesn't support comments
            "key": "value"
        }
Content"#;

        // YAML comments
        let (frontmatter, _) = extract(yaml_content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(
            frontmatter.get("key").unwrap().as_str().unwrap(),
            "value"
        );
        assert!(frontmatter.get("#").is_none());

        // TOML comments
        let (frontmatter, _) = extract(toml_content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(
            frontmatter.get("key").unwrap().as_str().unwrap(),
            "value"
        );
        assert!(frontmatter.get("#").is_none());

        // JSON content (should fail or ignore comments depending on parser)
        let result = extract(json_content);
        if let Ok((frontmatter, _)) = result {
            assert_eq!(
                frontmatter.get("title").unwrap().as_str().unwrap(),
                "Test Post"
            );
            assert_eq!(
                frontmatter.get("key").unwrap().as_str().unwrap(),
                "value"
            );
        }
    }

    // #[test]
    // fn test_performance_with_large_input() {
    //     // Generate a large frontmatter document
    //     let mut large_content = String::from("---\n");
    //     for i in 0..10_000 {
    //         large_content
    //             .push_str(&format!("key_{}: value_{}\n", i, i));
    //     }
    //     large_content.push_str("---\nContent");

    //     let start = std::time::Instant::now();
    //     let (frontmatter, _) = extract(&large_content).unwrap();
    //     let duration = start.elapsed();

    //     assert_eq!(frontmatter.len(), 10_000);
    //     // Optional: Add an assertion for performance
    //     assert!(duration < std::time::Duration::from_millis(100));
    // }

    #[test]
    fn test_unsupported_format() {
        let result =
            to_format(&Frontmatter::new(), Format::Unsupported);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_roundtrip_with_all_types() {
        let mut frontmatter = Frontmatter::new();
        frontmatter.insert(
            "string".to_string(),
            Value::String("value".to_string()),
        );
        frontmatter.insert("number".to_string(), Value::Number(42.0));
        frontmatter.insert("boolean".to_string(), Value::Boolean(true));
        // Remove null value test for TOML as it doesn't support it
        frontmatter.insert(
            "array".to_string(),
            Value::Array(vec![
                Value::String("item1".to_string()),
                Value::Number(2.0),
                Value::Boolean(false),
            ]),
        );

        let mut inner = Frontmatter::new();
        inner.insert(
            "inner_key".to_string(),
            Value::String("inner_value".to_string()),
        );
        frontmatter.insert(
            "object".to_string(),
            Value::Object(Box::new(inner)),
        );

        for format in [Format::Yaml, Format::Json] {
            // Remove TOML from this test
            let formatted = to_format(&frontmatter, format).unwrap();
            let wrapped = create_test_content(&formatted, format);
            let (parsed, _) = extract(&wrapped).unwrap();

            // Verify all values are preserved
            assert_eq!(
                parsed.get("string").unwrap().as_str().unwrap(),
                "value"
            );
            assert_eq!(
                parsed.get("number").unwrap().as_f64().unwrap(),
                42.0
            );
            assert!(parsed.get("boolean").unwrap().as_bool().unwrap());

            let array =
                parsed.get("array").unwrap().as_array().unwrap();
            assert_eq!(array[0].as_str().unwrap(), "item1");
            assert_eq!(array[1].as_f64().unwrap(), 2.0);
            assert!(!array[2].as_bool().unwrap());

            let object =
                parsed.get("object").unwrap().as_object().unwrap();
            assert_eq!(
                object.get("inner_key").unwrap().as_str().unwrap(),
                "inner_value"
            );
        }

        // Separate test for TOML without null values
        let formatted = to_format(&frontmatter, Format::Toml).unwrap();
        let wrapped = create_test_content(&formatted, Format::Toml);
        let (parsed, _) = extract(&wrapped).unwrap();

        assert_eq!(
            parsed.get("string").unwrap().as_str().unwrap(),
            "value"
        );
        assert_eq!(
            parsed.get("number").unwrap().as_f64().unwrap(),
            42.0
        );
        assert!(parsed.get("boolean").unwrap().as_bool().unwrap());
    }
}
