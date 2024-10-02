// src/lib.rs

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/frontmatter-gen/images/favicon.ico",
    html_logo_url = "https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg",
    html_root_url = "https://docs.rs/frontmatter-gen"
)]
#![crate_name = "frontmatter_gen"]
#![crate_type = "lib"]

/// The `error` module contains error types related to the frontmatter parsing process.
pub mod error;
/// The `extractor` module contains functions for extracting raw frontmatter from content.
pub mod extractor;
/// The `parser` module contains functions for parsing frontmatter into a structured format.
pub mod parser;
/// The `types` module contains types related to the frontmatter parsing process.
pub mod types;

use error::FrontmatterError;
use extractor::{detect_format, extract_raw_frontmatter};
use parser::{parse, to_string};
// Re-export types for external access
pub use types::{Format, Frontmatter, Value}; // Add `Frontmatter` and `Format` to the public interface

/// Extracts frontmatter from a string of content.
///
/// This function attempts to extract frontmatter from the given content string.
/// It supports YAML, TOML, and JSON formats.
///
/// # Arguments
///
/// * `content` - A string slice containing the content to parse.
///
/// # Returns
///
/// * `Ok((Frontmatter, &str))` - A tuple containing the parsed frontmatter and the remaining content.
/// * `Err(FrontmatterError)` - An error if extraction or parsing fails.
///
/// # Examples
///
/// ```
/// use frontmatter_gen::{extract, Frontmatter};
///
/// let yaml_content = r#"---
/// title: My Post
/// date: 2023-05-20
/// ---
/// Content here"#;
///
/// let (frontmatter, remaining_content) = extract(yaml_content).unwrap();
/// assert_eq!(frontmatter.get("title").unwrap().as_str().unwrap(), "My Post");
/// assert_eq!(remaining_content, "Content here");
/// ```
pub fn extract(
    content: &str,
) -> Result<(Frontmatter, &str), FrontmatterError> {
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
/// * `frontmatter` - The Frontmatter to convert.
/// * `format` - The target Format to convert to.
///
/// # Returns
///
/// * `Ok(String)` - The frontmatter converted to the specified format.
/// * `Err(FrontmatterError)` - An error if conversion fails.
///
/// # Examples
///
/// ```
/// use frontmatter_gen::{Frontmatter, Format, to_format};
///
/// let mut frontmatter = Frontmatter::new();
/// frontmatter.insert("title".to_string(), "My Post".into());
/// frontmatter.insert("date".to_string(), "2023-05-20".into());
///
/// let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
/// assert!(yaml.contains("title: My Post"));
/// assert!(yaml.contains("date: '2023-05-20'"));
/// ```
pub fn to_format(
    frontmatter: &Frontmatter,
    format: Format,
) -> Result<String, FrontmatterError> {
    to_string(frontmatter, format)
}
