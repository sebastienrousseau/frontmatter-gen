//! # Frontmatter Parser and Serialiser Module
//!
//! This module provides robust functionality for parsing and serialising frontmatter
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

use serde::Serialize;
use serde_json::Value as JsonValue;
use serde_yml::Value as YmlValue;
use std::collections::HashMap;
use toml::Value as TomlValue;

use crate::{
    error::FrontmatterError, types::Frontmatter, Format, Value,
};

// Constants for optimisation and validation
const SMALL_STRING_SIZE: usize = 24;
const MAX_NESTING_DEPTH: usize = 32;
const MAX_KEYS: usize = 1000;

/// Options for controlling parsing behaviour
#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    /// Maximum allowed nesting depth
    pub max_depth: usize,
    /// Maximum allowed number of keys
    pub max_keys: usize,
    /// Whether to validate structure
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

/// Optimises string storage based on length
///
/// For strings shorter than `SMALL_STRING_SIZE`, uses standard allocation.
/// For longer strings, pre-allocates exact capacity to avoid reallocations.
///
/// # Arguments
///
/// * `s` - The input string slice to optimise
///
/// # Returns
///
/// An optimised owned String
#[inline]
fn optimize_string(s: &str) -> String {
    if s.len() <= SMALL_STRING_SIZE {
        s.to_string()
    } else {
        let mut string = String::with_capacity(s.len());
        string.push_str(s);
        string
    }
}

/// Parses raw frontmatter string into a `Frontmatter` object based on the specified format.
///
/// This function attempts to parse the provided string into a structured `Frontmatter`
/// object according to the specified format. It performs validation by default
/// and optimises memory allocation where possible.
///
/// # Arguments
///
/// * `raw_frontmatter` - A string slice containing the raw frontmatter content
/// * `format` - The `Format` enum specifying the desired format
/// * `options` - Optional parsing options for controlling validation and limits
///
/// # Returns
///
/// A `Result` containing either the parsed `Frontmatter` object or a `FrontmatterError`
///
/// # Examples
///
/// ```rust
/// use frontmatter_gen::{Format, parser};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let yaml = "title: My Post\ndate: 2025-09-09\n";
/// let frontmatter = parser::parse_with_options(
///     yaml,
///     Format::Yaml,
///     None
/// )?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns `FrontmatterError` if:
/// - The input is not valid in the specified format
/// - The structure exceeds configured limits
/// - The format is unsupported
pub fn parse_with_options(
    raw_frontmatter: &str,
    format: Format,
    options: Option<ParseOptions>,
) -> Result<Frontmatter, FrontmatterError> {
    let options = options.unwrap_or_default();

    // Validate format assumptions against the raw input
    if format == Format::Yaml && raw_frontmatter.starts_with("---") {
        eprintln!("Warning: Format set to YAML but input does not start with '---'");
    }
    if format == Format::Toml && !raw_frontmatter.contains('=') {
        return Err(FrontmatterError::ConversionError(
            "Format set to TOML but input does not contain '=' signs."
                .to_string(),
        ));
    }
    if format == Format::Json && !raw_frontmatter.starts_with('{') {
        return Err(FrontmatterError::ConversionError(
            "Format set to JSON but input does not start with '{'."
                .to_string(),
        ));
    }

    let frontmatter = match format {
        Format::Yaml => parse_yaml(raw_frontmatter).map_err(|e| {
            eprintln!("YAML parsing failed: {}", e);
            e
        })?,
        Format::Toml => parse_toml(raw_frontmatter).map_err(|e| {
            eprintln!("TOML parsing failed: {}", e);
            e
        })?,
        Format::Json => parse_json(raw_frontmatter).map_err(|e| {
            eprintln!("JSON parsing failed: {}", e);
            e
        })?,
        Format::Unsupported => {
            let err_msg = "Unsupported format provided".to_string();
            eprintln!("{}", err_msg);
            return Err(FrontmatterError::ConversionError(err_msg));
        }
    };

    // Perform validation if the options specify it
    if options.validate {
        println!(
            "Validating frontmatter with max_depth={} and max_keys={}",
            options.max_depth, options.max_keys
        );
        validate_frontmatter(
            &frontmatter,
            options.max_depth,
            options.max_keys,
        )
        .map_err(|e| {
            eprintln!("Validation failed: {}", e);
            e
        })?;
    }

    Ok(frontmatter)
}

/// Convenience wrapper around `parse_with_options` using default options
///
/// # Arguments
///
/// * `raw_frontmatter` - A string slice containing the raw frontmatter content
/// * `format` - The `Format` enum specifying the desired format
///
/// # Returns
///
/// A `Result` containing either the parsed `Frontmatter` object or a `FrontmatterError`
pub fn parse(
    raw_frontmatter: &str,
    format: Format,
) -> Result<Frontmatter, FrontmatterError> {
    parse_with_options(raw_frontmatter, format, None)
}

/// Converts a `Frontmatter` object to a string representation in the specified format.
///
/// Performs optimised serialisation with pre-allocated buffers where possible.
///
/// # Arguments
///
/// * `frontmatter` - Reference to the `Frontmatter` object to serialise
/// * `format` - The target format for serialisation
///
/// # Returns
///
/// A `Result` containing the serialised string or a `FrontmatterError`
///
pub fn to_string(
    frontmatter: &Frontmatter,
    format: Format,
) -> Result<String, FrontmatterError> {
    match format {
        Format::Yaml => to_yaml(frontmatter),
        Format::Toml => to_toml(frontmatter),
        Format::Json => to_json_optimized(frontmatter),
        Format::Unsupported => Err(FrontmatterError::ConversionError(
            "Unsupported format".to_string(),
        )),
    }
}

// YAML Implementation
// -----------------
fn parse_yaml(raw: &str) -> Result<Frontmatter, FrontmatterError> {
    // Log the raw input for debugging
    // eprintln!("Debug: Raw input received by parse_yaml: {}", raw);

    // Parse the YAML content into a serde_yaml::Value
    let yml_value: YmlValue = serde_yml::from_str(raw)
        .map_err(|e| FrontmatterError::YamlParseError { source: e })?;

    // Prepare the frontmatter container
    let capacity = yml_value.as_mapping().map_or(0, |m| m.len());
    let mut frontmatter = Frontmatter(HashMap::with_capacity(capacity));

    // Convert the YAML mapping into the frontmatter structure
    if let YmlValue::Mapping(mapping) = yml_value {
        for (key, value) in mapping {
            if let YmlValue::String(k) = key {
                let _ = frontmatter.insert(k, yml_to_value(&value));
            } else {
                // Log a warning for non-string keys
                eprintln!("Warning: Non-string key ignored in YAML frontmatter");
            }
        }
    } else {
        return Err(FrontmatterError::ParseError(
            "YAML frontmatter is not a valid mapping".to_string(),
        ));
    }

    Ok(frontmatter)
}

fn yml_to_value(yml: &YmlValue) -> Value {
    match yml {
        YmlValue::Null => Value::Null,
        YmlValue::Bool(b) => Value::Boolean(*b),
        YmlValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(i as f64)
            } else if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::Number(0.0)
            }
        }
        YmlValue::String(s) => Value::String(optimize_string(s)),
        YmlValue::Sequence(seq) => {
            let mut vec = Vec::with_capacity(seq.len());
            vec.extend(seq.iter().map(yml_to_value));
            Value::Array(vec)
        }
        YmlValue::Mapping(map) => {
            let mut result =
                Frontmatter(HashMap::with_capacity(map.len()));
            for (k, v) in map {
                if let YmlValue::String(key) = k {
                    let _ = result
                        .0
                        .insert(optimize_string(key), yml_to_value(v));
                }
            }
            Value::Object(Box::new(result))
        }
        YmlValue::Tagged(tagged) => Value::Tagged(
            optimize_string(&tagged.tag.to_string()),
            Box::new(yml_to_value(&tagged.value)),
        ),
    }
}

fn to_yaml(
    frontmatter: &Frontmatter,
) -> Result<String, FrontmatterError> {
    serde_yml::to_string(&frontmatter.0)
        .map_err(|e| FrontmatterError::ConversionError(e.to_string()))
}

// TOML Implementation
// -----------------

fn parse_toml(raw: &str) -> Result<Frontmatter, FrontmatterError> {
    let toml_value: TomlValue =
        raw.parse().map_err(FrontmatterError::TomlParseError)?;

    let capacity = match &toml_value {
        TomlValue::Table(table) => table.len(),
        _ => 0,
    };

    let mut frontmatter = Frontmatter(HashMap::with_capacity(capacity));

    if let TomlValue::Table(table) = toml_value {
        for (key, value) in table {
            let _ = frontmatter.0.insert(key, toml_to_value(&value));
        }
    }

    Ok(frontmatter)
}

fn toml_to_value(toml: &TomlValue) -> Value {
    match toml {
        TomlValue::String(s) => Value::String(optimize_string(s)),
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
                    .insert(optimize_string(k), toml_to_value(v));
            }
            Value::Object(Box::new(result))
        }
        TomlValue::Datetime(dt) => Value::String(dt.to_string()),
    }
}

fn to_toml(
    frontmatter: &Frontmatter,
) -> Result<String, FrontmatterError> {
    toml::to_string(&frontmatter.0)
        .map_err(|e| FrontmatterError::ConversionError(e.to_string()))
}

// JSON Implementation
// -----------------

fn parse_json(raw: &str) -> Result<Frontmatter, FrontmatterError> {
    let json_value: JsonValue = serde_json::from_str(raw)
        .map_err(FrontmatterError::JsonParseError)?;

    let capacity = match &json_value {
        JsonValue::Object(obj) => obj.len(),
        _ => 0,
    };

    let mut frontmatter = Frontmatter(HashMap::with_capacity(capacity));

    if let JsonValue::Object(obj) = json_value {
        for (key, value) in obj {
            let _ = frontmatter.0.insert(key, json_to_value(&value));
        }
    }

    Ok(frontmatter)
}

fn json_to_value(json: &JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Boolean(*b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(i as f64)
            } else if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::Number(0.0)
            }
        }
        JsonValue::String(s) => Value::String(optimize_string(s)),
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
                    .insert(optimize_string(k), json_to_value(v));
            }
            Value::Object(Box::new(result))
        }
    }
}

/// Optimised JSON serialisation with pre-allocated buffer
fn to_json_optimized(
    frontmatter: &Frontmatter,
) -> Result<String, FrontmatterError> {
    let estimated_size = estimate_json_size(frontmatter);
    let buf = Vec::with_capacity(estimated_size);
    let formatter = serde_json::ser::CompactFormatter;
    let mut ser =
        serde_json::Serializer::with_formatter(buf, formatter);

    frontmatter.0.serialize(&mut ser).map_err(|e| {
        FrontmatterError::ConversionError(e.to_string())
    })?;

    String::from_utf8(ser.into_inner())
        .map_err(|e| FrontmatterError::ConversionError(e.to_string()))
}

// Validation and Utilities
// -----------------------

/// Validates a frontmatter structure against configured limits.
///
/// Checks:
/// - Maximum nesting depth
/// - Maximum number of keys
/// - Structure validity
///
/// # Arguments
///
/// * `fm` - Reference to the frontmatter to validate
/// * `max_depth` - Maximum allowed nesting depth
/// * `max_keys` - Maximum allowed number of keys
///
/// # Returns
///
/// `Ok(())` if validation passes, `FrontmatterError` otherwise
fn validate_frontmatter(
    fm: &Frontmatter,
    max_depth: usize,
    max_keys: usize,
) -> Result<(), FrontmatterError> {
    if fm.0.len() > max_keys {
        return Err(FrontmatterError::ContentTooLarge {
            size: fm.0.len(),
            max: max_keys,
        });
    }

    // Validate nesting depth
    for value in fm.0.values() {
        check_depth(value, 0, max_depth)?;
    }

    Ok(())
}

/// Recursively checks the nesting depth of a value
fn check_depth(
    value: &Value,
    current_depth: usize,
    max_depth: usize,
) -> Result<(), FrontmatterError> {
    if current_depth > max_depth {
        return Err(FrontmatterError::NestingTooDeep {
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

/// Estimates the JSON string size for a frontmatter object
///
/// Used for pre-allocating buffers in serialisation
fn estimate_json_size(fm: &Frontmatter) -> usize {
    let mut size = 2; // {}
    for (k, v) in &fm.0 {
        size += k.len() + 3; // "key":
        size += estimate_value_size(v);
        size += 1; // ,
    }
    size
}

/// Estimates the serialised size of a value
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
    fn test_string_optimization() {
        let short_str = "short";
        let long_str = "a".repeat(SMALL_STRING_SIZE + 1);

        let optimized_short = optimize_string(short_str);
        let optimized_long = optimize_string(&long_str);

        assert_eq!(optimized_short, short_str);
        assert_eq!(optimized_long, long_str);
        assert!(optimized_long.capacity() >= long_str.len());
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
        let yaml = r#"
        nested:
          level1:
            level2:
              value: test
        "#;

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
            Err(FrontmatterError::YamlParseError { .. })
        ));

        // Test invalid TOML
        let invalid_toml = "test = = invalid";
        assert!(matches!(
            parse(invalid_toml, Format::Toml),
            Err(FrontmatterError::TomlParseError(_))
        ));

        // Test invalid JSON
        let invalid_json = "{invalid}";
        assert!(matches!(
            parse(invalid_json, Format::Json),
            Err(FrontmatterError::JsonParseError(_))
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
}
