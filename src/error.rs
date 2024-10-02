use serde_json::Error as JsonError;
use serde_yml::Error as YamlError;
use thiserror::Error;

/// Represents errors that can occur during frontmatter parsing, conversion, and extraction.
///
/// This enum uses the `thiserror` crate to provide clear and structured error messages,
/// making it easier to debug and handle issues that arise when processing frontmatter.
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
    /// Helper function to create a generic parse error with a custom message.
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
}
