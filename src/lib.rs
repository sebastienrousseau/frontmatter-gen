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
//! This library provides robust handling of frontmatter with the following key features:
//!
//! - **Zero-copy parsing** for optimal memory efficiency
//! - **Type-safe operations** with comprehensive error handling
//! - **Multiple format support** (YAML, TOML, JSON)
//! - **Secure processing** with input validation and size limits
//! - **Async support** with the `ssg` feature flag
//!
//! ## Security Features
//!
//! - Input validation to prevent malicious content
//! - Size limits to prevent denial of service attacks
//! - Safe string handling to prevent memory corruption
//! - Secure path handling for file operations
//!
//! ## Quick Start
//!
//! ```rust
//! use frontmatter_gen::{extract, Format, Frontmatter, Result};
//!
//! let content = r#"---
//! title: Test Post
//! date: 2025-09-09
//! ---
//! Content here"#;
//!
//! let result = extract(content);
//! assert!(result.is_ok());
//! let (frontmatter, content) = result.unwrap();
//! assert_eq!(
//!     frontmatter.get("title").and_then(|v| v.as_str()),
//!     Some("Test Post")
//! );
//! assert_eq!(content.trim(), "Content here");
//! # Ok::<(), frontmatter_gen::FrontmatterError>(())
//! ```
//!
//! ## Feature Flags
//!
//! - `default`: Core frontmatter functionality
//! - `cli`: Command-line interface support
//! - `ssg`: Static Site Generator functionality (includes CLI)
//!
//! ## Error Handling
//!
//! All operations return a `Result` type with detailed error information:
//!
//! ```rust
//! use frontmatter_gen::{extract, FrontmatterError};
//!
//! fn process_content(content: &str) -> Result<(), FrontmatterError> {
//!     let (frontmatter, _) = extract(content)?;
//!
//!     // Validate required fields
//!     if !frontmatter.contains_key("title") {
//!         return Err(FrontmatterError::ValidationError(
//!             "Missing required field: title".to_string()
//!         ));
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::num::NonZeroUsize;

// Re-export core types and traits
pub use crate::{
    config::Config,
    error::FrontmatterError,
    extractor::{detect_format, extract_raw_frontmatter},
    parser::{parse, to_string},
    types::{Format, Frontmatter, Value},
};

// Module declarations
#[cfg(feature = "cli")]
pub mod cli;
pub mod config;
pub mod engine;
pub mod error;
pub mod extractor;
pub mod parser;
#[cfg(feature = "ssg")]
pub mod ssg;
pub mod types;
pub mod utils;

/// Maximum size allowed for frontmatter content (1MB)
pub const MAX_FRONTMATTER_SIZE: NonZeroUsize =
    unsafe { NonZeroUsize::new_unchecked(1024 * 1024) };

/// Maximum allowed nesting depth for structured data
pub const MAX_NESTING_DEPTH: NonZeroUsize =
    unsafe { NonZeroUsize::new_unchecked(32) };

/// A specialized Result type for frontmatter operations.
///
/// This type alias provides a consistent error type throughout the crate
/// and simplifies error handling for library users.
pub type Result<T> = std::result::Result<T, FrontmatterError>;

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

/// Configuration options for parsing operations.
///
/// Provides fine-grained control over parsing behaviour and security limits.
#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    /// Maximum allowed content size
    pub max_size: NonZeroUsize,
    /// Maximum allowed nesting depth
    pub max_depth: NonZeroUsize,
    /// Whether to validate content structure
    pub validate: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            max_size: MAX_FRONTMATTER_SIZE,
            max_depth: MAX_NESTING_DEPTH,
            validate: true,
        }
    }
}

/// Validates input content against security constraints.
///
/// # Security
///
/// This function helps prevent denial of service attacks by:
/// - Limiting the maximum size of frontmatter content
/// - Validating content structure
/// - Checking for malicious patterns
///
/// # Errors
///
/// Returns `FrontmatterError` if:
/// - Content exceeds maximum size
/// - Content contains invalid characters
/// - Content structure is invalid
#[inline]
fn validate_input(content: &str, options: &ParseOptions) -> Result<()> {
    // Size validation
    if content.len() > options.max_size.get() {
        return Err(FrontmatterError::ContentTooLarge {
            size: content.len(),
            max: options.max_size.get(),
        });
    }

    // Character validation
    if content.contains('\0') {
        return Err(FrontmatterError::ValidationError(
            "Content contains null bytes".to_string(),
        ));
    }

    // Control character validation (except whitespace)
    if content
        .chars()
        .any(|c| c.is_control() && !c.is_whitespace())
    {
        return Err(FrontmatterError::ValidationError(
            "Content contains invalid control characters".to_string(),
        ));
    }

    // Path traversal prevention
    if content.contains("../") || content.contains("..\\") {
        return Err(FrontmatterError::ValidationError(
            "Content contains path traversal patterns".to_string(),
        ));
    }

    // Line ending validation
    if content.contains("\r\n")
        && content.contains('\n')
        && !content.contains("\r\n")
    {
        return Err(FrontmatterError::ValidationError(
            "Mixed line endings detected".to_string(),
        ));
    }

    Ok(())
}

/// Extracts and parses frontmatter from content with format auto-detection.
///
/// This function provides zero-copy extraction of frontmatter where possible,
/// automatically detecting the format (YAML, TOML, or JSON) and parsing it
/// into a structured representation.
///
/// # Security
///
/// This function includes several security measures:
/// - Input validation and size limits
/// - Safe string handling
/// - Protection against malicious content
///
/// # Performance
///
/// Optimized for performance with:
/// - Zero-copy operations where possible
/// - Single-pass parsing
/// - Minimal allocations
/// - Pre-allocated buffers
///
/// # Examples
///
/// ```rust
/// use frontmatter_gen::extract;
///
/// let content = r#"---
/// title: My Post
/// date: 2025-09-09
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
/// - Content exceeds size limits
/// - Content is malformed
/// - Frontmatter format is invalid
/// - Parsing fails
pub fn extract(content: &str) -> Result<(Frontmatter, &str)> {
    let options = ParseOptions::default();
    validate_input(content, &options)?;

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
/// # Security
///
/// This function includes validation of:
/// - Input size limits
/// - Format compatibility
/// - Output safety
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
mod extractor_tests {
    use crate::FrontmatterError;

    fn mock_operation(
        input: Option<&str>,
    ) -> Result<String, FrontmatterError> {
        match input {
            Some(value) => Ok(value.to_uppercase()), // Successful operation
            None => Err(FrontmatterError::ParseError(
                "Input is missing".to_string(),
            )),
        }
    }

    #[test]
    fn test_result_type_success() {
        let input = Some("hello");
        let result = mock_operation(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO".to_string());
    }

    #[test]
    fn test_result_type_error() {
        let input = None;
        let result = mock_operation(input);
        assert!(matches!(
            result,
            Err(FrontmatterError::ParseError(ref e)) if e == "Input is missing"
        ));
    }

    #[test]
    fn test_result_type_pattern_matching() {
        let input = Some("world");
        let result = mock_operation(input);
        match result {
            Ok(value) => assert_eq!(value, "WORLD".to_string()),
            Err(e) => panic!("Operation failed: {:?}", e),
        }
    }

    #[test]
    fn test_result_type_unwrap() {
        let input = Some("rust");
        let result = mock_operation(input);
        assert_eq!(result.unwrap(), "RUST".to_string());
    }

    #[test]
    fn test_result_type_expect() {
        let input = Some("test");
        let result = mock_operation(input);
        assert_eq!(
            result.expect("Unexpected error"),
            "TEST".to_string()
        );
    }

    #[test]
    fn test_result_type_debug_format() {
        let input = None;
        let result = mock_operation(input);
        assert_eq!(
            format!("{:?}", result),
            "Err(ParseError(\"Input is missing\"))"
        );
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_yaml_frontmatter() {
        let raw = "title: Test Post\npublished: true";
        let format = Format::Yaml;
        let parsed = parse(raw, format).unwrap();
        assert_eq!(
            parsed.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert!(parsed.get("published").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_parse_toml_frontmatter() {
        let raw = "title = \"Test Post\"\npublished = true";
        let format = Format::Toml;
        let parsed = parse(raw, format).unwrap();
        assert_eq!(
            parsed.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert!(parsed.get("published").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_invalid_yaml_syntax() {
        let raw = "title: : invalid yaml";
        let format = Format::Yaml;
        let result = parse(raw, format);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_toml_syntax() {
        let raw = "title = \"Unmatched quote";
        let format = Format::Toml;
        let result = parse(raw, format);
        assert!(result.is_err(), "Should fail for invalid TOML syntax");
    }

    #[test]
    fn test_parse_invalid_json_syntax() {
        let raw = "{\"title\": \"Missing closing brace\"";
        let format = Format::Json;
        let result = parse(raw, format);
        assert!(result.is_err(), "Should fail for invalid JSON syntax");
    }

    #[test]
    fn test_parse_with_unknown_format() {
        let raw = "random text";
        let format = Format::Unsupported;
        let result = parse(raw, format);
        assert!(result.is_err(), "Should fail for unsupported formats");
    }

    #[test]
    fn test_parse_valid_yaml() {
        let raw = "title: Valid Post\npublished: true";
        let format = Format::Yaml;
        let frontmatter = parse(raw, format).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Valid Post"
        );
        assert!(frontmatter
            .get("published")
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[test]
    fn test_parse_malformed_yaml() {
        let raw = "title: : bad yaml";
        let format = Format::Yaml;
        let result = parse(raw, format);
        assert!(result.is_err(), "Should fail for malformed YAML");
    }

    #[test]
    fn test_parse_json() {
        let raw = r#"{"title": "Valid Post", "draft": false}"#;
        let format = Format::Json;
        let frontmatter = parse(raw, format).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Valid Post"
        );
        assert!(!frontmatter.get("draft").unwrap().as_bool().unwrap());
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn test_to_format_yaml() {
        let mut frontmatter = Frontmatter::new();
        let _ = frontmatter.insert(
            "title".to_string(),
            Value::String("Test Post".to_string()),
        );
        let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
        assert!(yaml.contains("title: Test Post"));
    }

    #[test]
    fn test_format_conversion_roundtrip() {
        let mut frontmatter = Frontmatter::new();
        let _ = frontmatter.insert(
            "key".to_string(),
            Value::String("value".to_string()),
        );
        let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
        let content = format!("---\n{}\n---\nContent", yaml);
        let (parsed, _) = extract(&content).unwrap();
        assert_eq!(
            parsed.get("key").unwrap().as_str().unwrap(),
            "value"
        );
    }

    #[test]
    fn test_unsupported_format() {
        let result =
            to_format(&Frontmatter::new(), Format::Unsupported);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_to_yaml() {
        let mut frontmatter = Frontmatter::new();
        let _ = frontmatter.insert(
            "title".to_string(),
            Value::String("Test Post".into()),
        );
        let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
        assert!(yaml.contains("title: Test Post"));
    }

    #[test]
    fn test_roundtrip_conversion() {
        let content = "---\ntitle: Test Post\n---\nContent";
        let (parsed, _) = extract(content).unwrap();
        let yaml = to_format(&parsed, Format::Yaml).unwrap();
        assert!(yaml.contains("title: Test Post"));
    }

    #[test]
    fn test_format_invalid_data() {
        let frontmatter = Frontmatter::new();
        let result = to_format(&frontmatter, Format::Unsupported);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_end_to_end_extraction_and_parsing() {
        let content = "---\ntitle: Test Post\n---\nContent here";
        let (frontmatter, content) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test Post"
        );
        assert_eq!(content.trim(), "Content here");
    }

    #[test]
    fn test_roundtrip_conversion() {
        let content = "---\ntitle: Test Post\n---\nContent";
        let (frontmatter, _) = extract(content).unwrap();
        let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
        assert!(yaml.contains("title: Test Post"));
    }

    #[test]
    fn test_complete_workflow() {
        let content = "---\ntitle: Integration Test\n---\nBody content";
        let (frontmatter, body) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Integration Test"
        );
        assert_eq!(body.trim(), "Body content");
    }

    #[test]
    fn test_end_to_end_error_handling() {
        let content = "Invalid frontmatter";
        let result = extract(content);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_special_characters_handling() {
        let cases = vec![
            (
                "---\ntitle: \"Special: &chars\"\n---\nContent",
                "Special: &chars",
            ),
            (
                "---\ntitle: \"Another > test\"\n---\nContent",
                "Another > test",
            ),
        ];

        for (content, expected_title) in cases {
            let (frontmatter, _) = extract(content).unwrap();
            assert_eq!(
                frontmatter.get("title").unwrap().as_str().unwrap(),
                expected_title
            );
        }
    }

    #[cfg(feature = "ssg")]
    #[tokio::test]
    async fn test_async_extraction() {
        let content = "---\ntitle: Async Test\n---\nContent";
        let (frontmatter, body) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Async Test"
        );
        assert_eq!(body.trim(), "Content");
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
    fn test_special_characters() {
        let content =
            "---\ntitle: \"Special & <characters>\"\n---\nContent";
        let (frontmatter, _) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Special & <characters>"
        );
    }
}

#[cfg(test)]
mod validate_input_tests {
    use super::*;

    #[test]
    fn test_validate_input_exceeds_max_size() {
        let options = ParseOptions::default();
        let oversized_content = "a".repeat(options.max_size.get() + 1);
        let result = validate_input(&oversized_content, &options);
        assert!(matches!(
            result,
            Err(FrontmatterError::ContentTooLarge { .. })
        ));
    }

    #[test]
    fn test_validate_input_contains_null_bytes() {
        let options = ParseOptions::default();
        let malicious_content = "title: Valid\0Post";
        let result = validate_input(malicious_content, &options);
        assert!(matches!(
            result,
            Err(FrontmatterError::ValidationError(ref e)) if e == "Content contains null bytes"
        ));
    }

    #[test]
    fn test_validate_input_path_traversal() {
        let options = ParseOptions::default();
        let malicious_content = "../malicious/path";
        let result = validate_input(malicious_content, &options);
        assert!(matches!(
            result,
            Err(FrontmatterError::ValidationError(ref e)) if e == "Content contains path traversal patterns"
        ));
    }
}
