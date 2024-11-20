//! Error handling for the frontmatter-gen crate.
//!
//! This module provides a comprehensive set of error types to handle various
//! failure scenarios that may occur during frontmatter parsing, conversion,
//! and extraction operations. Each error variant includes detailed error
//! messages and context to aid in debugging and error handling.
//!
//! # Error Handling Strategies
//!
//! The error system provides several ways to handle errors:
//!
//! - **Context-aware errors**: Use `ErrorContext` to add line/column information
//! - **Categorised errors**: Group errors by type using `ErrorCategory`
//! - **Error conversion**: Convert from standard errors using `From` implementations
//! - **Rich error messages**: Detailed error descriptions with context
//!
//! # Features
//!
//! - Type-safe error handling with descriptive messages
//! - Support for YAML, TOML, and JSON parsing errors
//! - Content validation errors with size and depth checks
//! - Format-specific error handling
//! - Extraction and conversion error handling
//!
//! # Examples
//!
//! ```
//! use frontmatter_gen::error::FrontmatterError;
//!
//! fn example() -> Result<(), FrontmatterError> {
//!     // Example of handling YAML parsing errors
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

/// Provides additional context for frontmatter errors.
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Line number where the error occurred
    pub line: Option<usize>,
    /// Column number where the error occurred
    pub column: Option<usize>,
    /// Snippet of the content where the error occurred
    pub snippet: Option<String>,
}

/// Represents errors that can occur during frontmatter operations.
///
/// This enumeration uses the `thiserror` crate to provide structured error
/// messages, improving the ease of debugging and handling errors encountered
/// in frontmatter processing.
///
/// Each variant represents a specific type of error that may occur during
/// frontmatter operations, with appropriate context and error details.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FrontmatterError {
    /// Content exceeds the maximum allowed size.
    ///
    /// This error occurs when the content size is larger than the configured
    /// maximum limit.
    ///
    /// # Fields
    ///
    /// * `size` - The actual size of the content
    /// * `max` - The maximum allowed size
    #[error("Content size {size} exceeds maximum allowed size of {max} bytes")]
    ContentTooLarge {
        /// The actual size of the content
        size: usize,
        /// The maximum allowed size
        max: usize,
    },

    /// Nesting depth exceeds the maximum allowed.
    ///
    /// This error occurs when the structure's nesting depth is greater than
    /// the configured maximum depth.
    #[error(
        "Nesting depth {depth} exceeds maximum allowed depth of {max}"
    )]
    NestingTooDeep {
        /// The actual nesting depth
        depth: usize,
        /// The maximum allowed depth
        max: usize,
    },

    /// Error occurred whilst parsing YAML content.
    ///
    /// This error occurs when the YAML parser encounters invalid syntax or
    /// structure.
    #[error("Failed to parse YAML: {source}")]
    YamlParseError {
        /// The original error from the YAML parser
        source: YamlError,
    },

    /// Error occurred whilst parsing TOML content.
    ///
    /// This error occurs when the TOML parser encounters invalid syntax or
    /// structure.
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),

    /// Error occurred whilst parsing JSON content.
    ///
    /// This error occurs when the JSON parser encounters invalid syntax or
    /// structure.
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] JsonError),

    /// The frontmatter format is invalid or unsupported.
    ///
    /// This error occurs when the frontmatter format cannot be determined or
    /// is not supported by the library.
    #[error("Invalid frontmatter format")]
    InvalidFormat,

    /// Error occurred during conversion between formats.
    ///
    /// This error occurs when converting frontmatter from one format to another
    /// fails.
    #[error("Failed to convert frontmatter: {0}")]
    ConversionError(String),

    /// Generic error during parsing.
    ///
    /// This error occurs when a parsing operation fails with a generic error.
    #[error("Failed to parse frontmatter: {0}")]
    ParseError(String),

    /// Unsupported or unknown frontmatter format was detected.
    ///
    /// This error occurs when an unsupported frontmatter format is encountered
    /// at a specific line.
    #[error("Unsupported frontmatter format detected at line {line}")]
    UnsupportedFormat {
        /// The line number where the unsupported format was encountered
        line: usize,
    },

    /// No frontmatter content was found.
    ///
    /// This error occurs when attempting to extract frontmatter from content
    /// that does not contain any frontmatter section.
    #[error("No frontmatter found in the content")]
    NoFrontmatterFound,

    /// Invalid JSON frontmatter.
    ///
    /// This error occurs when the JSON frontmatter is malformed or invalid.
    #[error("Invalid JSON frontmatter")]
    InvalidJson,

    /// Invalid TOML frontmatter.
    ///
    /// This error occurs when the TOML frontmatter is malformed or invalid.
    #[error("Invalid TOML frontmatter")]
    InvalidToml,

    /// Invalid YAML frontmatter.
    ///
    /// This error occurs when the YAML frontmatter is malformed or invalid.
    #[error("Invalid YAML frontmatter")]
    InvalidYaml,

    /// Invalid URL format.
    ///
    /// This error occurs when an invalid URL is encountered in the frontmatter.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Invalid language code.
    ///
    /// This error occurs when an invalid language code is encountered in the
    /// frontmatter.
    #[error("Invalid language code: {0}")]
    InvalidLanguage(String),

    /// JSON frontmatter exceeds maximum nesting depth.
    ///
    /// This error occurs when the JSON frontmatter structure exceeds the
    /// maximum allowed nesting depth.
    #[error("JSON frontmatter exceeds maximum nesting depth")]
    JsonDepthLimitExceeded,

    /// Error during frontmatter extraction.
    ///
    /// This error occurs when there is a problem extracting frontmatter from
    /// the content.
    #[error("Extraction error: {0}")]
    ExtractionError(String),

    /// Input validation error.
    ///
    /// This error occurs when the input fails validation checks.
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

/// Categories of frontmatter errors.
///
/// This enumeration defines the main categories of errors that can occur
/// during frontmatter operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorCategory {
    /// Parsing-related errors
    Parsing,
    /// Validation-related errors
    Validation,
    /// Conversion-related errors
    Conversion,
    /// Configuration-related errors
    Configuration,
}

impl FrontmatterError {
    /// Returns the category of the error.
    ///
    /// # Returns
    ///
    /// Returns the `ErrorCategory` that best describes this error.
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::YamlParseError { .. }
            | Self::TomlParseError(_)
            | Self::JsonParseError(_)
            | Self::ParseError(_) => ErrorCategory::Parsing,
            Self::ValidationError(_) => ErrorCategory::Validation,
            Self::ConversionError(_) => ErrorCategory::Conversion,
            Self::ContentTooLarge { .. }
            | Self::NestingTooDeep { .. } => {
                ErrorCategory::Configuration
            }
            _ => ErrorCategory::Parsing,
        }
    }

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

    /// Adds context to an error.
    ///
    /// # Arguments
    ///
    /// * `context` - Additional context information about the error
    ///
    /// # Examples
    ///
    /// ```
    /// use frontmatter_gen::error::{FrontmatterError, ErrorContext};
    ///
    /// let context = ErrorContext {
    ///     line: Some(42),
    ///     column: Some(10),
    ///     snippet: Some("invalid content".to_string()),
    /// };
    ///
    /// let error = FrontmatterError::ParseError("Invalid syntax".to_string())
    ///     .with_context(context);
    /// ```
    pub fn with_context(self, context: ErrorContext) -> Self {
        match self {
            Self::ParseError(msg) => {
                let mut formatted_message = format!(
                    "{} (line: {}, column: {})",
                    msg,
                    context.line.unwrap_or(0),
                    context.column.unwrap_or(0)
                );
                if let Some(snippet) = &context.snippet {
                    formatted_message
                        .push_str(&format!(" near '{}'", snippet));
                }
                Self::ParseError(formatted_message)
            }
            _ => self,
        }
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

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "at {}:{}",
            self.line.map_or("unknown".to_string(), |l| l.to_string()),
            self.column
                .map_or("unknown".to_string(), |c| c.to_string())
        )?;
        if let Some(snippet) = &self.snippet {
            write!(f, " near '{}'", snippet)?;
        }
        Ok(())
    }
}

// Common conversions -- add this after your existing From implementations

/// Converts an `EngineError` into a `FrontmatterError`
///
/// This allows engine errors to be converted into frontmatter errors when needed,
/// preserving the error context and message.
///
/// # Examples
///
/// ```
/// use frontmatter_gen::error::{EngineError, FrontmatterError};
/// use std::io;
///
/// let engine_error = EngineError::ContentError("content processing failed".to_string());
/// let frontmatter_error: FrontmatterError = engine_error.into();
/// assert!(matches!(frontmatter_error, FrontmatterError::ParseError(_)));
/// ```
impl From<EngineError> for FrontmatterError {
    fn from(err: EngineError) -> Self {
        match err {
            EngineError::ContentError(msg) => {
                Self::ParseError(format!("Content error: {}", msg))
            }
            EngineError::TemplateError(msg) => {
                Self::ParseError(format!("Template error: {}", msg))
            }
            EngineError::AssetError(msg) => {
                Self::ParseError(format!("Asset error: {}", msg))
            }
            EngineError::FileSystemError { source } => {
                Self::ParseError(format!(
                    "File system error: {}",
                    source
                ))
            }
            EngineError::MetadataError(msg) => {
                Self::ParseError(format!("Metadata error: {}", msg))
            }
        }
    }
}

impl Clone for EngineError {
    fn clone(&self) -> Self {
        match self {
            Self::ContentError(msg) => Self::ContentError(msg.clone()),
            Self::TemplateError(msg) => {
                Self::TemplateError(msg.clone())
            }
            Self::AssetError(msg) => Self::AssetError(msg.clone()),
            Self::FileSystemError { source } => Self::FileSystemError {
                source: std::io::Error::new(
                    source.kind(),
                    source.to_string(),
                ),
            },
            Self::MetadataError(msg) => {
                Self::MetadataError(msg.clone())
            }
        }
    }
}

// Common conversions
impl From<std::io::Error> for FrontmatterError {
    fn from(err: std::io::Error) -> Self {
        Self::ParseError(err.to_string())
    }
}

impl From<FrontmatterError> for String {
    fn from(err: FrontmatterError) -> String {
        err.to_string()
    }
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

        #[test]
        fn test_toml_parse_error() {
            let toml_data = "invalid = toml";
            let result: Result<toml::Value, _> =
                toml::from_str(toml_data);
            assert!(result.is_err());
            let error =
                FrontmatterError::TomlParseError(result.unwrap_err());
            assert!(matches!(
                error,
                FrontmatterError::TomlParseError(_)
            ));
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
        use super::*;

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

        #[test]
        fn test_error_with_context() {
            let context = ErrorContext {
                line: Some(42),
                column: Some(10),
                snippet: Some("invalid syntax".to_string()),
            };

            let error = FrontmatterError::ParseError(
                "Parse failed".to_string(),
            )
            .with_context(context);

            assert!(error.to_string().contains("line: 42"));
            assert!(error.to_string().contains("column: 10"));
        }

        #[test]
        fn test_engine_error_clone() {
            let original =
                EngineError::ContentError("test error".to_string());
            let cloned = original.clone();
            assert_eq!(cloned.to_string(), original.to_string());
        }

        #[test]
        fn test_from_io_error() {
            let io_error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            );
            let frontmatter_error = FrontmatterError::from(io_error);
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
        }

        #[test]
        fn test_error_context_display() {
            let context = ErrorContext {
                line: Some(42),
                column: Some(10),
                snippet: Some("invalid syntax".to_string()),
            };
            assert_eq!(
                context.to_string(),
                "at 42:10 near 'invalid syntax'"
            );

            let partial_context = ErrorContext {
                line: Some(42),
                column: None,
                snippet: None,
            };
            assert_eq!(partial_context.to_string(), "at 42:unknown");
        }

        #[test]
        fn test_engine_error_conversion() {
            // Test content error conversion
            let engine_error =
                EngineError::ContentError("test error".to_string());
            let frontmatter_error: FrontmatterError =
                engine_error.into();
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
            assert!(frontmatter_error
                .to_string()
                .contains("Content error: test error"));

            // Test filesystem error conversion
            let io_error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not found",
            );
            let engine_error =
                EngineError::FileSystemError { source: io_error };
            let frontmatter_error: FrontmatterError =
                engine_error.into();
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
            assert!(frontmatter_error
                .to_string()
                .contains("File system error"));

            // Test metadata error conversion
            let engine_error = EngineError::MetadataError(
                "metadata error".to_string(),
            );
            let frontmatter_error: FrontmatterError =
                engine_error.into();
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
            assert!(frontmatter_error
                .to_string()
                .contains("Metadata error"));
        }

        #[test]
        fn test_clone_yaml_parse_error() {
            let yaml_data = "invalid: : yaml";
            let result: Result<serde_yml::Value, _> =
                serde_yml::from_str(yaml_data);
            assert!(result.is_err());
            let original = FrontmatterError::YamlParseError {
                source: result.unwrap_err(),
            };
            let cloned = original.clone();
            // Since YamlParseError clones to InvalidFormat
            assert!(matches!(cloned, FrontmatterError::InvalidFormat));
        }

        #[test]
        fn test_clone_json_parse_error() {
            let json_data = "{ invalid json }";
            let result: Result<serde_json::Value, _> =
                serde_json::from_str(json_data);
            assert!(result.is_err());
            let original =
                FrontmatterError::JsonParseError(result.unwrap_err());
            let cloned = original.clone();
            // Since JsonParseError clones to InvalidFormat
            assert!(matches!(cloned, FrontmatterError::InvalidFormat));
        }
    }

    /// Tests for the `category` method of FrontmatterError
    mod category_tests {
        use super::*;

        #[test]
        fn test_error_category() {
            // Parsing category
            let yaml_error = serde_yml::from_str::<serde_yml::Value>(
                "invalid: : yaml",
            )
            .unwrap_err();
            let toml_error =
                toml::from_str::<toml::Value>("invalid = toml")
                    .unwrap_err();
            let json_error = serde_json::from_str::<serde_json::Value>(
                "{ invalid json }",
            )
            .unwrap_err();

            let errors = vec![
                FrontmatterError::YamlParseError { source: yaml_error },
                FrontmatterError::TomlParseError(toml_error),
                FrontmatterError::JsonParseError(json_error),
                FrontmatterError::ParseError("test error".to_string()),
                FrontmatterError::InvalidFormat,
                FrontmatterError::UnsupportedFormat { line: 1 },
                FrontmatterError::NoFrontmatterFound,
                FrontmatterError::InvalidJson,
                FrontmatterError::InvalidToml,
                FrontmatterError::InvalidYaml,
                FrontmatterError::JsonDepthLimitExceeded,
                FrontmatterError::ExtractionError(
                    "test error".to_string(),
                ),
                FrontmatterError::InvalidUrl("test url".to_string()),
                FrontmatterError::InvalidLanguage(
                    "test lang".to_string(),
                ),
            ];

            for error in errors {
                assert_eq!(
                    error.category(),
                    ErrorCategory::Parsing,
                    "Error {:?} should have category Parsing",
                    error
                );
            }

            // Validation category
            let error = FrontmatterError::ValidationError(
                "test error".to_string(),
            );
            assert_eq!(error.category(), ErrorCategory::Validation);

            // Conversion category
            let error = FrontmatterError::ConversionError(
                "test error".to_string(),
            );
            assert_eq!(error.category(), ErrorCategory::Conversion);

            // Configuration category
            let errors = vec![
                FrontmatterError::ContentTooLarge {
                    size: 1000,
                    max: 500,
                },
                FrontmatterError::NestingTooDeep { depth: 10, max: 5 },
            ];
            for error in errors {
                assert_eq!(
                    error.category(),
                    ErrorCategory::Configuration,
                    "Error {:?} should have category Configuration",
                    error
                );
            }
        }
    }

    /// Tests for converting EngineError variants into FrontmatterError
    mod engine_error_conversion_tests {
        use super::*;

        #[test]
        fn test_engine_error_conversion_template_error() {
            let engine_error = EngineError::TemplateError(
                "template processing failed".to_string(),
            );
            let frontmatter_error: FrontmatterError =
                engine_error.into();
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
            assert!(frontmatter_error.to_string().contains(
                "Template error: template processing failed"
            ));
        }

        #[test]
        fn test_engine_error_conversion_asset_error() {
            let engine_error = EngineError::AssetError(
                "asset processing failed".to_string(),
            );
            let frontmatter_error: FrontmatterError =
                engine_error.into();
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
            assert!(frontmatter_error
                .to_string()
                .contains("Asset error: asset processing failed"));
        }
    }

    // Additional tests to cover remaining lines and edge cases
    #[cfg(test)]
    mod additional_tests {
        use super::*;

        #[test]
        fn test_with_context_non_parse_error() {
            let context = ErrorContext {
                line: Some(10),
                column: Some(5),
                snippet: Some("example snippet".to_string()),
            };

            let error = FrontmatterError::ValidationError(
                "invalid input".to_string(),
            );
            let modified_error = error.clone().with_context(context);
            // `with_context` should not modify non-parse errors.
            assert_eq!(modified_error.to_string(), error.to_string());
        }

        #[test]
        fn test_error_context_display_edge_cases() {
            let missing_line_column = ErrorContext {
                line: None,
                column: None,
                snippet: Some("snippet only".to_string()),
            };
            assert_eq!(
                missing_line_column.to_string(),
                "at unknown:unknown near 'snippet only'"
            );

            let missing_snippet = ErrorContext {
                line: Some(3),
                column: Some(15),
                snippet: None,
            };
            assert_eq!(missing_snippet.to_string(), "at 3:15");

            let missing_all = ErrorContext {
                line: None,
                column: None,
                snippet: None,
            };
            assert_eq!(missing_all.to_string(), "at unknown:unknown");
        }

        #[test]
        fn test_default_category() {
            let error = FrontmatterError::InvalidJson;
            assert_eq!(error.category(), ErrorCategory::Parsing);

            let unsupported_error =
                FrontmatterError::UnsupportedFormat { line: 42 };
            assert_eq!(
                unsupported_error.category(),
                ErrorCategory::Parsing
            );
        }

        #[test]
        fn test_io_error_conversion() {
            let io_error = std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "access denied",
            );
            let frontmatter_error = FrontmatterError::from(io_error);
            assert!(matches!(
                frontmatter_error,
                FrontmatterError::ParseError(_)
            ));
            assert!(frontmatter_error
                .to_string()
                .contains("access denied"));
        }

        #[test]
        fn test_frontmatter_error_to_string_conversion() {
            let error = FrontmatterError::InvalidFormat;
            let error_string: String = error.into();
            assert_eq!(error_string, "Invalid frontmatter format");
        }

        #[test]
        fn test_all_error_variants() {
            let large_error = FrontmatterError::ContentTooLarge {
                size: 12345,
                max: 10000,
            };
            assert_eq!(
                large_error.to_string(),
                "Content size 12345 exceeds maximum allowed size of 10000 bytes"
            );

            let nesting_error =
                FrontmatterError::NestingTooDeep { depth: 20, max: 10 };
            assert_eq!(
                nesting_error.to_string(),
                "Nesting depth 20 exceeds maximum allowed depth of 10"
            );

            let unsupported_format =
                FrontmatterError::UnsupportedFormat { line: 99 };
            assert_eq!(
                unsupported_format.to_string(),
                "Unsupported frontmatter format detected at line 99"
            );

            let no_frontmatter = FrontmatterError::NoFrontmatterFound;
            assert_eq!(
                no_frontmatter.to_string(),
                "No frontmatter found in the content"
            );

            let invalid_url = FrontmatterError::InvalidUrl(
                "http://invalid-url".to_string(),
            );
            assert_eq!(
                invalid_url.to_string(),
                "Invalid URL: http://invalid-url"
            );

            let invalid_language =
                FrontmatterError::InvalidLanguage("xx".to_string());
            assert_eq!(
                invalid_language.to_string(),
                "Invalid language code: xx"
            );

            let json_depth_limit =
                FrontmatterError::JsonDepthLimitExceeded;
            assert_eq!(
                json_depth_limit.to_string(),
                "JSON frontmatter exceeds maximum nesting depth"
            );
        }

        #[test]
        fn test_generic_parse_error_with_context() {
            let context = ErrorContext {
                line: Some(5),
                column: Some(20),
                snippet: Some("unexpected token".to_string()),
            };
            let error = FrontmatterError::generic_parse_error(
                "Unexpected error",
            )
            .with_context(context);
            assert!(error.to_string().contains("line: 5"));
            assert!(error.to_string().contains("column: 20"));
            assert!(error.to_string().contains("unexpected token"));
        }

        #[test]
        fn test_category_fallback() {
            let unknown_error = FrontmatterError::InvalidYaml; // Any untested error
            assert_eq!(
                unknown_error.category(),
                ErrorCategory::Parsing
            ); // Default fallback for unlisted errors
        }
    }
}
