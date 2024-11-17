// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Frontmatter Generator
//!
//! `frontmatter-gen` is a CLI tool designed for extracting, validating, and managing front matter
//! from content files used in static site generation. It provides tools for processing front matter
//! in various formats (YAML, TOML, JSON) and building static sites with customizable templates.
//!
//! ## Features
//!
//! - **Validation**: Ensure required front matter fields are present and correctly formatted.
//! - **Extraction**: Extract front matter in various formats and output it to a file or stdout.
//! - **Site Generation**: Build static sites with configurable content, output, and template directories.
//!
//! ## Usage
//!
//! Use the command-line interface to interact with the tool:
//!
//! ```bash
//! frontmatter-gen validate --file content.md --required title date author
//! frontmatter-gen extract --file content.md --format yaml --output frontmatter.yaml
//! frontmatter-gen build --content-dir content --output-dir public --template-dir templates
//! ```
//!
//! ## Configuration
//!
//! The tool optionally reads from a `frontmatter-gen.toml` configuration file for defaults,
//! such as required fields for validation, or directories for content and templates.

use anyhow::{Context, Result};
use clap::{Arg, Command};
use frontmatter_gen::{engine::Engine, to_format, Config, Format};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Custom error types for front matter validation.
#[derive(Error, Debug)]
pub enum FrontmatterError {
    #[error("Missing required field: {0}")]
    /// Error for missing required fields in front matter.
    MissingField(String),
    #[error("Invalid date format: {0}")]
    /// Error for invalid date format in front matter.
    InvalidDate(String),
    #[error("Invalid pattern for field '{0}': {1}")]
    /// Error for fields that do not match a specified pattern.
    InvalidPattern(String, String),
}

/// Configuration structure for `frontmatter-gen`.
#[derive(Debug, Deserialize, Default)]
struct AppConfig {
    validate: Option<ValidationConfig>,
    extract: Option<ExtractConfig>,
    build: Option<BuildConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct ValidationConfig {
    required_fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
struct ExtractConfig {
    default_format: Option<String>,
    output: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct BuildConfig {
    content_dir: Option<String>,
    output_dir: Option<String>,
    template_dir: Option<String>,
}

/// Parses command-line arguments and loads optional configuration from `frontmatter-gen.toml`.
fn load_configuration() -> Result<(clap::ArgMatches, AppConfig)> {
    let matches = Command::new("frontmatter-gen")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool for front matter extraction, validation, and static site generation")
        .subcommand_required(true)
        .subcommand(
            Command::new("validate")
                .about("Validates front matter in a file")
                .arg(
                    Arg::new("file")
                        .required(true)
                        .help("Path to the file to validate"),
                )
                .arg(
                    Arg::new("required")
                        .long("required")
                        .num_args(1..) // One or more required fields
                        .help("List of required fields"),
                ),
        )
        .subcommand(
            Command::new("extract")
                .about("Extracts front matter from a file")
                .arg(
                    Arg::new("file")
                        .required(true)
                        .help("Path to the file to extract from"),
                )
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Output format (yaml, toml, json)"),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .help("File to write the extracted front matter to"),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Builds a static site from the given directories")
                .arg(
                    Arg::new("content-dir")
                        .long("content-dir")
                        .help("Directory containing site content"),
                )
                .arg(
                    Arg::new("output-dir")
                        .long("output-dir")
                        .help("Directory where the generated site will be output"),
                )
                .arg(
                    Arg::new("template-dir")
                        .long("template-dir")
                        .help("Directory containing site templates"),
                ),
        )
        .get_matches();

    // Load configuration file if present
    let config: AppConfig =
        if Path::new("frontmatter-gen.toml").exists() {
            let content = fs::read_to_string("frontmatter-gen.toml")?;
            toml::from_str(&content)?
        } else {
            AppConfig::default()
        };

    Ok((matches, config))
}

#[tokio::main]
async fn main() -> Result<()> {
    let (matches, config) = load_configuration()?;

    match matches.subcommand() {
        Some(("validate", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").unwrap();
            let required_fields = sub_matches
                .get_many::<String>("required")
                .map(|vals| {
                    vals.flat_map(|val| {
                        val.split(',').map(String::from)
                    })
                    .collect::<Vec<_>>()
                })
                .or_else(|| {
                    config.validate.as_ref()?.required_fields.clone()
                })
                .unwrap_or_else(|| {
                    vec![
                        "title".to_string(),
                        "date".to_string(),
                        "author".to_string(),
                    ]
                });

            // Convert Vec<String> to Vec<&str>
            let required_fields: Vec<&str> =
                required_fields.iter().map(String::as_str).collect();

            // Pass slice to validate_command
            validate_command(Path::new(file), &required_fields).await?;
        }
        Some(("extract", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").unwrap();
            let format = sub_matches
                .get_one::<String>("format")
                .map(String::as_str)
                .or(config
                    .extract
                    .as_ref()
                    .and_then(|c| c.default_format.as_deref()))
                .unwrap_or("yaml");

            let output = sub_matches
                .get_one::<String>("output")
                .map(String::as_str)
                .or_else(|| {
                    config
                        .extract
                        .as_ref()
                        .and_then(|c| c.output.as_deref())
                })
                .map(PathBuf::from);

            extract_command(Path::new(file), format, output).await?;
        }
        Some(("build", sub_matches)) => {
            let content_dir = sub_matches
                .get_one::<String>("content-dir")
                .map(String::as_str)
                .or_else(|| {
                    config
                        .build
                        .as_ref()
                        .and_then(|c| c.content_dir.as_deref())
                })
                .unwrap_or("content");
            let output_dir = sub_matches
                .get_one::<String>("output-dir")
                .map(String::as_str)
                .or_else(|| {
                    config
                        .build
                        .as_ref()
                        .and_then(|c| c.output_dir.as_deref())
                })
                .unwrap_or("public");
            let template_dir = sub_matches
                .get_one::<String>("template-dir")
                .map(String::as_str)
                .or_else(|| {
                    config
                        .build
                        .as_ref()
                        .and_then(|c| c.template_dir.as_deref())
                })
                .unwrap_or("templates");

            build_command(
                Path::new(content_dir),
                Path::new(output_dir),
                Path::new(template_dir),
            )
            .await?;
        }
        _ => unreachable!(
            "Clap should ensure that a valid subcommand is provided"
        ),
    }

    Ok(())
}

/// Validates front matter in a file.
async fn validate_command(
    file: &Path,
    required_fields: &[&str],
) -> Result<()> {
    // Read the file content
    let content = tokio::fs::read_to_string(file)
        .await
        .context("Failed to read input file")?;

    // Debugging log for content
    // eprintln!("Content: {:?}", content);

    // Extract front matter using frontmatter_gen
    let (frontmatter, _) = frontmatter_gen::extract(&content)
        .context("Failed to extract front matter")?;

    // Debugging log for extracted front matter
    // eprintln!("Extracted Frontmatter: {:?}", frontmatter);

    // Validate each required field
    for &field in required_fields {
        if !frontmatter.contains_key(field) {
            return Err(anyhow::anyhow!(
                "Validation failed: Missing required field '{}'",
                field
            ));
        }
    }

    println!("Validation successful: All required fields are present.");
    Ok(())
}

/// Extracts front matter from a file and outputs it in the specified format.
///
/// This function reads the input file, extracts the front matter,
/// formats it according to the specified format, and optionally writes
/// it to an output file. If no output file is specified, the formatted
/// front matter is printed to the console.
///
/// # Arguments
///
/// * `input` - The path to the input file.
/// * `format` - The format to output the front matter (e.g., "yaml", "toml", "json").
/// * `output` - An optional path to the output file where the front matter will be saved.
///
/// # Errors
///
/// Returns an error if:
/// - The input file cannot be read.
/// - The front matter cannot be extracted or formatted.
/// - Writing to the output file fails.
///
/// # Examples
///
/// ```
/// extract_command(
///     Path::new("content.md"),
///     "yaml",
///     Some(PathBuf::from("frontmatter.yaml"))
/// ).await?;
/// ```
async fn extract_command(
    input: &Path,
    format: &str,
    output: Option<PathBuf>,
) -> Result<()> {
    // Read the content of the input file
    let content =
        tokio::fs::read_to_string(input).await.with_context(|| {
            format!("Failed to read input file: {:?}", input)
        })?;

    // Extract the front matter and the remaining content
    let (frontmatter, remaining_content) =
        frontmatter_gen::extract(&content)
            .context("Failed to extract front matter from the file")?;

    // Determine the desired format and convert the front matter
    let output_format = match format {
        "yaml" => Format::Yaml,
        "toml" => Format::Toml,
        "json" => Format::Json,
        other => {
            return Err(anyhow::anyhow!(
                "Unsupported format specified: '{}'. Supported formats are: yaml, toml, json.",
                other
            ));
        }
    };

    let formatted_frontmatter = to_format(&frontmatter, output_format)
        .context("Failed to format the extracted front matter")?;

    // Write the front matter to the specified output file or print to console
    if let Some(output_path) = output {
        fs::write(&output_path, &formatted_frontmatter).with_context(
            || {
                format!(
                    "Failed to write to output file: {:?}",
                    output_path
                )
            },
        )?;
        println!(
            "Front matter successfully written to output file: {:?}",
            output_path
        );
    } else {
        println!("Extracted Front Matter (format: {}):", format);
        println!("{}", formatted_frontmatter);
    }

    // Print the remaining content to the console
    println!("\nRemaining Content:\n{}", remaining_content);

    Ok(())
}

/// Builds a static site.
async fn build_command(
    content_dir: &Path,
    output_dir: &Path,
    template_dir: &Path,
) -> Result<()> {
    let config = Config::builder()
        .site_name("my-site")
        .content_dir(content_dir)
        .output_dir(output_dir)
        .template_dir(template_dir)
        .build()?;

    let engine = Engine::new()?;
    engine.generate(&config).await?;

    println!("Site built successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_command_all_fields_present() {
        let content = r#"---
title: "My Title"
date: "2025-09-09"
author: "Jane Doe"
---"#;

        let test_file_path = "test.md";

        // Write the test file
        let write_result =
            tokio::fs::write(test_file_path, content).await;
        assert!(
            write_result.is_ok(),
            "Failed to write test file: {:?}",
            write_result
        );

        // Debugging: Print the content of the test file
        let read_content =
            tokio::fs::read_to_string(test_file_path).await;
        assert!(
            read_content.is_ok(),
            "Failed to read test file: {:?}",
            read_content
        );
        println!("Content of test file:\n{}", read_content.unwrap());

        // Convert Vec<String> to Vec<&str>
        let required_fields = vec!["title", "date", "author"];

        // Run the validate_command function
        let result = validate_command(
            Path::new(test_file_path),
            &required_fields,
        )
        .await;

        // Debugging: Check the result of the validation
        if let Err(e) = &result {
            println!("Validation failed with error: {:?}", e);
        }

        assert!(
            result.is_ok(),
            "Validation failed with error: {:?}",
            result
        );

        // Ensure the test file is removed
        if Path::new(test_file_path).exists() {
            let remove_result =
                tokio::fs::remove_file(test_file_path).await;
            assert!(
                remove_result.is_ok(),
                "Failed to remove test file: {:?}",
                remove_result
            );
        } else {
            println!(
                "Test file '{}' does not exist during cleanup.",
                test_file_path
            );
        }
    }

    #[tokio::test]
    async fn test_extract_command_to_stdout() {
        let test_file_path = "test.md";
        let content = r#"---
title: "My Title"
date: "2025-09-09"
author: "Jane Doe"
---"#;

        // Write the test file
        let write_result =
            tokio::fs::write(test_file_path, content).await;
        assert!(
            write_result.is_ok(),
            "Failed to write test file: {:?}",
            write_result
        );

        // Ensure the file exists
        assert!(
            Path::new(test_file_path).exists(),
            "The test file does not exist after creation."
        );

        // Run the extract_command function
        let result =
            extract_command(Path::new(test_file_path), "yaml", None)
                .await;
        assert!(
            result.is_ok(),
            "Extraction failed with error: {:?}",
            result
        );

        // Cleanup: Ensure the file is removed after the test
        if Path::new(test_file_path).exists() {
            let remove_result =
                tokio::fs::remove_file(test_file_path).await;
            assert!(
                remove_result.is_ok(),
                "Failed to remove test file: {:?}",
                remove_result
            );
        } else {
            // Log a message instead of panicking if the file doesn't exist
            eprintln!(
            "Test file '{}' was already removed or not found during cleanup.",
            test_file_path
        );
        }
    }

    #[tokio::test]
    async fn test_build_command_missing_dirs() {
        let content_dir = Path::new("missing_content");
        let output_dir = Path::new("missing_public");
        let template_dir = Path::new("missing_templates");

        // Run the build command, which is expected to fail
        let result =
            build_command(content_dir, output_dir, template_dir).await;
        assert!(result.is_err(), "Expected an error but got success");

        // Cleanup: Ensure the directories are removed after the test
        if content_dir.exists() {
            let remove_result =
                tokio::fs::remove_dir_all(content_dir).await;
            assert!(
                remove_result.is_ok(),
                "Failed to remove content directory: {:?}",
                remove_result
            );
        }

        if output_dir.exists() {
            let remove_result =
                tokio::fs::remove_dir_all(output_dir).await;
            assert!(
                remove_result.is_ok(),
                "Failed to remove output directory: {:?}",
                remove_result
            );
        }

        if template_dir.exists() {
            let remove_result =
                tokio::fs::remove_dir_all(template_dir).await;
            assert!(
                remove_result.is_ok(),
                "Failed to remove template directory: {:?}",
                remove_result
            );
        }
    }
}
