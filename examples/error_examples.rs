// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Error Handling Examples
//!
//! This example demonstrates the usage of the error types and error handling
//! functionality in the FrontMatterGen library. It covers various error scenarios,
//! error conversion, and error handling for frontmatter parsing, conversion, and extraction.

#![allow(missing_docs)]

use frontmatter_gen::error::FrontmatterError;

/// Entry point for the FrontMatterGen error handling examples.
///
/// This function runs various examples demonstrating error creation, conversion,
/// and handling for different scenarios in the FrontMatterGen library.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Error Handling Examples\n");

    yaml_parse_error_example()?;
    toml_parse_error_example()?;
    json_parse_error_example()?;
    conversion_error_example()?;
    unsupported_format_error_example()?;
    extraction_error_example()?;

    // Add SSG-specific examples when the feature is enabled
    #[cfg(feature = "ssg")]
    {
        ssg_specific_error_example()?;
    }

    println!(
        "\n🎉  All error handling examples completed successfully!"
    );

    Ok(())
}

/// Demonstrates handling of YAML parsing errors.
///
/// This function attempts to parse invalid YAML content and shows
/// how FrontMatterGen handles parsing errors.
fn yaml_parse_error_example() -> Result<(), FrontmatterError> {
    println!("🦀 YAML Parse Error Example");
    println!("---------------------------------------------");

    let invalid_yaml = "invalid: yaml: data";
    let result: Result<serde_yml::Value, _> =
        serde_yml::from_str(invalid_yaml);

    match result {
        Ok(_) => println!(
            "    ❌  Unexpected success in parsing invalid YAML"
        ),
        Err(e) => {
            let error = FrontmatterError::YamlParseError { source: e };
            println!(
                "    ✅  Successfully caught YAML parse error: {}",
                error
            );
        }
    }

    Ok(())
}

/// Demonstrates handling of TOML parsing errors.
fn toml_parse_error_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 TOML Parse Error Example");
    println!("---------------------------------------------");

    let invalid_toml = "invalid = toml data";
    match toml::from_str::<toml::Value>(invalid_toml) {
        Ok(_) => println!(
            "    ❌  Unexpected success in parsing invalid TOML"
        ),
        Err(e) => {
            let error = FrontmatterError::TomlParseError(e);
            println!(
                "    ✅  Successfully caught TOML parse error: {}",
                error
            );
        }
    }

    Ok(())
}

/// Demonstrates handling of JSON parsing errors.
fn json_parse_error_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 JSON Parse Error Example");
    println!("---------------------------------------------");

    let invalid_json = "{ invalid json }";
    match serde_json::from_str::<serde_json::Value>(invalid_json) {
        Ok(_) => println!(
            "    ❌  Unexpected success in parsing invalid JSON"
        ),
        Err(e) => {
            let error = FrontmatterError::JsonParseError(e);
            println!(
                "    ✅  Successfully caught JSON parse error: {}",
                error
            );
        }
    }

    Ok(())
}

/// Demonstrates handling of frontmatter conversion errors.
fn conversion_error_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 Conversion Error Example");
    println!("---------------------------------------------");

    let error_message = "Failed to convert frontmatter data";
    let error =
        FrontmatterError::ConversionError(error_message.to_string());
    println!("    ✅  Created Conversion Error: {}", error);

    Ok(())
}

/// Demonstrates handling of unsupported format errors.
fn unsupported_format_error_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 Unsupported Format Error Example");
    println!("---------------------------------------------");

    let line = 42;
    let error = FrontmatterError::unsupported_format(line);
    println!(
        "    ✅  Created Unsupported Format Error for line {}: {}",
        line, error
    );

    Ok(())
}

/// Demonstrates handling of extraction errors.
fn extraction_error_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 Extraction Error Example");
    println!("---------------------------------------------");

    let error_message = "Failed to extract frontmatter";
    let error =
        FrontmatterError::ExtractionError(error_message.to_string());
    println!("    ✅  Created Extraction Error: {}", error);

    Ok(())
}

/// Demonstrates SSG-specific error handling.
/// This function is only available when the "ssg" feature is enabled.
#[cfg(feature = "ssg")]
fn ssg_specific_error_example() -> Result<(), FrontmatterError> {
    println!("\n🦀 SSG-Specific Error Example");
    println!("---------------------------------------------");

    // Example of URL validation error (SSG-specific)
    let invalid_url = "not-a-url";
    let error = FrontmatterError::InvalidUrl(invalid_url.to_string());
    println!("    ✅  Created URL Validation Error: {}", error);

    // Example of language code error (SSG-specific)
    let invalid_lang = "invalid";
    let error =
        FrontmatterError::InvalidLanguage(invalid_lang.to_string());
    println!("    ✅  Created Language Code Error: {}", error);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_error_handling(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Test core functionality
        yaml_parse_error_example()?;
        toml_parse_error_example()?;
        json_parse_error_example()?;
        Ok(())
    }

    // SSG-specific tests
    #[cfg(feature = "ssg")]
    mod ssg_tests {
        use super::*;

        #[test]
        fn test_ssg_error_handling(
        ) -> Result<(), Box<dyn std::error::Error>> {
            ssg_specific_error_example()?;
            Ok(())
        }
    }
}
