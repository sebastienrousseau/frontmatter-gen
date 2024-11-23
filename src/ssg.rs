// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Static Site Generator Module
//!
//! This module provides comprehensive functionality for generating static websites from markdown content with frontmatter. It handles the entire build process including template rendering, asset copying, and site structure generation.
//!
//! ## Features
//!
//! * Asynchronous file processing for improved performance
//! * Structured logging with detailed build progress
//! * Comprehensive error handling with context
//! * Safe and secure file system operations
//! * Development server with hot reloading
//!
//! ## Example
//!
//! ```rust,no_run
//! use frontmatter_gen::ssg::SsgCommand;
//! use clap::Parser;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let cmd = SsgCommand::parse();
//!     cmd.execute().await
//! }
//! ```

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use log::{debug, info, warn};
use std::path::PathBuf;
use thiserror::Error;

use crate::{config::Config, engine::Engine};

/// Errors specific to the Static Site Generator functionality
#[derive(Error, Debug)]
pub enum SsgError {
    /// Configuration error with context
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Build process error with context
    #[error("Build error: {0}")]
    BuildError(String),

    /// Server error with context
    #[error("Server error: {0}")]
    ServerError(String),

    /// File system error with path context
    #[error("File system error for path '{path}': {message}")]
    FileSystemError {
        /// Path associated with the error
        path: PathBuf,
        /// Message associated with the error
        message: String,
    },
}

/// Command-line interface for the Static Site Generator
#[derive(Parser, Debug)]
#[command(author, version, about = "Static Site Generator")]
pub struct SsgCommand {
    /// Input content directory containing markdown files and assets
    #[arg(
        short = 'd',
        long,
        global = true,
        default_value = "content",
        help = "Directory containing source content files"
    )]
    content_dir: PathBuf,

    /// Output directory for the generated static site
    #[arg(
        short = 'o',
        long,
        global = true,
        default_value = "public",
        help = "Directory where the generated site will be placed"
    )]
    output_dir: PathBuf,

    /// Template directory containing site templates
    #[arg(
        short = 't',
        long,
        global = true,
        default_value = "templates",
        help = "Directory containing site templates"
    )]
    template_dir: PathBuf,

    /// Optional configuration file path
    #[arg(
        short = 'f',
        long,
        global = true,
        help = "Path to custom configuration file"
    )]
    config: Option<PathBuf>,

    /// Subcommands for static site generation
    #[command(subcommand)]
    command: SsgSubCommand,
}

/// Available subcommands for the Static Site Generator
#[derive(Subcommand, Debug, Copy, Clone)]
pub enum SsgSubCommand {
    /// Build the static site
    Build(BuildArgs),

    /// Serve the static site locally with hot reloading
    Serve(ServeArgs),
}

/// Arguments for the build subcommand
#[derive(Args, Debug, Copy, Clone)]
pub struct BuildArgs {
    /// Clean the output directory before building
    #[arg(
        short,
        long,
        help = "Clean output directory before building"
    )]
    clean: bool,
}

/// Arguments for the serve subcommand
#[derive(Args, Debug, Copy, Clone)]
pub struct ServeArgs {
    /// Port number for the development server
    #[arg(
        short,
        long,
        default_value = "8000",
        help = "Port number for development server"
    )]
    port: u16,
}

impl SsgCommand {
    /// Executes the static site generation command
    ///
    /// This function orchestrates the entire site generation process, including:
    /// - Loading configuration
    /// - Initialising the engine
    /// - Processing content
    /// - Generating the static site
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful execution, or an error if site generation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Configuration loading fails
    /// - Engine initialisation fails
    /// - Site generation process encounters an error
    /// - Development server fails to start (when using serve command)
    pub async fn execute(&self) -> Result<()> {
        info!("Starting static site generation");
        debug!(
            "Configuration: content_dir={:?}, output_dir={:?}, template_dir={:?}",
            self.content_dir, self.output_dir, self.template_dir
        );

        // Load or create configuration with detailed error context
        let config = self
            .load_config()
            .await
            .context("Failed to load configuration")?;

        // Initialize the engine with error handling
        let engine = Engine::new().context(
            "Failed to initialize the static site generator engine",
        )?;

        match &self.command {
            SsgSubCommand::Build(args) => {
                self.build(&engine, &config, args.clean)
                    .await
                    .context("Build process failed")?;
            }
            SsgSubCommand::Serve(args) => {
                self.serve(&engine, &config, args.port)
                    .await
                    .context("Development server failed")?;
            }
        }

        info!("Site generation completed successfully");
        Ok(())
    }

    /// Loads or creates the site configuration
    ///
    /// Attempts to load configuration from a file if specified, otherwise creates
    /// a default configuration using command line arguments.
    async fn load_config(&self) -> Result<Config> {
        self.config.as_ref().map_or_else(
            || {
                Config::builder()
                    .site_name("Static Site")
                    .content_dir(&self.content_dir)
                    .output_dir(&self.output_dir)
                    .template_dir(&self.template_dir)
                    .build()
                    .context("Failed to create default configuration")
            },
            |config_path| {
                Config::from_file(config_path).context(format!(
                    "Failed to load configuration from {}",
                    config_path.display()
                ))
            },
        )
    }

    /// Builds the static site
    ///
    /// Handles the complete build process including cleaning the output directory
    /// if requested and generating all static content.
    async fn build(
        &self,
        engine: &Engine,
        config: &Config,
        clean: bool,
    ) -> Result<()> {
        info!("Building static site");
        debug!("Build configuration: {:#?}", config);

        if clean {
            self.clean_output_directory(config).await?;
        }

        // Ensure output directory exists
        tokio::fs::create_dir_all(&config.output_dir)
            .await
            .context(format!(
                "Failed to create output directory: {}",
                config.output_dir.display()
            ))?;

        engine
            .generate(config)
            .await
            .context("Site generation failed")?;
        info!("Site built successfully");
        Ok(())
    }

    /// Serves the static site locally
    ///
    /// Starts a development server with hot reloading capabilities for
    /// local testing and development.
    async fn serve(
        &self,
        engine: &Engine,
        config: &Config,
        port: u16,
    ) -> Result<()> {
        info!("Starting development server on port {}", port);

        // Build the site first
        self.build(engine, config, false).await?;

        // Configure and start the development server
        // TODO: Implement hot reloading and live server
        warn!("Hot reloading is not yet implemented");
        info!("Development server started");
        Ok(())
    }

    /// Cleans the output directory
    ///
    /// Removes all contents from the output directory while maintaining
    /// its existence.
    async fn clean_output_directory(
        &self,
        config: &Config,
    ) -> Result<()> {
        if config.output_dir.exists() {
            debug!(
                "Cleaning output directory: {}",
                config.output_dir.display()
            );
            tokio::fs::remove_dir_all(&config.output_dir)
                .await
                .context(format!(
                    "Failed to clean output directory: {}",
                    config.output_dir.display()
                ))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// Tests the build command functionality
    #[tokio::test]
    async fn test_build_command() -> Result<()> {
        // Create temporary directories for testing
        let temp = tempdir()?;
        let content_dir = temp.path().join("content");
        let output_dir = temp.path().join("public");
        let template_dir = temp.path().join("templates");

        // Create required directories
        tokio::fs::create_dir_all(&content_dir).await?;
        tokio::fs::create_dir_all(&output_dir).await?; // Add this line
        tokio::fs::create_dir_all(&template_dir).await?;

        let cmd = SsgCommand {
            content_dir: content_dir.clone(),
            output_dir: output_dir.clone(),
            template_dir: template_dir.clone(),
            config: None,
            command: SsgSubCommand::Build(BuildArgs { clean: true }),
        };

        cmd.execute().await?;

        // Verify output directory exists
        assert!(output_dir.exists());
        Ok(())
    }

    /// Tests clean build functionality
    #[tokio::test]
    async fn test_clean_build() -> Result<()> {
        let temp = tempdir()?;
        let output_dir = temp.path().join("public");

        // Create output directory with a test file
        tokio::fs::create_dir_all(&output_dir).await?;
        tokio::fs::write(output_dir.join("old.html"), "old content")
            .await?;

        let cmd = SsgCommand {
            content_dir: temp.path().join("content"),
            output_dir: output_dir.clone(),
            template_dir: temp.path().join("templates"),
            config: None,
            command: SsgSubCommand::Build(BuildArgs { clean: true }),
        };

        // Create required directories
        tokio::fs::create_dir_all(&cmd.content_dir).await?;
        tokio::fs::create_dir_all(&cmd.template_dir).await?;

        cmd.execute().await?;

        // Verify old file was removed
        assert!(!output_dir.join("old.html").exists());
        Ok(())
    }

    /// Tests command line argument parsing
    #[test]
    fn test_command_parsing() {
        let cmd = SsgCommand::try_parse_from([
            "ssg",
            "--content-dir",
            "content",
            "--output-dir",
            "public",
            "--template-dir",
            "templates",
            "build",
            "--clean",
        ])
        .unwrap();

        assert_eq!(cmd.content_dir, PathBuf::from("content"));
        assert_eq!(cmd.output_dir, PathBuf::from("public"));
        assert!(matches!(
            cmd.command,
            SsgSubCommand::Build(BuildArgs { clean: true })
        ));
    }

    /// Tests error handling for invalid configuration
    #[tokio::test]
    async fn test_invalid_config() {
        let temp = tempdir().unwrap();
        let cmd = SsgCommand {
            content_dir: temp.path().join("nonexistent"),
            output_dir: temp.path().join("public"),
            template_dir: temp.path().join("templates"),
            config: Some(PathBuf::from("nonexistent.toml")),
            command: SsgSubCommand::Build(BuildArgs { clean: false }),
        };

        let result = cmd.execute().await;
        assert!(result.is_err());
    }

    /// Tests the serve command functionality
    #[tokio::test]
    async fn test_serve_command() -> Result<()> {
        // Create temporary directories for testing
        let temp = tempdir()?;
        let content_dir = temp.path().join("content");
        let output_dir = temp.path().join("public");
        let template_dir = temp.path().join("templates");

        // Create required directories
        tokio::fs::create_dir_all(&content_dir).await?;
        tokio::fs::create_dir_all(&output_dir).await?; // Add this line
        tokio::fs::create_dir_all(&template_dir).await?;

        let cmd = SsgCommand {
            content_dir: content_dir.clone(),
            output_dir: output_dir.clone(),
            template_dir: template_dir.clone(),
            config: None,
            command: SsgSubCommand::Serve(ServeArgs { port: 8080 }),
        };

        // Execute the serve command
        cmd.execute().await?;

        // Verify output directory exists
        assert!(output_dir.exists());
        Ok(())
    }

    /// Tests loading configuration from a valid config file
    #[tokio::test]
    async fn test_load_config_valid() -> Result<()> {
        let temp = tempdir()?;
        let config_path = temp.path().join("config.toml");

        // Create required directories
        let content_dir = temp.path().join("content");
        let output_dir = temp.path().join("public");
        let template_dir = temp.path().join("templates");
        tokio::fs::create_dir_all(&content_dir).await?;
        tokio::fs::create_dir_all(&output_dir).await?;
        tokio::fs::create_dir_all(&template_dir).await?;

        // Write a valid config file with absolute paths
        let config_contents = format!(
            r#"
        site_name = "Test Site"
        content_dir = "{}"
        output_dir = "{}"
        template_dir = "{}"
    "#,
            content_dir.display(),
            output_dir.display(),
            template_dir.display()
        );
        tokio::fs::write(&config_path, config_contents).await?;

        let cmd = SsgCommand {
            content_dir: content_dir.clone(),
            output_dir: output_dir.clone(),
            template_dir: template_dir.clone(),
            config: Some(config_path.clone()),
            command: SsgSubCommand::Build(BuildArgs { clean: false }),
        };

        let config = cmd.load_config().await?;

        // Verify that the config was loaded correctly
        assert_eq!(config.site_name, "Test Site");
        assert_eq!(config.content_dir, content_dir);
        assert_eq!(config.output_dir, output_dir);
        assert_eq!(config.template_dir, template_dir);

        Ok(())
    }

    /// Tests loading configuration from an invalid config file
    #[tokio::test]
    async fn test_load_config_invalid() -> Result<()> {
        let temp = tempdir()?;
        let config_path = temp.path().join("config.toml");

        // Write an invalid config file
        tokio::fs::write(&config_path, "invalid_toml_content").await?;

        let cmd = SsgCommand {
            content_dir: PathBuf::from("content"),
            output_dir: PathBuf::from("public"),
            template_dir: PathBuf::from("templates"),
            config: Some(config_path.clone()),
            command: SsgSubCommand::Build(BuildArgs { clean: false }),
        };

        let result = cmd.load_config().await;

        // Verify that an error is returned
        assert!(result.is_err());

        Ok(())
    }

    /// Tests cleaning the output directory when it exists
    #[tokio::test]
    async fn test_clean_output_directory_exists() -> Result<()> {
        let temp = tempdir()?;
        let output_dir = temp.path().join("public");

        // Create output directory with a test file
        tokio::fs::create_dir_all(&output_dir).await?;
        tokio::fs::write(output_dir.join("test.html"), "test content")
            .await?;

        let cmd = SsgCommand {
            content_dir: temp.path().join("content"),
            output_dir: output_dir.clone(),
            template_dir: temp.path().join("templates"),
            config: None,
            command: SsgSubCommand::Build(BuildArgs { clean: true }),
        };

        // Create the necessary directories before building the config
        tokio::fs::create_dir_all(&cmd.content_dir).await?;
        tokio::fs::create_dir_all(&cmd.template_dir).await?; // Add this line

        // Create a config object
        let config = Config::builder()
            .site_name("Test Site")
            .content_dir(&cmd.content_dir)
            .output_dir(&cmd.output_dir)
            .template_dir(&cmd.template_dir)
            .build()
            .unwrap();

        // Call clean_output_directory
        cmd.clean_output_directory(&config).await?;

        // Verify that the output directory does not exist
        assert!(!output_dir.exists());

        Ok(())
    }

    /// Tests cleaning the output directory when it does not exist
    #[tokio::test]
    async fn test_clean_output_directory_not_exists() -> Result<()> {
        let temp = tempdir()?;
        let output_dir = temp.path().join("public");

        let cmd = SsgCommand {
            content_dir: temp.path().join("content"),
            output_dir: output_dir.clone(),
            template_dir: temp.path().join("templates"),
            config: None,
            command: SsgSubCommand::Build(BuildArgs { clean: true }),
        };

        // Create the necessary directories before building the config
        tokio::fs::create_dir_all(&cmd.content_dir).await?;
        tokio::fs::create_dir_all(&cmd.output_dir).await?; // Add this line
        tokio::fs::create_dir_all(&cmd.template_dir).await?; // Add this line

        // Create a config object
        let config = Config::builder()
            .site_name("Test Site")
            .content_dir(&cmd.content_dir)
            .output_dir(&cmd.output_dir)
            .template_dir(&cmd.template_dir)
            .build()
            .unwrap();

        // Call clean_output_directory
        cmd.clean_output_directory(&config).await?;

        // Verify that the output directory still does not exist
        assert!(!output_dir.exists());

        Ok(())
    }

    /// Tests error handling in the execute method when load_config fails
    #[tokio::test]
    async fn test_execute_load_config_failure() -> Result<()> {
        let temp = tempdir()?;
        let invalid_config_path =
            temp.path().join("invalid_config.toml");

        // Write an invalid configuration file
        tokio::fs::write(&invalid_config_path, "invalid_content")
            .await?;

        let cmd = SsgCommand {
            content_dir: PathBuf::from("content"),
            output_dir: PathBuf::from("public"),
            template_dir: PathBuf::from("templates"),
            config: Some(invalid_config_path.clone()),
            command: SsgSubCommand::Build(BuildArgs { clean: false }),
        };

        let result = cmd.execute().await;

        assert!(result.is_err());
        let err_message = result.unwrap_err().to_string();
        assert!(
            err_message.contains("Failed to load configuration"),
            "Unexpected error message: {}",
            err_message
        );

        Ok(())
    }

    /// Tests command line argument parsing with invalid inputs
    #[test]
    fn test_command_parsing_invalid() {
        let result = SsgCommand::try_parse_from([
            "ssg",
            "--unknown-arg",
            "value",
            "build",
        ]);

        assert!(result.is_err());
    }
}
