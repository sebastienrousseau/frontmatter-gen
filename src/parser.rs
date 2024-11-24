//! # Front Matter Parser and Serialiser Module
//!
//! This module provides robust functionality for parsing and serialising front matter
//! in various formats (YAML, TOML, and JSON). It focuses on:
//!
//! - Memory efficiency through pre-allocation and string optimisation
//! - Type safety with comprehensive error handling
//! - Performance optimisation with minimal allocations
//! - Validation of input data
//! - Consistent cross-format handling
//!
//! ## Features
//!
//! - Multi-format support (YAML, TOML, JSON)
//! - Zero-copy parsing where possible
//! - Efficient memory management
//! - Comprehensive validation
//! - Rich error context
//!
//! ## Usage Example
//!
//! ```rust
//! use frontmatter_gen::{Format, parser};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let yaml = "title: My Post\ndate: 2025-09-09\n";
//! let front_matter = parser::parse_with_options(
//!     yaml,
//!     Format::Yaml,
//!     None
//! )?;
//! # Ok(())
//! # }
//! ```

use serde::Serialize;
use serde_json::Value as JsonValue;
use serde_yml::Value as YamlValue;
use std::{collections::HashMap, sync::Arc};
use toml::Value as TomlValue;

use crate::{error::Error, types::Frontmatter, Format, Value};

// Constants for optimisation and validation
const SMALL_STRING_SIZE: usize = 24;
const MAX_NESTING_DEPTH: usize = 32;
const MAX_KEYS: usize = 1000;

/// Options for controlling parsing behaviour.
///
/// Provides configuration for maximum allowed nesting depth, maximum number of keys,
/// and whether to perform validation.
#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    /// Maximum allowed nesting depth.
    pub max_depth: usize,
    /// Maximum allowed number of keys.
    pub max_keys: usize,
    /// Whether to validate the structure.
    pub validate: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            max_depth: MAX_NESTING_DEPTH,
            max_keys: MAX_KEYS,
            validate: true,
        }
    }
}

/// Optimises string storage based on length.
///
/// For strings shorter than `SMALL_STRING_SIZE`, uses standard allocation.
/// For longer strings, pre-allocates exact capacity to avoid reallocations.
///
/// # Arguments
///
/// * `s` - The input string slice to optimise.
///
/// # Returns
///
/// An optimised owned `String`.
#[inline]
fn optimise_string(s: &str) -> String {
    if s.len() <= SMALL_STRING_SIZE {
        s.to_string()
    } else {
        let mut string = String::with_capacity(s.len());
        string.push_str(s);
        string
    }
}

/// Parses raw front matter string into a `Frontmatter` object based on the specified format.
///
/// This function attempts to parse the provided string into a structured `Frontmatter`
/// object according to the specified format. It performs validation by default
/// and optimises memory allocation where possible.
///
/// # Arguments
///
/// * `raw_front_matter` - A string slice containing the raw front matter content.
/// * `format` - The `Format` enum specifying the desired format.
/// * `options` - Optional parsing options for controlling validation and limits.
///
/// # Returns
///
/// A `Result` containing either the parsed `Frontmatter` object or a `Error`.
///
/// # Errors
///
/// Returns `Error` if:
/// - The input is not valid in the specified format
/// - The structure exceeds configured limits
/// - The format is unsupported
pub fn parse_with_options(
    raw_front_matter: &str,
    format: Format,
    options: Option<ParseOptions>,
) -> Result<Frontmatter, Error> {
    let options = options.unwrap_or_default();

    // Check for unsupported formats
    if format == Format::Unsupported {
        let err_msg = format!(
            "Unsupported format: {:?}. Supported formats are YAML, TOML, and JSON.",
            format
        );
        log::error!("{}", err_msg);
        return Err(Error::ConversionError(err_msg));
    }

    // Trim the input and validate format assumptions
    let trimmed_content = raw_front_matter.trim();

    // Format-specific validation
    match format {
        Format::Yaml => {
            if !trimmed_content.starts_with("---") {
                log::debug!("YAML front matter validation: Content structure appears non-standard");
            }
        }
        Format::Toml => {
            if !trimmed_content.contains('=') {
                return Err(Error::ConversionError(
                    "Format set to TOML but input does not contain '=' signs.".to_string(),
                ));
            }
        }
        Format::Json => {
            if !trimmed_content.starts_with('{') {
                return Err(Error::ConversionError(
                    "Format set to JSON but input does not start with '{'."
                        .to_string(),
                ));
            }
        }
        Format::Unsupported => unreachable!(), // We've already handled this case above
    };

    let front_matter = match format {
        Format::Yaml => parse_yaml(trimmed_content).map_err(|e| {
            log::error!("YAML parsing failed: {}", e);
            e
        })?,
        Format::Toml => parse_toml(trimmed_content).map_err(|e| {
            log::error!("TOML parsing failed: {}", e);
            e
        })?,
        Format::Json => parse_json(trimmed_content).map_err(|e| {
            log::error!("JSON parsing failed: {}", e);
            e
        })?,
        Format::Unsupported => unreachable!(),
    };

    // Perform validation if specified in options
    if options.validate {
        log::debug!(
            "Validating front matter: maximum allowed nesting depth is {}, maximum allowed number of keys is {}",
            options.max_depth,
            options.max_keys
        );

        validate_frontmatter(
            &front_matter,
            options.max_depth,
            options.max_keys,
        )
        .map_err(|e| {
            log::error!("Front matter validation failed: {}", e);
            e
        })?;
    }

    Ok(front_matter)
}

/// Convenience wrapper around `parse_with_options` using default options.
///
/// # Arguments
///
/// * `raw_front_matter` - A string slice containing the raw front matter content.
/// * `format` - The `Format` enum specifying the desired format.
///
/// # Returns
///
/// A `Result` containing either the parsed `Frontmatter` object or a `Error`.
///
/// # Errors
///
/// Returns an `Error` if:
/// - The format is invalid or unsupported.
/// - Parsing fails due to invalid syntax.
/// - Validation fails if enabled.
pub fn parse(
    raw_front_matter: &str,
    format: Format,
) -> Result<Frontmatter, Error> {
    parse_with_options(raw_front_matter, format, None)
}

/// Converts a `Frontmatter` object to a string representation in the specified format.
///
/// # Arguments
///
/// * `front_matter` - Reference to the `Frontmatter` object to serialise.
/// * `format` - The target format for serialisation.
///
/// # Returns
///
/// A `Result` containing the serialised string or a `Error`.
///
/// # Errors
///
/// Returns `Error` if:
/// - Serialisation fails.
/// - The specified format is unsupported.
pub fn to_string(
    front_matter: &Frontmatter,
    format: Format,
) -> Result<String, Error> {
    match format {
        Format::Yaml => to_yaml(front_matter),
        Format::Toml => to_toml(front_matter),
        Format::Json => to_json_optimised(front_matter),
        Format::Unsupported => Err(Error::ConversionError(
            "Unsupported format".to_string(),
        )),
    }
}

// YAML Implementation
// -------------------

/// Parses a YAML string into a `Frontmatter` object.
///
/// # Arguments
///
/// * `raw` - The raw YAML string.
///
/// # Returns
///
/// A `Result` containing the parsed `Frontmatter` or a `Error`.
fn parse_yaml(raw: &str) -> Result<Frontmatter, Error> {
    // Parse the YAML content into a serde_yml::Value
    let yaml_value: YamlValue = serde_yml::from_str(raw)
        .map_err(|e| Error::YamlParseError { source: e.into() })?;

    // Prepare the front matter container
    let capacity =
        yaml_value.as_mapping().map_or(0, serde_yml::Mapping::len);
    let mut front_matter =
        Frontmatter(HashMap::with_capacity(capacity));

    // Convert the YAML mapping into the front matter structure
    if let YamlValue::Mapping(mapping) = yaml_value {
        for (key, value) in mapping {
            if let YamlValue::String(k) = key {
                let _ = front_matter.insert(k, yaml_to_value(&value));
            } else {
                // Log a warning for non-string keys
                log::warn!("Warning: Non-string key ignored in YAML front matter");
            }
        }
    } else {
        return Err(Error::ParseError(
            "YAML front matter is not a valid mapping".to_string(),
        ));
    }

    Ok(front_matter)
}

/// Converts a `serde_yml::Value` into a `Value`.
fn yaml_to_value(yaml: &YamlValue) -> Value {
    match yaml {
        YamlValue::Null => Value::Null,
        YamlValue::Bool(b) => Value::Boolean(*b),
        YamlValue::Number(n) => {
            n.as_i64()
                .map_or_else(
                    || {
                        n.as_f64().map_or_else(
                            || {
                                log::warn!(
                                    "Invalid or unsupported number encountered in YAML: {:?}",
                                    n
                                );
                                Value::Number(0.0) // Fallback for invalid numbers
                            },
                            Value::Number,
                        )
                    },
                    |i| {
                        if i.abs() < (1_i64 << 52) {
                            Value::Number(i as f64)
                        } else {
                            log::warn!(
                                "Integer {} exceeds precision of f64. Defaulting to 0.0",
                                i
                            );
                            Value::Number(0.0) // Fallback for large values outside f64 precision
                        }
                    },
                )
        }
        YamlValue::String(s) => Value::String(optimise_string(s)),
        YamlValue::Sequence(seq) => {
            let mut vec = Vec::with_capacity(seq.len());
            vec.extend(seq.iter().map(yaml_to_value));
            Value::Array(vec)
        }
        YamlValue::Mapping(map) => {
            let mut result =
                Frontmatter(HashMap::with_capacity(map.len()));
            for (k, v) in map {
                if let YamlValue::String(key) = k {
                    let _ = result
                        .0
                        .insert(optimise_string(key), yaml_to_value(v));
                } else {
                    log::warn!(
                        "Non-string key in YAML mapping ignored: {:?}",
                        k
                    );
                }
            }
            Value::Object(Box::new(result))
        }
        YamlValue::Tagged(tagged) => Value::Tagged(
            optimise_string(&tagged.tag.to_string()),
            Box::new(yaml_to_value(&tagged.value)),
        ),
    }
}

/// Serialises a `Frontmatter` object into a YAML string.
///
/// # Arguments
///
/// * `front_matter` - The `Frontmatter` object to serialise.
///
/// # Returns
///
/// A `Result` containing the serialised YAML string or a `Error`.
fn to_yaml(front_matter: &Frontmatter) -> Result<String, Error> {
    serde_yml::to_string(&front_matter.0)
        .map_err(|e| Error::ConversionError(e.to_string()))
}

// TOML Implementation
// -------------------

/// Parses a TOML string into a `Frontmatter` object.
///
/// # Arguments
///
/// * `raw` - The raw TOML string.
///
/// # Returns
///
/// A `Result` containing the parsed `Frontmatter` or a `Error`.
fn parse_toml(raw: &str) -> Result<Frontmatter, Error> {
    let toml_value: TomlValue =
        raw.parse().map_err(Error::TomlParseError)?;

    let capacity = match &toml_value {
        TomlValue::Table(table) => table.len(),
        _ => 0,
    };

    let mut front_matter =
        Frontmatter(HashMap::with_capacity(capacity));

    if let TomlValue::Table(table) = toml_value {
        for (key, value) in table {
            let _ = front_matter.0.insert(key, toml_to_value(&value));
        }
    }

    Ok(front_matter)
}

/// Converts a `toml::Value` into a `Value`.
fn toml_to_value(toml: &TomlValue) -> Value {
    match toml {
        TomlValue::String(s) => Value::String(optimise_string(s)),
        TomlValue::Integer(i) => Value::Number(*i as f64),
        TomlValue::Float(f) => Value::Number(*f),
        TomlValue::Boolean(b) => Value::Boolean(*b),
        TomlValue::Array(arr) => {
            let mut vec = Vec::with_capacity(arr.len());
            vec.extend(arr.iter().map(toml_to_value));
            Value::Array(vec)
        }
        TomlValue::Table(table) => {
            let mut result =
                Frontmatter(HashMap::with_capacity(table.len()));
            for (k, v) in table {
                let _ = result
                    .0
                    .insert(optimise_string(k), toml_to_value(v));
            }
            Value::Object(Box::new(result))
        }
        TomlValue::Datetime(dt) => Value::String(dt.to_string()),
    }
}

/// Serialises a `Frontmatter` object into a TOML string.
///
/// # Arguments
///
/// * `front_matter` - The `Frontmatter` object to serialise.
///
/// # Returns
///
/// A `Result` containing the serialised TOML string or a `Error`.
fn to_toml(front_matter: &Frontmatter) -> Result<String, Error> {
    toml::to_string(&front_matter.0)
        .map_err(|e| Error::ConversionError(e.to_string()))
}

// JSON Implementation
// -------------------

/// Parses a JSON string into a `Frontmatter` object.
///
/// # Arguments
///
/// * `raw` - The raw JSON string.
///
/// # Returns
///
/// A `Result` containing the parsed `Frontmatter` or a `Error`.
fn parse_json(raw: &str) -> Result<Frontmatter, Error> {
    let json_value: JsonValue = serde_json::from_str(raw)
        .map_err(|e| Error::JsonParseError(Arc::new(e)))?;

    let capacity = match &json_value {
        JsonValue::Object(obj) => obj.len(),
        _ => 0,
    };

    let mut front_matter =
        Frontmatter(HashMap::with_capacity(capacity));

    if let JsonValue::Object(obj) = json_value {
        for (key, value) in obj {
            let _ = front_matter.0.insert(key, json_to_value(&value));
        }
    }

    Ok(front_matter)
}

/// Converts a `serde_json::Value` into a `Value`.
fn json_to_value(json: &JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Boolean(*b),
        JsonValue::Number(n) => n.as_i64().map_or_else(
            || {
                if let Some(f) = n.as_f64() {
                    Value::Number(f)
                } else {
                    Value::Number(0.0)
                }
            },
            |i| Value::Number(i as f64),
        ),
        JsonValue::String(s) => Value::String(optimise_string(s)),
        JsonValue::Array(arr) => {
            let mut vec = Vec::with_capacity(arr.len());
            vec.extend(arr.iter().map(json_to_value));
            Value::Array(vec)
        }
        JsonValue::Object(obj) => {
            let mut result =
                Frontmatter(HashMap::with_capacity(obj.len()));
            for (k, v) in obj {
                let _ = result
                    .0
                    .insert(optimise_string(k), json_to_value(v));
            }
            Value::Object(Box::new(result))
        }
    }
}

/// Optimised JSON serialisation with pre-allocated buffer.
///
/// # Arguments
///
/// * `front_matter` - The `Frontmatter` object to serialise.
///
/// # Returns
///
/// A `Result` containing the serialised JSON string or a `Error`.
fn to_json_optimised(
    front_matter: &Frontmatter,
) -> Result<String, Error> {
    let estimated_size = estimate_json_size(front_matter);
    let buf = Vec::with_capacity(estimated_size);
    let formatter = serde_json::ser::CompactFormatter;
    let mut ser =
        serde_json::Serializer::with_formatter(buf, formatter);

    front_matter
        .0
        .serialize(&mut ser)
        .map_err(|e| Error::ConversionError(e.to_string()))?;

    String::from_utf8(ser.into_inner())
        .map_err(|e| Error::ConversionError(e.to_string()))
}

// Validation and Utilities
// ------------------------

/// Validates a front matter structure against configured limits.
///
/// Checks:
/// - Maximum nesting depth.
/// - Maximum number of keys.
/// - Structure validity.
///
/// # Arguments
///
/// * `fm` - Reference to the front matter to validate.
/// * `max_depth` - Maximum allowed nesting depth.
/// * `max_keys` - Maximum allowed number of keys.
///
/// # Returns
///
/// `Ok(())` if validation passes, `Error` otherwise.
///
/// # Errors
///
/// Returns `Error` if:
/// - The number of keys exceeds `max_keys`.
/// - The nesting depth exceeds `max_depth`.
pub fn validate_frontmatter(
    fm: &Frontmatter,
    max_depth: usize,
    max_keys: usize,
) -> Result<(), Error> {
    if fm.0.len() > max_keys {
        return Err(Error::ContentTooLarge {
            size: fm.0.len(),
            max: max_keys,
        });
    }

    // Validate nesting depth
    for value in fm.0.values() {
        check_depth(value, 1, max_depth)?;
    }

    Ok(())
}

/// Recursively checks the nesting depth of a value.
///
/// # Arguments
///
/// * `value` - The `Value` to check.
/// * `current_depth` - The current depth of recursion.
/// * `max_depth` - The maximum allowed depth.
///
/// # Returns
///
/// `Ok(())` if the depth is within limits, `Error` otherwise.
fn check_depth(
    value: &Value,
    current_depth: usize,
    max_depth: usize,
) -> Result<(), Error> {
    if current_depth > max_depth {
        return Err(Error::NestingTooDeep {
            depth: current_depth,
            max: max_depth,
        });
    }

    match value {
        Value::Array(arr) => {
            for item in arr {
                check_depth(item, current_depth + 1, max_depth)?;
            }
        }
        Value::Object(obj) => {
            for v in obj.0.values() {
                check_depth(v, current_depth + 1, max_depth)?;
            }
        }
        _ => {}
    }

    Ok(())
}

/// Estimates the JSON string size for a front matter object.
///
/// Used for pre-allocating buffers in serialisation.
///
/// # Arguments
///
/// * `fm` - The `Frontmatter` object.
///
/// # Returns
///
/// An estimated size in bytes.
fn estimate_json_size(fm: &Frontmatter) -> usize {
    let mut size = 2; // {}
    for (k, v) in &fm.0 {
        size += k.len() + 3; // "key":
        size += estimate_value_size(v);
        size += 1; // ,
    }
    size
}

/// Estimates the serialised size of a value.
///
/// # Arguments
///
/// * `value` - The `Value` to estimate.
///
/// # Returns
///
/// An estimated size in bytes.
fn estimate_value_size(value: &Value) -> usize {
    match value {
        Value::Null => 4,                // null
        Value::String(s) => s.len() + 2, // "string"
        Value::Number(_) => 8,           // average number length
        Value::Boolean(_) => 5,          // false/true
        Value::Array(arr) => {
            2 + arr.iter().map(estimate_value_size).sum::<usize>() // []
        }
        Value::Object(obj) => estimate_json_size(obj),
        Value::Tagged(tag, val) => {
            tag.len() + 2 + estimate_value_size(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // Helper function for creating test data
    fn create_test_frontmatter() -> Frontmatter {
        let mut fm = Frontmatter::new();
        let _ = fm.insert(
            "string".to_string(),
            Value::String("test".to_string()),
        );
        let _ = fm.insert("number".to_string(), Value::Number(PI));
        let _ = fm.insert("boolean".to_string(), Value::Boolean(true));
        let _ = fm.insert(
            "array".to_string(),
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
        );
        fm
    }

    #[test]
    fn test_string_optimisation() {
        let short_str = "short";
        let long_str = "a".repeat(SMALL_STRING_SIZE + 1);

        let optimised_short = optimise_string(short_str);
        let optimised_long = optimise_string(&long_str);

        assert_eq!(optimised_short, short_str);
        assert_eq!(optimised_long, long_str);
        assert!(optimised_long.capacity() >= long_str.len());
    }

    #[test]
    fn test_validation() {
        // Test max keys validation
        let mut large_fm = Frontmatter::new();
        for i in 0..MAX_KEYS + 1 {
            let _ = large_fm.insert(
                i.to_string(),
                Value::String("value".to_string()),
            );
        }
        assert!(validate_frontmatter(
            &large_fm,
            MAX_NESTING_DEPTH,
            MAX_KEYS
        )
        .is_err());

        // Test nesting depth validation
        let mut nested_fm = Frontmatter::new();
        let mut current = Value::Null;
        for _ in 0..MAX_NESTING_DEPTH + 1 {
            current = Value::Object(Box::new(Frontmatter(
                [("nested".to_string(), current)].into_iter().collect(),
            )));
        }
        let _ = nested_fm.insert("deep".to_string(), current);
        assert!(validate_frontmatter(
            &nested_fm,
            MAX_NESTING_DEPTH,
            MAX_KEYS
        )
        .is_err());
    }

    #[test]
    fn test_format_roundtrip() {
        let original = create_test_frontmatter();

        // Test YAML roundtrip
        let yaml = to_string(&original, Format::Yaml).unwrap();
        let from_yaml = parse(&yaml, Format::Yaml).unwrap();
        assert_eq!(original, from_yaml);

        // Test TOML roundtrip
        let toml = to_string(&original, Format::Toml).unwrap();
        let from_toml = parse(&toml, Format::Toml).unwrap();
        assert_eq!(original, from_toml);

        // Test JSON roundtrip
        let json = to_string(&original, Format::Json).unwrap();
        let from_json = parse(&json, Format::Json).unwrap();
        assert_eq!(original, from_json);
    }

    #[test]
    fn test_parse_options() {
        let yaml = r"
        nested:
          level1:
            level2:
              value: test
        ";

        // Test with default options
        assert!(parse_with_options(yaml, Format::Yaml, None).is_ok());

        // Test with restricted depth
        let restricted_options = ParseOptions {
            max_depth: 2,
            max_keys: MAX_KEYS,
            validate: true,
        };
        assert!(parse_with_options(
            yaml,
            Format::Yaml,
            Some(restricted_options)
        )
        .is_err());
    }

    #[test]
    fn test_error_handling() {
        // Test invalid YAML
        let invalid_yaml = "test: : invalid";
        assert!(matches!(
            parse(invalid_yaml, Format::Yaml),
            Err(Error::YamlParseError { .. })
        ));

        // Test invalid TOML
        let invalid_toml = "test = = invalid";
        assert!(matches!(
            parse(invalid_toml, Format::Toml),
            Err(Error::TomlParseError(_))
        ));

        // Test invalid JSON
        let invalid_json = "{invalid}";
        assert!(matches!(
            parse(invalid_json, Format::Json),
            Err(Error::JsonParseError(_))
        ));
    }

    #[test]
    fn test_size_estimation() {
        let fm = create_test_frontmatter();
        let estimated_size = estimate_json_size(&fm);
        let actual_json = to_string(&fm, Format::Json).unwrap();

        // Estimated size should be reasonably close to actual size
        assert!(estimated_size >= actual_json.len());
        assert!(estimated_size <= actual_json.len() * 2);
    }

    #[test]
    fn test_large_integer_conversion() {
        let large_i64 = 1_i64 << 53;
        let fallback_value = Value::Number(0.0);

        assert_eq!(
            yaml_to_value(&YamlValue::Number(large_i64.into())),
            fallback_value
        );
    }
}
