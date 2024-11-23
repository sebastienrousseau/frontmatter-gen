//! This module provides functionality for extracting frontmatter from content.
//!
//! It includes functions to extract frontmatter in various formats (YAML, TOML, JSON) from a given string content, as well as utilities to detect the format of the frontmatter.

use crate::error::Error;
use crate::types::Format;

/// Extracts raw frontmatter from the content, detecting YAML, TOML, or JSON formats.
///
/// This function tries to extract frontmatter based on the common delimiters for
/// YAML (`---`), TOML (`+++`), and JSON (`{}`). If frontmatter is detected, it
/// returns the extracted frontmatter and the remaining content.
///
/// # Arguments
///
/// * `content` - The full content string that may contain frontmatter.
///
/// # Returns
///
/// A `Result` containing a tuple of two `&str` slices: the raw frontmatter and the remaining content.
/// If no valid frontmatter format is found, it returns `Error::InvalidFormat`.
///
/// # Errors
///
/// - `Error::InvalidFormat`: When the frontmatter format is not recognized.
/// - `Error::ExtractionError`: When there is an issue extracting frontmatter.
///
/// # Example
///
/// ```rust
/// use frontmatter_gen::extractor::{extract_delimited_frontmatter, extract_raw_frontmatter, extract_json_frontmatter};
/// let content = "---\ntitle: Example\n---\nContent here";
/// let result = extract_raw_frontmatter(content).unwrap();
/// assert_eq!(result.0, "title: Example");
/// assert_eq!(result.1, "Content here");
/// ```
pub fn extract_raw_frontmatter(
    content: &str,
) -> Result<(&str, &str), Error> {
    // Extract YAML frontmatter
    if let Some(yaml) =
        extract_delimited_frontmatter(content, "---\n", "\n---")
            .or_else(|| {
                extract_delimited_frontmatter(
                    content, "---\r\n", "\r\n---",
                )
            })
    {
        let remaining = &content[content
            .find("\n---\n")
            .or_else(|| content.find("\r\n---\r\n"))
            .map_or(content.len(), |i| i + 5)..];
        return Ok((yaml, remaining.trim_start()));
    }

    // Extract TOML frontmatter
    if let Some(toml) =
        extract_delimited_frontmatter(content, "+++\n", "\n+++")
            .or_else(|| {
                extract_delimited_frontmatter(
                    content, "+++\r\n", "\r\n+++",
                )
            })
    {
        let remaining = &content[content
            .find("\n+++\n")
            .or_else(|| content.find("\r\n+++\r\n"))
            .map_or(content.len(), |i| i + 5)..];
        return Ok((toml, remaining.trim_start()));
    }

    // Extract JSON frontmatter
    if let Ok(json) = extract_json_frontmatter(content) {
        let remaining = &content[json.len()..];
        return Ok((json, remaining.trim_start()));
    }

    // Handle cases where frontmatter delimiters exist but are empty
    if content.starts_with("---\n---")
        || content.starts_with("+++\n+++")
    {
        return Err(Error::InvalidFormat);
    }

    Err(Error::InvalidFormat)
}

/// Extracts JSON frontmatter from the content by detecting balanced curly braces (`{}`).
///
/// This function attempts to locate a valid JSON object starting with `{` and checks for balanced
/// curly braces to identify the end of the frontmatter. If the JSON object is found, it returns
/// the frontmatter as a string slice. A maximum nesting depth is enforced to prevent deeply nested
/// JSON from causing stack overflow.
///
/// # Arguments
///
/// * `content` - The full content string that may contain JSON frontmatter.
///
/// # Returns
///
/// A `Result` containing the extracted JSON frontmatter string slice.
/// If no valid JSON frontmatter is detected, it returns an `Error`.
///
/// # Errors
///
/// - `Error::InvalidJson`: If the content does not start with `{` or contains unbalanced braces.
/// - `Error::JsonDepthLimitExceeded`: If the JSON object exceeds the allowed nesting depth.
///
/// # Example
///
/// ```rust
/// use frontmatter_gen::extractor::extract_json_frontmatter;
/// let content = "{ \"title\": \"Example\" }\nContent";
/// let frontmatter = extract_json_frontmatter(content).unwrap();
/// assert_eq!(frontmatter, "{ \"title\": \"Example\" }");
/// ```
pub fn extract_json_frontmatter(content: &str) -> Result<&str, Error> {
    const MAX_DEPTH: usize = 100; // Limit maximum nesting depth
    let trimmed = content.trim_start();

    // If the content doesn't start with '{', it's not JSON frontmatter.
    if !trimmed.starts_with('{') {
        return Err(Error::InvalidJson);
    }

    let mut brace_count = 0;
    let mut depth = 0;
    let mut in_string = false;
    let mut escape_next = false;

    // Iterate over the characters in the trimmed content, looking for balanced braces.
    for (idx, ch) in trimmed.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '"' if !in_string => in_string = true,
            '"' if in_string => in_string = false,
            '\\' if in_string => escape_next = true,
            '{' if !in_string => {
                brace_count += 1;
                depth += 1;
                // Check if the maximum depth is exceeded
                if depth > MAX_DEPTH {
                    return Err(Error::JsonDepthLimitExceeded);
                }
            }
            '}' if !in_string => {
                brace_count -= 1;
                // Decrease depth when closing braces
                if depth > 0 {
                    depth = depth.saturating_sub(1);
                }
                // Once braces are balanced (brace_count == 0), we've reached the end of the JSON object.
                if brace_count == 0 {
                    return Ok(&trimmed[..=idx]);
                }
            }
            _ => {}
        }
    }

    // If no balanced braces are found, return an error.
    Err(Error::InvalidJson)
}

/// Detects the format of the extracted frontmatter.
///
/// This function analyzes the raw frontmatter and determines whether it is in YAML,
/// TOML, or JSON format by examining the structure of the data.
///
/// # Arguments
///
/// * `raw_frontmatter` - The extracted frontmatter as a string slice.
///
/// # Returns
///
/// A `Result` containing the detected `Format` (either `Json`, `Toml`, or `Yaml`).
///
/// # Errors
///
/// - `Error::InvalidFormat`: If the format cannot be determined.
///
/// # Example
///
/// ```rust
/// use frontmatter_gen::extractor::detect_format;
/// use frontmatter_gen::Format;
/// let raw = "---\ntitle: Example\n---";
/// let format = detect_format(raw).unwrap();
/// assert_eq!(format, Format::Yaml);
/// ```
pub fn detect_format(raw_frontmatter: &str) -> Result<Format, Error> {
    let trimmed = raw_frontmatter.trim_start();

    // Check for YAML front matter marker
    if trimmed.starts_with("---") {
        return Ok(Format::Yaml);
    }

    // Check for JSON structure
    if trimmed.starts_with('{') {
        return Ok(Format::Json);
    }

    // Check for YAML-like structure
    if trimmed.contains(':') && !trimmed.contains('{') {
        return Ok(Format::Yaml);
    }

    // Check for TOML-like structure
    if trimmed.contains('=') {
        return Ok(Format::Toml);
    }

    // Default to an error if none of the formats match
    Err(Error::InvalidFormat)
}

/// Extracts frontmatter enclosed by the given start and end delimiters.
///
/// This function checks for frontmatter enclosed by delimiters like `---` for YAML or `+++` for TOML.
/// It returns the extracted frontmatter if the delimiters are found.
///
/// # Arguments
///
/// * `content` - The full content string containing frontmatter.
/// * `start_delim` - The starting delimiter (e.g., `---\n` for YAML).
/// * `end_delim` - The ending delimiter (e.g., `\n---\n` for YAML).
///
/// # Returns
///
/// An `Option` containing the extracted frontmatter as a string slice. Returns `None`
/// if the delimiters are not found.
///
/// # Example
///
/// ```rust
/// use frontmatter_gen::extractor::extract_delimited_frontmatter;
/// let content = "---\ntitle: Example\n---\nContent";
/// let frontmatter = extract_delimited_frontmatter(content, "---\n", "\n---\n").unwrap();
/// assert_eq!(frontmatter, "title: Example");
/// ```
#[must_use]
pub fn extract_delimited_frontmatter<'a>(
    content: &'a str,
    start_delim: &str,
    end_delim: &str,
) -> Option<&'a str> {
    let start_index = content.find(start_delim)? + start_delim.len();
    let end_index = content.find(end_delim)?;

    if start_index <= end_index {
        Some(content[start_index..end_index].trim())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for extracting raw frontmatter
    mod extract_raw_frontmatter {
        use super::*;

        #[test]
        fn test_extract_yaml() {
            let content = r"---
title: Example
---
Content here";
            let result = extract_raw_frontmatter(content).unwrap();
            assert_eq!(result.0, "title: Example");
            assert_eq!(result.1, "Content here");
        }

        #[test]
        fn test_extract_toml() {
            let content = r#"+++
title = "Example"
+++
Content here"#;
            let result = extract_raw_frontmatter(content).unwrap();
            assert_eq!(result.0, r#"title = "Example""#);
            assert_eq!(result.1, "Content here");
        }

        #[test]
        fn test_extract_json() {
            let content = r#"{ "title": "Example" }
Content here"#;
            let result = extract_raw_frontmatter(content).unwrap();
            assert_eq!(result.0, r#"{ "title": "Example" }"#);
            assert_eq!(result.1, "Content here");
        }

        #[test]
        fn test_invalid_format() {
            let content = "Invalid frontmatter";
            let result = detect_format(content);
            if let Err(Error::InvalidFormat) = result {
                // Test passed
            } else {
                panic!("Expected Err(InvalidFormat), got {:?}", result);
            }
        }
    }

    /// Tests for JSON frontmatter extraction
    mod extract_json_frontmatter {
        use super::*;

        #[test]
        fn test_valid_json() {
            let content = r#"{ "title": "Example" }
Content here"#;
            let result = extract_json_frontmatter(content).unwrap();
            assert_eq!(result, r#"{ "title": "Example" }"#);
        }

        #[test]
        fn test_nested_json() {
            let content = r#"{ "a": { "b": { "c": { "d": { "e": {} }}}}}
Content here"#;
            let result = extract_json_frontmatter(content);
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap(),
                r#"{ "a": { "b": { "c": { "d": { "e": {} }}}}}"#
            );
        }

        #[test]
        fn test_too_deep_json() {
            let mut content = String::from("{ ");
            for _ in 0..101 {
                content.push_str(r#""a": { "#);
            }
            content.push_str(&"}".repeat(101));
            content.push_str("\nContent here");

            let result = extract_json_frontmatter(&content);
            assert!(matches!(
                result,
                Err(Error::JsonDepthLimitExceeded)
            ));
        }

        #[test]
        fn test_escaped_characters() {
            let content = r#"{ "title": "Example with \"quotes\" and {braces}", "content": "Some text with \\ backslash" }
Actual content starts here"#;
            let result = extract_json_frontmatter(content).unwrap();
            assert_eq!(
                result,
                r#"{ "title": "Example with \"quotes\" and {braces}", "content": "Some text with \\ backslash" }"#
            );
        }

        #[test]
        fn test_invalid_json() {
            let content = "Not a JSON frontmatter";
            let result = extract_json_frontmatter(content);
            assert!(matches!(result, Err(Error::InvalidJson)));
        }
    }

    /// Tests for format detection
    mod detect_format {
        use super::*;

        #[test]
        fn test_yaml_format() {
            let content = "title: Example";
            let result = detect_format(content).unwrap();
            assert_eq!(result, Format::Yaml);
        }

        #[test]
        fn test_toml_format() {
            let content = "title = \"Example\"";
            let result = detect_format(content).unwrap();
            assert_eq!(result, Format::Toml);
        }

        #[test]
        fn test_json_format() {
            let content = r#"{ "title": "Example" }"#;
            let result = detect_format(content).unwrap();
            assert_eq!(result, Format::Json);
        }

        #[test]
        fn test_invalid_format() {
            let content = "Invalid content";
            let result = detect_format(content);
            assert!(matches!(result, Err(Error::InvalidFormat)));
        }
    }

    /// Tests for delimited frontmatter extraction
    mod extract_delimited_frontmatter {
        use super::*;

        #[test]
        fn test_valid_yaml() {
            let content = "---\ntitle: Example\n---\nContent here";
            let result = extract_delimited_frontmatter(
                content, "---\n", "\n---\n",
            )
            .unwrap();
            assert_eq!(result, "title: Example");
        }

        #[test]
        fn test_valid_toml() {
            let content = "+++\ntitle = \"Example\"\n+++\nContent here";
            let result = extract_delimited_frontmatter(
                content, "+++\n", "\n+++\n",
            )
            .unwrap();
            assert_eq!(result, r#"title = "Example""#);
        }

        #[test]
        fn test_valid_windows_format() {
            let content =
                "---\r\ntitle: Example\r\n---\r\nContent here";
            let result = extract_delimited_frontmatter(
                content,
                "---\r\n",
                "\r\n---\r\n",
            )
            .unwrap();
            assert_eq!(result, "title: Example");
        }

        #[test]
        fn test_missing_start_delimiter() {
            let content = "title: Example\n---\nContent here";
            let result = extract_delimited_frontmatter(
                content, "---\n", "\n---\n",
            );
            assert!(result.is_none());
        }

        #[test]
        fn test_missing_end_delimiter() {
            let content = "---\ntitle: Example\nContent here";
            let result = extract_delimited_frontmatter(
                content, "---\n", "\n---\n",
            );
            assert!(result.is_none());
        }
    }
}
