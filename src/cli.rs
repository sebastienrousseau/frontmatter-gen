// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Command Line Interface Module
//!
//! This module provides the command-line interface functionality for the frontmatter-gen library.
//! It handles parsing of command-line arguments and executing the corresponding operations.
//!
//! ## Features
//!
//! - Command-line argument parsing using clap
//! - Subcommands for different operations (extract, validate)
//! - Error handling and user-friendly messages
//!
//! ## Usage
//!
//! ```bash
//! # Extract frontmatter
//! cargo run --features="cli" extract input.md --format yaml
//!
//! # Validate frontmatter
//! cargo run --features="cli" validate input.md --required title,date
//! ```

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{extract, to_format, Format};

/// Command line arguments parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI commands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract frontmatter from a file
    Extract {
        /// Input file path
        #[arg(required = true)]
        input: PathBuf,

        /// Output format (yaml, toml, json)
        #[arg(short, long, default_value = "yaml")]
        format: String,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate frontmatter in a file
    Validate {
        /// Input file path
        #[arg(required = true)]
        input: PathBuf,

        /// Required fields (comma-separated)
        #[arg(short, long)]
        required: Option<String>,
    },
}

impl Cli {
    /// Process CLI commands
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File operations fail
    /// - Frontmatter parsing fails
    /// - Validation fails
    /// - Format conversion fails
    pub async fn process(&self) -> Result<()> {
        match &self.command {
            Commands::Extract {
                input,
                format,
                output,
            } => process_extract(input, format, output).await,
            Commands::Validate { input, required } => {
                process_validate(input, required).await
            }
        }
    }
}

/// Process extract command
///
/// # Arguments
///
/// * `input` - Path to input file
/// * `format` - Output format
/// * `output` - Optional output file path
///
/// # Errors
///
/// Returns an error if:
/// - Input file cannot be read
/// - Frontmatter parsing fails
/// - Format conversion fails
/// - Output file cannot be written
async fn process_extract(
    input: &PathBuf,
    format: &str,
    output: &Option<PathBuf>,
) -> Result<()> {
    // Read input file
    let content =
        tokio::fs::read_to_string(input).await.with_context(|| {
            format!("Failed to read input file: {}", input.display())
        })?;

    // Extract frontmatter
    let (frontmatter, remaining) = extract(&content)
        .with_context(|| "Failed to extract frontmatter")?;

    // Convert to specified format
    let output_format = match format.to_lowercase().as_str() {
        "yaml" => Format::Yaml,
        "toml" => Format::Toml,
        "json" => Format::Json,
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported format: {}",
                format
            ))
        }
    };

    let formatted = to_format(&frontmatter, output_format)
        .with_context(|| "Failed to format frontmatter")?;

    // Handle output
    if let Some(output_path) = output {
        tokio::fs::write(output_path, formatted)
            .await
            .with_context(|| {
                format!(
                    "Failed to write to output file: {}",
                    output_path.display()
                )
            })?;
        println!("Frontmatter extracted to: {}", output_path.display());
    } else {
        println!("Extracted Frontmatter:\n{}", formatted);
        println!("\nRemaining Content:\n{}", remaining);
    }

    Ok(())
}

/// Process validate command
///
/// # Arguments
///
/// * `input` - Path to input file
/// * `required` - Optional comma-separated list of required fields
///
/// # Errors
///
/// Returns an error if:
/// - Input file cannot be read
/// - Frontmatter parsing fails
/// - Required fields are missing
async fn process_validate(
    input: &PathBuf,
    required: &Option<String>,
) -> Result<()> {
    // Read input file
    let content =
        tokio::fs::read_to_string(input).await.with_context(|| {
            format!("Failed to read input file: {}", input.display())
        })?;

    // Extract frontmatter
    let (frontmatter, _) = extract(&content)
        .with_context(|| "Failed to extract frontmatter")?;

    // Validate required fields
    if let Some(required_fields) = required {
        let fields: Vec<&str> = required_fields.split(',').collect();
        for field in fields {
            if !frontmatter.contains_key(field) {
                return Err(anyhow::anyhow!(
                    "Missing required field: {}",
                    field
                ));
            }
        }
    }

    println!("Validation successful!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_extract_command() -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.md");
        let output_path = dir.path().join("output.yaml");

        // Create test input file with strict YAML formatting
        let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;
        let mut file = File::create(&input_path)?;
        writeln!(file, "{}", content)?;

        // Test extract command
        process_extract(
            &input_path,
            "yaml",
            &Some(output_path.clone()),
        )
        .await?;

        // Read and log the output for debugging
        let output_content =
            tokio::fs::read_to_string(&output_path).await?;
        log::debug!("Generated YAML content:\n{}", output_content);

        // Verify output - use more flexible assertions
        assert!(
            output_content.contains("title:"),
            "title field not found in output"
        );
        assert!(
            output_content.contains("Test"),
            "Test value not found in output"
        );
        assert!(
            output_content.contains("date:"),
            "date field not found in output"
        );
        assert!(
            output_content.contains("2024-01-01"),
            "date value not found in output"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_command() -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.md");

        // Create test input file
        let content = r#"---
title: Test
date: 2024-01-01
---
Content here"#;
        let mut file = File::create(&input_path)?;
        writeln!(file, "{}", content)?;

        // Test validate command with valid fields
        process_validate(&input_path, &Some("title,date".to_string()))
            .await?;

        // Test validate command with missing field
        let result = process_validate(
            &input_path,
            &Some("title,author".to_string()),
        )
        .await;
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_cli_parsing() {
        use clap::Parser;

        // Test extract command parsing
        let args = Cli::parse_from([
            "program", "extract", "input.md", "--format", "yaml",
        ]);
        match args.command {
            Commands::Extract { input, format, .. } => {
                assert_eq!(input, PathBuf::from("input.md"));
                assert_eq!(format, "yaml");
            }
            _ => panic!("Expected Extract command"),
        }

        // Test validate command parsing
        let args = Cli::parse_from([
            "program",
            "validate",
            "input.md",
            "--required",
            "title,date",
        ]);
        match args.command {
            Commands::Validate { input, required } => {
                assert_eq!(input, PathBuf::from("input.md"));
                assert_eq!(required, Some("title,date".to_string()));
            }
            _ => panic!("Expected Validate command"),
        }
    }
}
