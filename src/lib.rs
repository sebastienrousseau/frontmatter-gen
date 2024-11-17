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
mod extractor_tests {
    use super::*;

    #[test]
    fn test_extract_yaml_frontmatter() {
        let content = "---\ntitle: Test Post\n---\nContent here";
        let (frontmatter, remaining) =
            extract_raw_frontmatter(content).unwrap();
        assert_eq!(frontmatter, "title: Test Post");
        assert_eq!(remaining.trim(), "Content here");
    }

    #[test]
    fn test_extract_toml_frontmatter() {
        let content = "+++\ntitle = \"Test Post\"\n+++\nContent here";
        let (frontmatter, remaining) =
            extract_raw_frontmatter(content).unwrap();
        assert_eq!(frontmatter, "title = \"Test Post\"");
        assert_eq!(remaining.trim(), "Content here");
    }

    #[test]
    fn test_detect_format_yaml() {
        let frontmatter = "title: Test Post";
        let format = detect_format(frontmatter).unwrap();
        assert_eq!(format, Format::Yaml);
    }

    #[test]
    fn test_detect_format_toml() {
        let frontmatter = "title = \"Test Post\"";
        let format = detect_format(frontmatter).unwrap();
        assert_eq!(format, Format::Toml);
    }

    #[test]
    fn test_extract_no_frontmatter() {
        let content = "Content without frontmatter";
        let result = extract_raw_frontmatter(content);
        assert!(
            result.is_err(),
            "Should fail if no frontmatter delimiters are found"
        );
    }

    #[test]
    fn test_extract_partial_frontmatter() {
        let content = "---\ntitle: Incomplete";
        let result = extract_raw_frontmatter(content);
        assert!(
            result.is_err(),
            "Should fail for incomplete frontmatter"
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
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn test_to_format_yaml() {
        let mut frontmatter = Frontmatter::new();
        frontmatter.insert(
            "title".to_string(),
            Value::String("Test Post".to_string()),
        );
        let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
        assert!(yaml.contains("title: Test Post"));
    }

    #[test]
    fn test_format_conversion_roundtrip() {
        let mut frontmatter = Frontmatter::new();
        frontmatter.insert(
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
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_special_characters_handling() {
        let content =
            "---\ntitle: \"Test: Special Characters!\"\n---\nContent";
        let (frontmatter, _) = extract(content).unwrap();
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test: Special Characters!"
        );
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
}
