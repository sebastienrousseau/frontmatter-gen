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
        log::info!(
            "Frontmatter extracted to `{}`",
            output_path.display()
        );
    } else {
        log::info!(
            "Extracted Frontmatter as {}\n\n{}\n\n",
            output_format,
            formatted
        );
        log::info!("Remaining Markdown Content\n\n{}\n\n", remaining);
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

    // Tests for process_extract function
    mod extract_tests {
        use super::*;

        #[tokio::test]
        async fn test_extract_command_default_format() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path().join("output.yaml");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Test extract command without specifying format (should default to "yaml")
            let args = vec![
                "program",
                "extract",
                input_path.to_str().unwrap(),
                "--output",
                output_path.to_str().unwrap(),
            ];
            let cli = Cli::parse_from(args);
            let result = cli.process().await;
            assert!(result.is_ok());

            // Verify output file was created
            let output_content =
                tokio::fs::read_to_string(&output_path).await?;
            assert!(output_content.contains("title:"));
            assert!(output_content.contains("Test"));

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_uppercase_format() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path().join("output.yaml");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Test extract command with uppercase format
            let result = process_extract(
                &input_path,
                "YAML",
                &Some(output_path.clone()),
            )
            .await;
            assert!(result.is_ok());

            // Verify output file was created
            let output_content =
                tokio::fs::read_to_string(&output_path).await?;
            assert!(output_content.contains("title:"));
            assert!(output_content.contains("Test"));

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_invalid_format() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Test extract command with an invalid format to ensure it returns an error
            let result =
                process_extract(&input_path, "invalid_format", &None)
                    .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e.to_string().contains("Unsupported format"));
            }

            Ok(())
        }

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
        async fn test_extract_command_invalid_input_file() -> Result<()>
        {
            let input_path = PathBuf::from("nonexistent.md");
            let output_path = None;
            let result =
                process_extract(&input_path, "yaml", &output_path)
                    .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to read input file"));
            }
            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_unsupported_format() -> Result<()>
        {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file
            let content = r"---
title: Test
date: 2024-01-01
---
Content here";
            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result =
                process_extract(&input_path, "xml", &None).await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e.to_string().contains("Unsupported format"));
            }
            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_no_frontmatter() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file without frontmatter
            let content = "Content here without frontmatter";
            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result =
                process_extract(&input_path, "yaml", &None).await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to extract frontmatter"));
            }
            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_invalid_frontmatter() -> Result<()>
        {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file with invalid frontmatter
            let content = r#"---
title: "Test
date: 2024-01-01
---
Content here"#; // Note the missing closing quote for title

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result =
                process_extract(&input_path, "yaml", &None).await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to extract frontmatter"));
            }
            Ok(())
        }

        #[cfg(unix)]
        #[tokio::test]
        async fn test_extract_command_output_write_error() -> Result<()>
        {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_dir = dir.path().join("readonly_dir");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;
            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Create a read-only directory
            tokio::fs::create_dir(&output_dir).await?;
            let mut perms =
                tokio::fs::metadata(&output_dir).await?.permissions();
            perms.set_readonly(true);
            tokio::fs::set_permissions(&output_dir, perms).await?;

            let output_path = output_dir.join("output.yaml");

            // Attempt to write to the read-only directory
            let result = process_extract(
                &input_path,
                "yaml",
                &Some(output_path.clone()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to write to output file"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_toml_format() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path().join("output.toml");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result = process_extract(
                &input_path,
                "toml",
                &Some(output_path.clone()),
            )
            .await;
            assert!(result.is_ok());

            // Read and log the output for debugging
            let output_content =
                tokio::fs::read_to_string(&output_path).await?;
            log::debug!("Generated TOML content:\n{}", output_content);

            // Verify output
            assert!(output_content.contains("title = "));
            assert!(output_content.contains("Test"));
            assert!(output_content.contains("date = "));
            assert!(output_content.contains("2024-01-01"));

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_json_format() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path().join("output.json");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result = process_extract(
                &input_path,
                "json",
                &Some(output_path.clone()),
            )
            .await;
            assert!(result.is_ok());

            // Read and log the output for debugging
            let output_content =
                tokio::fs::read_to_string(&output_path).await?;
            log::debug!("Generated JSON content:\n{}", output_content);

            // Verify output
            assert!(output_content.contains("\"title\":"));
            assert!(output_content.contains("Test"));
            assert!(output_content.contains("\"date\":"));
            assert!(output_content.contains("2024-01-01"));

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_no_output_file() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result =
                process_extract(&input_path, "yaml", &None).await;
            assert!(result.is_ok());

            // Since output is to stdout, we can't easily capture it here
            // We can assume that if no error occurred, the function worked as expected

            Ok(())
        }
    }

    // Tests for process_validate function
    mod validate_tests {
        use super::*;

        #[tokio::test]
        async fn test_validate_command_required_fields_whitespace_only(
        ) -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Test validate command with required fields containing only whitespace
            let result =
                process_validate(&input_path, &Some("   ".to_string()))
                    .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Missing required field:    "));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_cli_process_with_invalid_subcommand() {
            let result =
                Cli::try_parse_from(["program", "invalid_command"]);
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_validate_command_missing_required_field(
        ) -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file without the 'author' field
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // 'author' field is required but missing
            let result = process_validate(
                &input_path,
                &Some("author".to_string()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Missing required field: author"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_output_is_directory() -> Result<()>
        {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path(); // Use the directory path instead of a file

            // Create test input file with valid frontmatter
            let content = r#"---
title: "Test"
date: "2024-01-01"
---
Content here"#;

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result = process_extract(
                &input_path,
                "yaml",
                &Some(output_path.to_path_buf()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to write to output file"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_extract_command_with_empty_frontmatter(
        ) -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path().join("output.yaml");

            // Create test input file with empty frontmatter
            let content = r"---
---
Content here";

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result = process_extract(
                &input_path,
                "yaml",
                &Some(output_path.clone()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to extract frontmatter"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_validate_command() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file
            let content = r"---
title: Test
date: 2024-01-01
---
Content here";
            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Test validate command with valid fields
            process_validate(
                &input_path,
                &Some("title,date".to_string()),
            )
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

        #[tokio::test]
        async fn test_validate_command_invalid_input_file() -> Result<()>
        {
            let input_path = PathBuf::from("nonexistent.md");

            let result = process_validate(
                &input_path,
                &Some("title".to_string()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to read input file"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_validate_command_no_frontmatter() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file without frontmatter
            let content = "Content here without frontmatter";
            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result = process_validate(
                &input_path,
                &Some("title".to_string()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to extract frontmatter"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_validate_command_invalid_frontmatter(
        ) -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file with invalid frontmatter
            let content = r"---
title: 'Test
date: 2024-01-01
---
Content here";

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let result = process_validate(
                &input_path,
                &Some("title".to_string()),
            )
            .await;
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(e
                    .to_string()
                    .contains("Failed to extract frontmatter"));
            }

            Ok(())
        }

        #[tokio::test]
        async fn test_validate_command_no_required_fields() -> Result<()>
        {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file with valid frontmatter
            let content = r"---
title: Test
date: 2024-01-01
---
Content here";

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            // Test validate command with no required fields
            let result = process_validate(&input_path, &None).await;
            assert!(result.is_ok());

            Ok(())
        }
    }

    // Tests for CLI parsing
    mod cli_parsing_tests {
        use super::*;
        use clap::Parser;

        #[test]
        fn test_cli_parsing_extract_default_format() {
            // Test extract command parsing without format argument
            let args =
                Cli::parse_from(["program", "extract", "input.md"]);
            match args.command {
                Commands::Extract { input, format, .. } => {
                    assert_eq!(input, PathBuf::from("input.md"));
                    assert_eq!(format, "yaml"); // Default value
                }
                _ => panic!("Expected Extract command"),
            }
        }

        #[test]
        fn test_cli_parsing_invalid_command() {
            // Test parsing an invalid command
            let result =
                Cli::try_parse_from(["program", "invalid", "input.md"]);
            assert!(result.is_err());
        }

        #[test]
        fn test_cli_parsing() {
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
                    assert_eq!(
                        required,
                        Some("title,date".to_string())
                    );
                }
                _ => panic!("Expected Validate command"),
            }
        }
    }

    // Tests for CLI process function
    mod cli_process_tests {
        use super::*;

        #[tokio::test]
        async fn test_cli_process_extract() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");
            let output_path = dir.path().join("output.yaml");

            // Create test input file with valid frontmatter
            let content = r"---
title: Test
date: 2024-01-01
---
Content here";

            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let cli = Cli {
                command: Commands::Extract {
                    input: input_path.clone(),
                    format: "yaml".to_string(),
                    output: Some(output_path.clone()),
                },
            };

            let result = cli.process().await;
            assert!(result.is_ok());

            // Verify output file was created
            let output_content =
                tokio::fs::read_to_string(&output_path).await?;
            assert!(output_content.contains("title:"));
            assert!(output_content.contains("Test"));

            Ok(())
        }

        #[tokio::test]
        async fn test_cli_process_validate() -> Result<()> {
            let dir = tempdir()?;
            let input_path = dir.path().join("test.md");

            // Create test input file
            let content = r"---
title: Test
date: 2024-01-01
---
Content here";
            let mut file = File::create(&input_path)?;
            writeln!(file, "{}", content)?;

            let cli = Cli {
                command: Commands::Validate {
                    input: input_path.clone(),
                    required: Some("title,date".to_string()),
                },
            };

            let result = cli.process().await;
            assert!(result.is_ok());

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_extract_command_empty_format() -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.md");

        // Create test input file with valid frontmatter
        let content = r"---
title: 'Test'
---
Content here";

        let mut file = File::create(&input_path)?;
        writeln!(file, "{}", content)?;

        // Test extract command with an empty format string
        let result = process_extract(&input_path, "", &None).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Unsupported format"));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_command_required_fields_with_whitespace(
    ) -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.md");

        // Create test input file with valid frontmatter
        let content = r"---
title: 'Test'
date: '2024-01-01'
---
Content here";

        let mut file = File::create(&input_path)?;
        writeln!(file, "{}", content)?;

        // Required fields with leading/trailing whitespace
        let result = process_validate(
            &input_path,
            &Some(" title , date ".to_string()),
        )
        .await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e
                .to_string()
                .contains("Missing required field:  title "));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_command_duplicate_required_fields(
    ) -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.md");

        // Create test input file with valid frontmatter
        let content = r"---
title: 'Test'
date: '2024-01-01'
---
Content here";

        let mut file = File::create(&input_path)?;
        writeln!(file, "{}", content)?;

        // Required fields with duplicates
        let result = process_validate(
            &input_path,
            &Some("title,date,title".to_string()),
        )
        .await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_extract_command_with_complex_data() -> Result<()> {
        let dir = tempdir()?;
        let input_path = dir.path().join("test.md");
        let output_path = dir.path().join("output.json");

        // Create test input file with complex data types
        let content = r"---
title: 'Test'
tags:
  - rust
  - cli
nested:
  level1:
    level2: 'deep value'
---
Content here";

        let mut file = File::create(&input_path)?;
        writeln!(file, "{}", content)?;

        // Test extract command with JSON format
        let result = process_extract(
            &input_path,
            "json",
            &Some(output_path.clone()),
        )
        .await;
        assert!(result.is_ok());

        // Read and verify the output
        let output_content =
            tokio::fs::read_to_string(&output_path).await?;
        let json_output: serde_json::Value =
            serde_json::from_str(&output_content)?;
        assert_eq!(json_output["title"], "Test");
        assert_eq!(json_output["tags"][0], "rust");
        assert_eq!(
            json_output["nested"]["level1"]["level2"],
            "deep value"
        );

        Ok(())
    }
}
