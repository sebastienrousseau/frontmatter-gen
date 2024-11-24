//! Error handling for the frontmatter-gen crate.
//!
//! This module provides a comprehensive set of error types to handle various
//! failure scenarios that may occur during front matter parsing, conversion,
//! and extraction operations. Each error variant includes detailed error
//! messages and context to aid in debugging and error handling.
//!
//! # Error Handling Strategies
//!
//! The error system provides several ways to handle errors:
//!
//! - **Context-aware errors**: Use `Context` to add line/column information
//! - **Categorized errors**: Group errors by type using `Category`
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
//! ```rust
//! use frontmatter_gen::error::Error;
//!
//! fn example() -> Result<(), Error> {
//!     // Example of handling YAML parsing errors
//!     let invalid_yaml = "invalid: : yaml";
//!     match serde_yml::from_str::<serde_yml::Value>(invalid_yaml) {
//!         Ok(_) => Ok(()),
//!         Err(e) => Err(Error::YamlParseError { source: e.into() }),
//!     }
//! }
//! ```

use serde_json::Error as JsonError;
use serde_yml::Error as YamlError;
use std::sync::Arc;
use thiserror::Error;

/// Provides additional context for front matter errors.
#[derive(Debug, Clone)]
pub struct Context {
    /// Line number where the error occurred.
    pub line: Option<usize>,
    /// Column number where the error occurred.
    pub column: Option<usize>,
    /// Snippet of the content where the error occurred.
    pub snippet: Option<String>,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "at {}:{}",
            self.line.unwrap_or(0),
            self.column.unwrap_or(0)
        )?;
        if let Some(snippet) = &self.snippet {
            write!(f, " near '{}'", snippet)?;
        }
        Ok(())
    }
}

/// Represents errors that can occur during front matter operations.
///
/// This enumeration uses the `thiserror` crate to provide structured error
/// messages, improving the ease of debugging and handling errors encountered
/// in front matter processing.
///
/// Each variant represents a specific type of error that may occur during
/// front matter operations, with appropriate context and error details.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Content exceeds the maximum allowed size.
    ///
    /// This error occurs when the content size is larger than the configured
    /// maximum limit.
    ///
    /// # Fields
    ///
    /// * `size` - The actual size of the content
    /// * `max` - The maximum allowed size
    #[error("Your front matter contains too many fields ({size}). The maximum allowed is {max}.")]
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
        "Your front matter is nested too deeply ({depth} levels). The maximum allowed nesting depth is {max}."
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
        source: Arc<YamlError>,
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
    JsonParseError(Arc<JsonError>),

    /// The front matter format is invalid or unsupported.
    ///
    /// This error occurs when the front matter format cannot be determined or
    /// is not supported by the library.
    #[error("Invalid front matter format")]
    InvalidFormat,

    /// Error occurred during conversion between formats.
    ///
    /// This error occurs when converting front matter from one format to another
    /// fails.
    #[error("Failed to convert front matter: {0}")]
    ConversionError(String),

    /// Generic error during parsing.
    ///
    /// This error occurs when a parsing operation fails with a generic error.
    #[error("Failed to parse front matter: {0}")]
    ParseError(String),

    /// Unsupported or unknown front matter format was detected.
    ///
    /// This error occurs when an unsupported front matter format is encountered
    /// at a specific line.
    #[error("Unsupported front matter format detected at line {line}")]
    UnsupportedFormat {
        /// The line number where the unsupported format was encountered
        line: usize,
    },

    /// No front matter content was found.
    ///
    /// This error occurs when attempting to extract front matter from content
    /// that does not contain any front matter section.
    #[error("No front matter found in the content")]
    NoFrontmatterFound,

    /// Invalid JSON front matter.
    ///
    /// This error occurs when the JSON front matter is malformed or invalid.
    #[error(
        "Invalid JSON front matter: malformed or invalid structure."
    )]
    InvalidJson,

    /// Invalid URL format.
    ///
    /// This error occurs when an invalid URL is encountered in the front matter.
    #[error(
        "Invalid URL: {0}. Ensure the URL is well-formed and valid."
    )]
    InvalidUrl(String),

    /// Invalid TOML front matter.
    ///
    /// This error occurs when the TOML front matter is malformed or invalid.
    #[error(
        "Invalid TOML front matter: malformed or invalid structure."
    )]
    InvalidToml,

    /// Invalid YAML front matter.
    ///
    /// This error occurs when the YAML front matter is malformed or invalid.
    #[error(
        "Invalid YAML front matter: malformed or invalid structure."
    )]
    InvalidYaml,

    /// Invalid language code.
    ///
    /// This error occurs when an invalid language code is encountered in the
    /// front matter.
    #[error("Invalid language code: {0}")]
    InvalidLanguage(String),

    /// JSON front matter exceeds maximum nesting depth.
    ///
    /// This error occurs when the JSON front matter structure exceeds the
    /// maximum allowed nesting depth.
    #[error("JSON front matter exceeds maximum nesting depth")]
    JsonDepthLimitExceeded,

    /// Error during front matter extraction.
    ///
    /// This error occurs when there is a problem extracting front matter from
    /// the content.
    #[error("Extraction error: {0}")]
    ExtractionError(String),

    /// Serialization or deserialization error.
    ///
    /// This error occurs when there is a problem serializing or deserializing
    /// content.
    #[error("Serialization or deserialization error: {source}")]
    SerdeError {
        /// The original error from the serde library
        source: Arc<serde_json::Error>,
    },

    /// Input validation error.
    ///
    /// This error occurs when the input fails validation checks.
    #[error("Input validation error: {0}")]
    ValidationError(String),

    /// Generic error with a custom message.
    ///
    /// This error occurs when a generic error is encountered with a custom message.
    #[error("Generic error: {0}")]
    Other(String),
}

impl Clone for Error {
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
            Self::YamlParseError { source } => Self::YamlParseError {
                source: Arc::clone(source),
            },
            Self::JsonParseError(err) => {
                Self::JsonParseError(Arc::<serde_json::Error>::clone(
                    err,
                ))
            }
            Self::TomlParseError(err) => {
                Self::TomlParseError(err.clone())
            }
            Self::SerdeError { source } => Self::SerdeError {
                source: Arc::clone(source),
            },
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
            Self::Other(msg) => Self::Other(msg.clone()),
            Self::InvalidFormat => Self::InvalidFormat,
        }
    }
}

/// Categories of front matter errors.
///
/// This enumeration defines the main categories of errors that can occur
/// during front matter operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Category {
    /// Parsing-related errors.
    Parsing,
    /// Validation-related errors.
    Validation,
    /// Conversion-related errors.
    Conversion,
    /// Configuration-related errors.
    Configuration,
}

impl Error {
    /// Returns the category of the error.
    ///
    /// # Returns
    ///
    /// Returns the `Category` that best describes this error.
    #[must_use]
    pub const fn category(&self) -> Category {
        match self {
            Self::YamlParseError { .. }
            | Self::TomlParseError(_)
            | Self::JsonParseError(_)
            | Self::SerdeError { .. }
            | Self::ParseError(_)
            | Self::InvalidFormat
            | Self::UnsupportedFormat { .. }
            | Self::NoFrontmatterFound
            | Self::InvalidJson
            | Self::InvalidToml
            | Self::InvalidYaml
            | Self::JsonDepthLimitExceeded
            | Self::ExtractionError(_)
            | Self::InvalidUrl(_)
            | Self::InvalidLanguage(_) => Category::Parsing,
            Self::ValidationError(_) => Category::Validation,
            Self::ConversionError(_) => Category::Conversion,
            Self::ContentTooLarge { .. }
            | Self::NestingTooDeep { .. }
            | Self::Other(_) => Category::Configuration,
        }
    }

    /// Creates a generic parse error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice containing the error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use frontmatter_gen::error::Error;
    ///
    /// let error = Error::generic_parse_error("Invalid syntax");
    /// assert!(matches!(error, Error::ParseError(_)));
    /// ```
    #[must_use]
    pub fn generic_parse_error(message: &str) -> Self {
        Self::ParseError(message.to_string())
    }

    /// Creates an unsupported format error for a specific line.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number where the unsupported format was detected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use frontmatter_gen::error::Error;
    ///
    /// let error = Error::unsupported_format(42);
    /// assert!(matches!(error, Error::UnsupportedFormat { line: 42 }));
    /// ```
    #[must_use]
    pub const fn unsupported_format(line: usize) -> Self {
        Self::UnsupportedFormat { line }
    }

    /// Creates a validation error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice containing the validation error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use frontmatter_gen::error::Error;
    ///
    /// let error = Error::validation_error("Invalid character in title");
    /// assert!(matches!(error, Error::ValidationError(_)));
    /// ```
    #[must_use]
    pub fn validation_error(message: &str) -> Self {
        Self::ValidationError(message.to_string())
    }

    /// Adds context to an error.
    ///
    /// # Arguments
    ///
    /// * `context` - Additional context information about the error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use frontmatter_gen::error::{Error, Context};
    ///
    /// let context = Context {
    ///     line: Some(42),
    ///     column: Some(10),
    ///     snippet: Some("invalid content".to_string()),
    /// };
    ///
    /// let error = Error::ParseError("Invalid syntax".to_string())
    ///     .with_context(&context);
    /// ```
    #[must_use]
    pub fn with_context(self, context: &Context) -> Self {
        let context_info = format!(
            " (line: {}, column: {})",
            context.line.unwrap_or(0),
            context.column.unwrap_or(0)
        );
        let snippet_info = context
            .snippet
            .as_ref()
            .map(|s| format!(" near '{}'", s))
            .unwrap_or_default();

        match self {
            Self::ParseError(msg) => Self::ParseError(format!(
                "{msg}{context_info}{snippet_info}"
            )),
            Self::YamlParseError { source } => {
                Self::YamlParseError { source }
            }
            _ => self, // For unsupported variants
        }
    }
}

/// Errors that can occur during site generation.
///
/// This enum is used to represent higher-level errors encountered during site
/// generation processes, such as template rendering, file system operations,
/// and metadata processing.
#[derive(Error, Debug)]
pub enum EngineError {
    /// Error occurred during content processing.
    #[error("Content processing error: {0}")]
    ContentError(String),

    /// Error occurred during template processing.
    #[error("Template processing error: {0}")]
    TemplateError(String),

    /// Error occurred during asset processing.
    #[error("Asset processing error: {0}")]
    AssetError(String),

    /// Error occurred during file system operations.
    #[error("File system error: {source}")]
    FileSystemError {
        /// The original IO error that caused this error.
        source: std::io::Error,
        /// Additional context information about the error.
        context: String,
    },

    /// Error occurred during metadata processing.
    #[error("Metadata error: {0}")]
    MetadataError(String),
}

impl Clone for EngineError {
    fn clone(&self) -> Self {
        match self {
            Self::ContentError(msg) => Self::ContentError(msg.clone()),
            Self::TemplateError(msg) => {
                Self::TemplateError(msg.clone())
            }
            Self::AssetError(msg) => Self::AssetError(msg.clone()),
            Self::FileSystemError { source, context } => {
                Self::FileSystemError {
                    source: std::io::Error::new(
                        source.kind(),
                        source.to_string(),
                    ),
                    context: context.clone(),
                }
            }
            Self::MetadataError(msg) => {
                Self::MetadataError(msg.clone())
            }
        }
    }
}

/// Converts an `EngineError` into an `Error`.
///
/// This allows engine errors to be converted into front matter errors when needed,
/// preserving the error context and message.
///
/// # Examples
///
/// ```rust
/// use frontmatter_gen::error::{EngineError, Error};
/// use std::io;
///
/// let engine_error = EngineError::ContentError("content processing failed".to_string());
/// let frontmatter_error: Error = engine_error.into();
/// assert!(matches!(frontmatter_error, Error::ParseError(_)));
/// ```
impl From<EngineError> for Error {
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
            EngineError::FileSystemError { source, context } => {
                Self::ParseError(format!(
                    "File system error: {} ({})",
                    source, context
                ))
            }
            EngineError::MetadataError(msg) => {
                Self::ParseError(format!("Metadata error: {}", msg))
            }
        }
    }
}

/// Converts an IO error (`std::io::Error`) into a front matter `Error`.
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::ParseError(err.to_string())
    }
}

/// Converts a front matter `Error` into a string.
impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

#[cfg(test)]
mod tests {
    /// Tests for the `Error` enum and its associated methods.
    mod error_tests {
        use super::super::*;

        /// Test the `ContentTooLarge` error variant.
        #[test]
        fn test_content_too_large_error() {
            let error = Error::ContentTooLarge {
                size: 1000,
                max: 500,
            };
            assert_eq!(
                error.to_string(),
                "Your front matter contains too many fields (1000). The maximum allowed is 500."
            );
        }

        /// Test the `NestingTooDeep` error variant.
        #[test]
        fn test_nesting_too_deep_error() {
            let error = Error::NestingTooDeep { depth: 10, max: 5 };
            assert_eq!(
                error.to_string(),
                "Your front matter is nested too deeply (10 levels). The maximum allowed nesting depth is 5."
            );
        }

        /// Test the `JsonParseError` error variant.
        #[test]
        fn test_json_parse_error() {
            let json_data = r#"{"key": invalid}"#;
            let result: Result<serde_json::Value, _> =
                serde_json::from_str(json_data);
            assert!(result.is_err());
            let error =
                Error::JsonParseError(Arc::new(result.unwrap_err()));
            assert!(matches!(error, Error::JsonParseError(_)));
        }

        /// Test the `InvalidFormat` error variant.
        #[test]
        fn test_invalid_format_error() {
            let error = Error::InvalidFormat;
            assert_eq!(
                error.to_string(),
                "Invalid front matter format"
            );
        }

        /// Test the `ConversionError` error variant.
        #[test]
        fn test_conversion_error() {
            let error =
                Error::ConversionError("Conversion failed".to_string());
            assert_eq!(
                error.to_string(),
                "Failed to convert front matter: Conversion failed"
            );
        }

        /// Test the `UnsupportedFormat` error variant.
        #[test]
        fn test_unsupported_format() {
            let error = Error::unsupported_format(42);
            assert!(matches!(
                error,
                Error::UnsupportedFormat { line: 42 }
            ));
            assert_eq!(
                error.to_string(),
                "Unsupported front matter format detected at line 42"
            );
        }

        /// Test the `NoFrontmatterFound` error variant.
        #[test]
        fn test_no_frontmatter_found() {
            let error = Error::NoFrontmatterFound;
            assert_eq!(
                error.to_string(),
                "No front matter found in the content"
            );
        }

        /// Test the `InvalidJson` error variant.
        #[test]
        fn test_invalid_json_error() {
            let error = Error::InvalidJson;
            assert_eq!(error.to_string(), "Invalid JSON front matter: malformed or invalid structure.");
        }

        /// Test the `InvalidUrl` error variant.
        #[test]
        fn test_invalid_url_error() {
            let error =
                Error::InvalidUrl("http:// invalid.url".to_string());
            assert_eq!(
                error.to_string(),
                "Invalid URL: http:// invalid.url. Ensure the URL is well-formed and valid."
            );
        }

        /// Test the `InvalidYaml` error variant.
        #[test]
        fn test_invalid_yaml_error() {
            let error = Error::InvalidYaml;
            assert_eq!(
                error.to_string(),
                "Invalid YAML front matter: malformed or invalid structure."
            );
        }

        /// Test the `ValidationError` error variant.
        #[test]
        fn test_validation_error() {
            let error =
                Error::ValidationError("Invalid title".to_string());
            assert_eq!(
                error.to_string(),
                "Input validation error: Invalid title"
            );
        }

        /// Test the `JsonDepthLimitExceeded` error variant.
        #[test]
        fn test_json_depth_limit_exceeded() {
            let error = Error::JsonDepthLimitExceeded;
            assert_eq!(
                error.to_string(),
                "JSON front matter exceeds maximum nesting depth"
            );
        }

        /// Test the `category` method for different error variants.
        #[test]
        fn test_category_method() {
            let validation_error =
                Error::ValidationError("Invalid field".to_string());
            assert_eq!(
                validation_error.category(),
                Category::Validation
            );

            let conversion_error =
                Error::ConversionError("Conversion failed".to_string());
            assert_eq!(
                conversion_error.category(),
                Category::Conversion
            );

            let config_error =
                Error::ContentTooLarge { size: 100, max: 50 };
            assert_eq!(
                config_error.category(),
                Category::Configuration
            );
        }

        /// Test the `Clone` implementation for `Error`.
        #[test]
        fn test_error_clone() {
            let original = Error::ContentTooLarge {
                size: 200,
                max: 100,
            };
            let cloned = original.clone();
            assert!(
                matches!(cloned, Error::ContentTooLarge { size, max } if size == 200 && max == 100)
            );
        }
    }

    /// Tests for the `EngineError` enum.
    mod engine_error_tests {
        use super::super::*;
        use std::io;

        /// Test the `ContentError` variant.
        #[test]
        fn test_content_error() {
            let error = EngineError::ContentError(
                "Processing failed".to_string(),
            );
            assert_eq!(
                error.to_string(),
                "Content processing error: Processing failed"
            );
        }

        /// Test `EngineError::FileSystemError` conversion to `Error`.
        #[test]
        fn test_engine_error_to_error_conversion() {
            let io_error =
                io::Error::new(io::ErrorKind::Other, "disk full");
            let engine_error = EngineError::FileSystemError {
                source: io_error,
                context: "Saving file".to_string(),
            };
            let converted: Error = engine_error.into();
            assert!(converted.to_string().contains("disk full"));
            assert!(converted.to_string().contains("Saving file"));
        }
    }

    /// Tests for the `Context` struct.
    mod context_tests {
        use super::super::*;

        /// Test the `Display` implementation of `Context`.
        #[test]
        fn test_context_display() {
            let context = Context {
                line: Some(42),
                column: Some(10),
                snippet: Some("invalid key".to_string()),
            };
            assert_eq!(
                context.to_string(),
                "at 42:10 near 'invalid key'"
            );
        }

        /// Test missing fields in `Context`.
        #[test]
        fn test_context_missing_fields() {
            let context = Context {
                line: None,
                column: None,
                snippet: Some("example snippet".to_string()),
            };
            assert_eq!(
                context.to_string(),
                "at 0:0 near 'example snippet'"
            );
        }
    }

    /// Tests for conversions.
    mod conversion_tests {
        use super::super::*;
        use std::io;

        /// Test the conversion from `std::io::Error` to `Error`.
        #[test]
        fn test_io_error_conversion() {
            let io_error =
                io::Error::new(io::ErrorKind::NotFound, "file missing");
            let error: Error = io_error.into();
            assert!(matches!(error, Error::ParseError(_)));
            assert!(error.to_string().contains("file missing"));
        }
    }



    /// Test the conversion of `EngineError` to `Error`.
        #[test]
        fn test_engine_error_conversion() {
            let engine_error =
                crate::error::EngineError::ContentError("content failed".to_string());
            let error: crate::Error = engine_error.into();
            assert!(matches!(error, crate::Error::ParseError(_)));
            assert!(error.to_string().contains("content failed"));
        }
}
