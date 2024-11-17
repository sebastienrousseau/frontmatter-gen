//! Error handling for the frontmatter-gen crate.
//!
//! This module provides a comprehensive set of error types to handle various
//! failure scenarios that may occur during frontmatter parsing, conversion,
//! and extraction. Each variant includes detailed error messages to aid in
//! debugging and improve error handling.
//!
//! # Examples
//!
//! ```
//! use frontmatter_gen::error::FrontmatterError;
//!
//! fn example() -> Result<(), FrontmatterError> {
//!     let invalid_yaml = "invalid: : yaml";
//!     match serde_yml::from_str::<serde_yml::Value>(invalid_yaml) {
//!         Ok(_) => Ok(()),
//!         Err(e) => Err(FrontmatterError::YamlParseError { source: e }),
//!     }
//! }
//! ```

use serde_json::Error as JsonError;
use serde_yml::Error as YamlError;
use thiserror::Error;

/// Represents errors that can occur during frontmatter operations.
///
/// This enum uses the `thiserror` crate to provide structured error messages,
/// improving the ease of debugging and handling errors encountered in
/// frontmatter processing.
#[derive(Error, Debug)]
pub enum FrontmatterError {
    /// Content exceeds the maximum allowed size
    #[error("Content size {size} exceeds maximum allowed size of {max} bytes")]
    ContentTooLarge {
        /// The actual size of the content
        size: usize,
        /// The maximum allowed size
        max: usize,
    },

    /// Nesting depth exceeds the maximum allowed
    #[error(
        "Nesting depth {depth} exceeds maximum allowed depth of {max}"
    )]
    NestingTooDeep {
        /// The actual nesting depth
        depth: usize,
        /// The maximum allowed depth
        max: usize,
    },

    /// Error occurred while parsing YAML content
    #[error("Failed to parse YAML: {source}")]
    YamlParseError {
        /// The original error from the YAML parser
        source: YamlError,
    },

    /// Error occurred while parsing TOML content
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),

    /// Error occurred while parsing JSON content
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] JsonError),

    /// The frontmatter format is invalid or unsupported
    #[error("Invalid frontmatter format")]
    InvalidFormat,

    /// Error occurred during conversion between formats
    #[error("Failed to convert frontmatter: {0}")]
    ConversionError(String),

    /// Generic error during parsing
    #[error("Failed to parse frontmatter: {0}")]
    ParseError(String),

    /// Unsupported or unknown frontmatter format was detected
    #[error("Unsupported frontmatter format detected at line {line}")]
    UnsupportedFormat {
        /// The line number where the unsupported format was encountered
        line: usize,
    },

    /// No frontmatter content was found
    #[error("No frontmatter found in the content")]
    NoFrontmatterFound,

    /// Invalid JSON frontmatter
    #[error("Invalid JSON frontmatter")]
    InvalidJson,

    /// Invalid TOML frontmatter
    #[error("Invalid TOML frontmatter")]
    InvalidToml,

    /// Invalid YAML frontmatter
    #[error("Invalid YAML frontmatter")]
    InvalidYaml,

    /// Invalid URL format
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Invalid language code
    #[error("Invalid language code: {0}")]
    InvalidLanguage(String),

    /// JSON frontmatter exceeds maximum nesting depth
    #[error("JSON frontmatter exceeds maximum nesting depth")]
    JsonDepthLimitExceeded,

    /// Error during frontmatter extraction
    #[error("Extraction error: {0}")]
    ExtractionError(String),

    /// Input validation error
    #[error("Input validation error: {0}")]
    ValidationError(String),
}

impl Clone for FrontmatterError {
    fn clone(&self) -> Self {
        match self {
            Self::ContentTooLarge { size, max } => {
                Self::ContentTooLarge {
                    size: *size,
                    max: *max,
                }
            }
            Self::NestingTooDeep { depth, max } => {
                Self::NestingTooDeep {
                    depth: *depth,
                    max: *max,
                }
            }
            Self::YamlParseError { .. } => Self::InvalidFormat,
            Self::TomlParseError(e) => Self::TomlParseError(e.clone()),
            Self::JsonParseError(_) => Self::InvalidFormat,
            Self::InvalidFormat => Self::InvalidFormat,
            Self::ConversionError(msg) => {
                Self::ConversionError(msg.clone())
            }
            Self::ParseError(msg) => Self::ParseError(msg.clone()),
            Self::UnsupportedFormat { line } => {
                Self::UnsupportedFormat { line: *line }
            }
            Self::NoFrontmatterFound => Self::NoFrontmatterFound,
            Self::InvalidJson => Self::InvalidJson,
            Self::InvalidToml => Self::InvalidToml,
            Self::InvalidYaml => Self::InvalidYaml,
            Self::JsonDepthLimitExceeded => {
                Self::JsonDepthLimitExceeded
            }
            Self::ExtractionError(msg) => {
                Self::ExtractionError(msg.clone())
            }
            Self::ValidationError(msg) => {
                Self::ValidationError(msg.clone())
            }
            Self::InvalidUrl(msg) => Self::InvalidUrl(msg.clone()),
            Self::InvalidLanguage(msg) => {
                Self::InvalidLanguage(msg.clone())
            }
        }
    }
}

impl FrontmatterError {
    /// Creates a generic parse error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice containing the error message
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::error::FrontmatterError;
    ///
    /// let error = FrontmatterError::generic_parse_error("Invalid syntax");
    /// assert!(matches!(error, FrontmatterError::ParseError(_)));
    /// ```
    #[must_use]
    pub fn generic_parse_error(message: &str) -> Self {
        Self::ParseError(message.to_string())
    }

    /// Creates an unsupported format error for a specific line.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number where the unsupported format was detected
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::error::FrontmatterError;
    ///
    /// let error = FrontmatterError::unsupported_format(42);
    /// assert!(matches!(error, FrontmatterError::UnsupportedFormat { line: 42 }));
    /// ```
    #[must_use]
    pub fn unsupported_format(line: usize) -> Self {
        Self::UnsupportedFormat { line }
    }

    /// Creates a validation error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice containing the validation error message
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::error::FrontmatterError;
    ///
    /// let error = FrontmatterError::validation_error("Invalid character in title");
    /// assert!(matches!(error, FrontmatterError::ValidationError(_)));
    /// ```
    #[must_use]
    pub fn validation_error(message: &str) -> Self {
        Self::ValidationError(message.to_string())
    }
}

/// Errors that can occur during site generation
#[derive(Error, Debug)]
pub enum EngineError {
    /// Error occurred during content processing
    #[error("Content processing error: {0}")]
    ContentError(String),

    /// Error occurred during template processing
    #[error("Template processing error: {0}")]
    TemplateError(String),

    /// Error occurred during asset processing
    #[error("Asset processing error: {0}")]
    AssetError(String),

    /// Error occurred during file system operations
    #[error("File system error: {source}")]
    FileSystemError {
        #[from]
        /// The underlying IO error
        source: std::io::Error,
    },

    /// Error occurred during metadata processing
    #[error("Metadata error: {0}")]
    MetadataError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for FrontmatterError
    mod frontmatter_error {
        use super::*;

        #[test]
        fn test_content_too_large_error() {
            let error = FrontmatterError::ContentTooLarge {
                size: 1000,
                max: 500,
            };
            assert!(error
                .to_string()
                .contains("Content size 1000 exceeds maximum"));
        }

        #[test]
        fn test_nesting_too_deep_error() {
            let error =
                FrontmatterError::NestingTooDeep { depth: 10, max: 5 };
            assert!(error
                .to_string()
                .contains("Nesting depth 10 exceeds maximum"));
        }

        #[test]
        fn test_json_parse_error() {
            let json_data = "{ invalid json }";
            let result: Result<serde_json::Value, _> =
                serde_json::from_str(json_data);
            assert!(result.is_err());
            let error =
                FrontmatterError::JsonParseError(result.unwrap_err());
            assert!(matches!(
                error,
                FrontmatterError::JsonParseError(_)
            ));
        }

        #[test]
        fn test_yaml_parse_error() {
            let yaml_data = "invalid: : yaml";
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
        fn test_validation_error() {
            let error = FrontmatterError::validation_error(
                "Test validation error",
            );
            assert!(matches!(
                error,
                FrontmatterError::ValidationError(_)
            ));
            assert_eq!(
                error.to_string(),
                "Input validation error: Test validation error"
            );
        }

        #[test]
        fn test_generic_parse_error() {
            let error = FrontmatterError::generic_parse_error(
                "Test parse error",
            );
            assert!(matches!(error, FrontmatterError::ParseError(_)));
            assert_eq!(
                error.to_string(),
                "Failed to parse frontmatter: Test parse error"
            );
        }

        #[test]
        fn test_unsupported_format_error() {
            let error = FrontmatterError::unsupported_format(42);
            assert!(matches!(
                error,
                FrontmatterError::UnsupportedFormat { line: 42 }
            ));
            assert_eq!(
                error.to_string(),
                "Unsupported frontmatter format detected at line 42"
            );
        }

        #[test]
        fn test_clone_implementation() {
            let original = FrontmatterError::ContentTooLarge {
                size: 1000,
                max: 500,
            };
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::ContentTooLarge {
                    size: 1000,
                    max: 500
                }
            ));

            let original =
                FrontmatterError::NestingTooDeep { depth: 10, max: 5 };
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::NestingTooDeep { depth: 10, max: 5 }
            ));
        }

        #[test]
        fn test_error_display() {
            let error = FrontmatterError::ContentTooLarge {
                size: 1000,
                max: 500,
            };
            assert_eq!(
                error.to_string(),
                "Content size 1000 exceeds maximum allowed size of 500 bytes"
            );

            let error = FrontmatterError::ValidationError(
                "Invalid input".to_string(),
            );
            assert_eq!(
                error.to_string(),
                "Input validation error: Invalid input"
            );
        }
    }

    /// Tests for EngineError
    mod engine_error {
        use super::*;
        use std::io;

        #[test]
        fn test_content_error() {
            let error =
                EngineError::ContentError("Content issue".to_string());
            assert!(matches!(error, EngineError::ContentError(_)));
            assert_eq!(
                error.to_string(),
                "Content processing error: Content issue"
            );
        }

        #[test]
        fn test_template_error() {
            let error = EngineError::TemplateError(
                "Template issue".to_string(),
            );
            assert!(matches!(error, EngineError::TemplateError(_)));
            assert_eq!(
                error.to_string(),
                "Template processing error: Template issue"
            );
        }

        #[test]
        fn test_asset_error() {
            let error =
                EngineError::AssetError("Asset issue".to_string());
            assert!(matches!(error, EngineError::AssetError(_)));
            assert_eq!(
                error.to_string(),
                "Asset processing error: Asset issue"
            );
        }

        #[test]
        fn test_filesystem_error() {
            let io_error =
                io::Error::new(io::ErrorKind::Other, "IO failure");
            let error =
                EngineError::FileSystemError { source: io_error };
            assert!(matches!(
                error,
                EngineError::FileSystemError { .. }
            ));
            assert_eq!(
                error.to_string(),
                "File system error: IO failure"
            );
        }

        #[test]
        fn test_metadata_error() {
            let error = EngineError::MetadataError(
                "Metadata issue".to_string(),
            );
            assert!(matches!(error, EngineError::MetadataError(_)));
            assert_eq!(
                error.to_string(),
                "Metadata error: Metadata issue"
            );
        }
    }

    /// Tests for the Clone implementation of `FrontmatterError`.
    mod clone_tests {
        use crate::error::FrontmatterError;

        #[test]
        fn test_clone_content_too_large() {
            let original = FrontmatterError::ContentTooLarge {
                size: 1000,
                max: 500,
            };
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::ContentTooLarge { size, max }
                if size == 1000 && max == 500
            ));
        }

        #[test]
        fn test_clone_nesting_too_deep() {
            let original =
                FrontmatterError::NestingTooDeep { depth: 10, max: 5 };
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::NestingTooDeep { depth, max }
                if depth == 10 && max == 5
            ));
        }

        #[test]
        fn test_clone_conversion_error() {
            let original = FrontmatterError::ConversionError(
                "conversion issue".to_string(),
            );
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::ConversionError(msg) if msg == "conversion issue"
            ));
        }

        #[test]
        fn test_clone_parse_error() {
            let original =
                FrontmatterError::ParseError("parse issue".to_string());
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::ParseError(msg) if msg == "parse issue"
            ));
        }

        #[test]
        fn test_clone_unsupported_format() {
            let original =
                FrontmatterError::UnsupportedFormat { line: 42 };
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::UnsupportedFormat { line } if line == 42
            ));
        }

        #[test]
        fn test_clone_no_frontmatter_found() {
            let original = FrontmatterError::NoFrontmatterFound;
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::NoFrontmatterFound
            ));
        }

        #[test]
        fn test_clone_invalid_json() {
            let original = FrontmatterError::InvalidJson;
            let cloned = original.clone();
            assert!(matches!(cloned, FrontmatterError::InvalidJson));
        }

        #[test]
        fn test_clone_invalid_toml() {
            let original = FrontmatterError::InvalidToml;
            let cloned = original.clone();
            assert!(matches!(cloned, FrontmatterError::InvalidToml));
        }

        #[test]
        fn test_clone_invalid_yaml() {
            let original = FrontmatterError::InvalidYaml;
            let cloned = original.clone();
            assert!(matches!(cloned, FrontmatterError::InvalidYaml));
        }

        #[test]
        fn test_clone_json_depth_limit_exceeded() {
            let original = FrontmatterError::JsonDepthLimitExceeded;
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::JsonDepthLimitExceeded
            ));
        }

        #[test]
        fn test_clone_extraction_error() {
            let original = FrontmatterError::ExtractionError(
                "extraction issue".to_string(),
            );
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::ExtractionError(msg) if msg == "extraction issue"
            ));
        }

        #[test]
        fn test_clone_validation_error() {
            let original = FrontmatterError::ValidationError(
                "validation issue".to_string(),
            );
            let cloned = original.clone();
            assert!(matches!(
                cloned,
                FrontmatterError::ValidationError(msg) if msg == "validation issue"
            ));
        }
    }
}
