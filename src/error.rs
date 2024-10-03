//! This module defines the error types used throughout the frontmatter-gen crate.
//!
//! It provides a comprehensive set of error variants to cover various failure scenarios that may occur during frontmatter parsing, conversion, and extraction.

use serde_json::Error as JsonError;
use serde_yml::Error as YamlError;
use thiserror::Error;

/// Represents errors that can occur during frontmatter parsing, conversion, and extraction.
///
/// This enum uses the `thiserror` crate to provide clear and structured error messages, making it easier to debug and handle issues that arise when processing frontmatter.
#[derive(Error, Debug)]
pub enum FrontmatterError {
    /// Error occurred while parsing YAML.
    #[error("Failed to parse YAML: {source}")]
    YamlParseError {
        /// The source error from the YAML parser.
        source: YamlError,
    },

    /// Error occurred while parsing TOML.
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),

    /// Error occurred while parsing JSON.
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] JsonError),

    /// The frontmatter format is invalid or unsupported.
    #[error("Invalid frontmatter format")]
    InvalidFormat,

    /// Error occurred during conversion between formats.
    #[error("Failed to convert frontmatter: {0}")]
    ConversionError(String),

    /// Generic parse error.
    #[error("Failed to parse frontmatter: {0}")]
    ParseError(String),

    /// Error for unsupported or unknown frontmatter format.
    #[error("Unsupported frontmatter format detected at line {line}")]
    UnsupportedFormat {
        /// The line number where the unsupported format was detected.
        line: usize,
    },

    /// No frontmatter found in the content.
    #[error("No frontmatter found in the content")]
    NoFrontmatterFound,

    /// Invalid JSON frontmatter.
    #[error("Invalid JSON frontmatter")]
    InvalidJson,

    /// Invalid TOML frontmatter.
    #[error("Invalid TOML frontmatter")]
    InvalidToml,

    /// Invalid YAML frontmatter.
    #[error("Invalid YAML frontmatter")]
    InvalidYaml,

    /// JSON frontmatter exceeds maximum nesting depth.
    #[error("JSON frontmatter exceeds maximum nesting depth")]
    JsonDepthLimitExceeded,

    /// Error occurred during frontmatter extraction.
    #[error("Extraction error: {0}")]
    ExtractionError(String),
}

impl Clone for FrontmatterError {
    fn clone(&self) -> Self {
        match self {
            // For non-clonable errors, we fallback to a custom or default error.
            FrontmatterError::YamlParseError { .. } => {
                FrontmatterError::InvalidFormat
            }
            FrontmatterError::TomlParseError(e) => {
                FrontmatterError::TomlParseError(e.clone())
            }
            FrontmatterError::JsonParseError { .. } => {
                FrontmatterError::InvalidFormat
            }
            FrontmatterError::InvalidFormat => {
                FrontmatterError::InvalidFormat
            }
            FrontmatterError::ConversionError(msg) => {
                FrontmatterError::ConversionError(msg.clone())
            }
            FrontmatterError::ExtractionError(msg) => {
                FrontmatterError::ExtractionError(msg.clone())
            }
            FrontmatterError::ParseError(msg) => {
                FrontmatterError::ParseError(msg.clone())
            }
            FrontmatterError::UnsupportedFormat { line } => {
                FrontmatterError::UnsupportedFormat { line: *line }
            }
            FrontmatterError::NoFrontmatterFound => {
                FrontmatterError::NoFrontmatterFound
            }
            FrontmatterError::InvalidJson => {
                FrontmatterError::InvalidJson
            }
            FrontmatterError::InvalidToml => {
                FrontmatterError::InvalidToml
            }
            FrontmatterError::InvalidYaml => {
                FrontmatterError::InvalidYaml
            }
            FrontmatterError::JsonDepthLimitExceeded => {
                FrontmatterError::JsonDepthLimitExceeded
            }
        }
    }
}

impl FrontmatterError {
    /// Creates a generic parse error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice containing the error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use frontmatter_gen::error::FrontmatterError;
    /// let error = FrontmatterError::generic_parse_error("Failed to parse at line 10");
    /// ```
    pub fn generic_parse_error(message: &str) -> FrontmatterError {
        FrontmatterError::ParseError(message.to_string())
    }

    /// Helper function to create an `UnsupportedFormat` error with a given line number.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number where the unsupported format was detected.
    ///
    /// # Example
    ///
    /// ```rust
    /// use frontmatter_gen::error::FrontmatterError;
    /// let error = FrontmatterError::unsupported_format(12);
    /// ```
    pub fn unsupported_format(line: usize) -> FrontmatterError {
        FrontmatterError::UnsupportedFormat { line }
    }
}

/// Example usage of the `FrontmatterError` enum.
///
/// This function demonstrates how you might handle various errors during frontmatter parsing.
///
/// # Returns
///
/// Returns a `Result` demonstrating a parsing error.
pub fn example_usage() -> Result<(), FrontmatterError> {
    let example_toml = "invalid toml content";

    // Attempt to parse TOML and handle errors
    match toml::from_str::<toml::Value>(example_toml) {
        Ok(_) => Ok(()),
        Err(e) => Err(FrontmatterError::TomlParseError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::Error;

    #[test]
    fn test_json_parse_error() {
        let json_data = "{ invalid json }";
        let result: Result<serde_json::Value, _> =
            serde_json::from_str(json_data);
        assert!(result.is_err());
        let error =
            FrontmatterError::JsonParseError(result.unwrap_err());
        assert!(matches!(error, FrontmatterError::JsonParseError(_)));
    }

    #[test]
    fn test_toml_parse_error() {
        let toml_data = "invalid toml data";
        let result: Result<toml::Value, _> = toml::from_str(toml_data);
        assert!(result.is_err());
        let error =
            FrontmatterError::TomlParseError(result.unwrap_err());
        assert!(matches!(error, FrontmatterError::TomlParseError(_)));
    }

    #[test]
    fn test_yaml_parse_error() {
        let yaml_data = "invalid: yaml: data";
        let result: Result<serde_yml::Value, _> =
            serde_yml::from_str(yaml_data);
        assert!(result.is_err());
        let error = FrontmatterError::YamlParseError {
            source: result.unwrap_err(),
        };
        assert!(matches!(
            error,
            FrontmatterError::YamlParseError { .. }
        ));
    }

    #[test]
    fn test_conversion_error_message() {
        let error_message = "Conversion failed";
        let error = FrontmatterError::ConversionError(
            error_message.to_string(),
        );
        assert!(matches!(error, FrontmatterError::ConversionError(_)));
        assert_eq!(
            error.to_string(),
            "Failed to convert frontmatter: Conversion failed"
        );
    }

    #[test]
    fn test_parse_error_message() {
        let error_message = "Failed to parse frontmatter";
        let error =
            FrontmatterError::ParseError(error_message.to_string());
        assert!(matches!(error, FrontmatterError::ParseError(_)));
        assert_eq!(
            error.to_string(),
            "Failed to parse frontmatter: Failed to parse frontmatter"
        );
    }

    #[test]
    fn test_generic_parse_error() {
        let error =
            FrontmatterError::generic_parse_error("Parsing failed");
        match error {
            FrontmatterError::ParseError(msg) => {
                assert_eq!(msg, "Parsing failed")
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_unsupported_format_error() {
        let error = FrontmatterError::unsupported_format(10);
        match error {
            FrontmatterError::UnsupportedFormat { line } => {
                assert_eq!(line, 10)
            }
            _ => panic!("Expected UnsupportedFormat"),
        }
    }

    #[test]
    fn test_clone_implementation() {
        let original =
            FrontmatterError::ConversionError("Test error".to_string());
        let cloned = original.clone();
        if let FrontmatterError::ConversionError(msg) = cloned {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected ConversionError");
        }

        let original = FrontmatterError::UnsupportedFormat { line: 42 };
        let cloned = original.clone();
        if let FrontmatterError::UnsupportedFormat { line } = cloned {
            assert_eq!(line, 42);
        } else {
            panic!("Expected UnsupportedFormat");
        }
    }

    #[test]
    fn test_invalid_format_error() {
        let error = FrontmatterError::InvalidFormat;
        assert!(matches!(error, FrontmatterError::InvalidFormat));
    }

    #[test]
    fn test_conversion_error() {
        let error = FrontmatterError::ConversionError(
            "Test conversion error".to_string(),
        );
        assert!(matches!(error, FrontmatterError::ConversionError(_)));
    }

    #[test]
    fn test_no_frontmatter_found_error() {
        let error = FrontmatterError::NoFrontmatterFound;
        assert!(matches!(error, FrontmatterError::NoFrontmatterFound));
    }

    #[test]
    fn test_invalid_json_error() {
        let error = FrontmatterError::InvalidJson;
        assert!(matches!(error, FrontmatterError::InvalidJson));
    }

    #[test]
    fn test_invalid_toml_error() {
        let error = FrontmatterError::InvalidToml;
        assert!(matches!(error, FrontmatterError::InvalidToml));
    }

    #[test]
    fn test_invalid_yaml_error() {
        let error = FrontmatterError::InvalidYaml;
        assert!(matches!(error, FrontmatterError::InvalidYaml));
    }

    #[test]
    fn test_json_depth_limit_exceeded_error() {
        let error = FrontmatterError::JsonDepthLimitExceeded;
        assert!(matches!(
            error,
            FrontmatterError::JsonDepthLimitExceeded
        ));
    }

    #[test]
    fn test_extraction_error() {
        let error = FrontmatterError::ExtractionError(
            "Test extraction error".to_string(),
        );
        assert!(matches!(error, FrontmatterError::ExtractionError(_)));
    }

    #[test]
    fn test_error_messages() {
        assert_eq!(
            FrontmatterError::InvalidFormat.to_string(),
            "Invalid frontmatter format"
        );
        assert_eq!(
            FrontmatterError::NoFrontmatterFound.to_string(),
            "No frontmatter found in the content"
        );
        assert_eq!(
            FrontmatterError::JsonDepthLimitExceeded.to_string(),
            "JSON frontmatter exceeds maximum nesting depth"
        );
    }

    #[test]
    fn test_example_usage() {
        let result = example_usage();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FrontmatterError::TomlParseError(_)
        ));
    }

    #[test]
    fn test_clone_fallback_yaml_parse_error() {
        let original = FrontmatterError::YamlParseError {
            source: YamlError::custom("invalid yaml"),
        };
        let cloned = original.clone();
        assert!(matches!(cloned, FrontmatterError::InvalidFormat));
    }

    #[test]
    fn test_clone_fallback_json_parse_error() {
        let original = FrontmatterError::JsonParseError(
            serde_json::from_str::<serde_json::Value>("invalid json")
                .unwrap_err(),
        );
        let cloned = original.clone();
        assert!(matches!(cloned, FrontmatterError::InvalidFormat));
    }

    #[test]
    fn test_unsupported_format_with_edge_cases() {
        let error = FrontmatterError::unsupported_format(0);
        if let FrontmatterError::UnsupportedFormat { line } = error {
            assert_eq!(line, 0);
        } else {
            panic!("Expected UnsupportedFormat with line 0");
        }

        let error = FrontmatterError::unsupported_format(usize::MAX);
        if let FrontmatterError::UnsupportedFormat { line } = error {
            assert_eq!(line, usize::MAX);
        } else {
            panic!(
                "Expected UnsupportedFormat with maximum line number"
            );
        }
    }

    #[test]
    fn test_no_frontmatter_fallback() {
        // Simulate a case where no frontmatter is found
        let _content = "Some content without frontmatter";
        let result: Result<(), FrontmatterError> =
            Err(FrontmatterError::NoFrontmatterFound);

        assert!(matches!(
            result.unwrap_err(),
            FrontmatterError::NoFrontmatterFound
        ));
    }

    #[test]
    fn test_json_depth_limit_exceeded() {
        let error = FrontmatterError::JsonDepthLimitExceeded;
        assert_eq!(
            error.to_string(),
            "JSON frontmatter exceeds maximum nesting depth"
        );
    }
}
