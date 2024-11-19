// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Static Site Generator Module
//!
//! This module provides functionality for generating static websites from markdown content
//! with frontmatter. It handles the entire build process including template rendering,
//! asset copying, and site structure generation.

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use log::{debug, info};
use std::path::PathBuf;

use crate::{config::Config, engine::Engine};

/// Command-line interface for the Static Site Generator
#[derive(Parser, Debug)]
#[command(author, version, about = "Static Site Generator")]
pub struct SsgCommand {
    /// Input content directory
    #[arg(short = 'd', long, global = true, default_value = "content")]
    content_dir: PathBuf,

    /// Output directory for generated site
    #[arg(short = 'o', long, global = true, default_value = "public")]
    output_dir: PathBuf,

    /// Template directory
    #[arg(
        short = 't',
        long,
        global = true,
        default_value = "templates"
    )]
    template_dir: PathBuf,

    /// Optional configuration file
    #[arg(short = 'f', long, global = true)]
    config: Option<PathBuf>,

    /// Subcommands for static site generation
    #[command(subcommand)]
    command: SsgSubCommand,
}

/// Subcommands for the Static Site Generator
#[derive(Subcommand, Debug, Copy, Clone)]
pub enum SsgSubCommand {
    /// Build the static site
    Build(BuildArgs),

    /// Serve the static site locally
    Serve(ServeArgs),
}

/// Arguments for the `build` subcommand
#[derive(Args, Debug, Copy, Clone)]
pub struct BuildArgs {
    /// Clean the output directory before building
    #[arg(short, long)]
    clean: bool,
}

/// Arguments for the `serve` subcommand
#[derive(Args, Debug, Copy, Clone)]
pub struct ServeArgs {
    /// Port number for the development server
    #[arg(short, long, default_value = "8000")]
    port: u16,
}

impl SsgCommand {
    /// Executes the static site generation command
    ///
    /// # Returns
    /// Returns `Ok(())` on successful execution, or an error if site generation fails.
    pub async fn execute(&self) -> Result<()> {
        info!("Starting static site generation");
        debug!("Global configuration: content_dir={:?}, output_dir={:?}, template_dir={:?}",
            self.content_dir, self.output_dir, self.template_dir);

        // Load or create configuration
        let config = if let Some(config_path) = &self.config {
            Config::from_file(config_path)?
        } else {
            Config::builder()
                .site_name("Static Site")
                .content_dir(&self.content_dir)
                .output_dir(&self.output_dir)
                .template_dir(&self.template_dir)
                .build()?
        };

        // Initialize the engine
        let engine = Engine::new()?;

        match &self.command {
            SsgSubCommand::Build(args) => {
                self.build(&engine, &config, args.clean).await
            }
            SsgSubCommand::Serve(args) => {
                self.serve(&engine, &config, args.port).await
            }
        }
    }

    /// Build the static site
    async fn build(
        &self,
        engine: &Engine,
        config: &Config,
        clean: bool,
    ) -> Result<()> {
        info!("Building static site");
        debug!("Configuration: {:#?}", config);

        if clean {
            debug!("Cleaning output directory");
            if config.output_dir.exists() {
                std::fs::remove_dir_all(&config.output_dir)
                    .context("Failed to clean output directory")?;
            }
        }

        engine.generate(config).await?;
        info!("Site built successfully");
        Ok(())
    }

    /// Serve the static site locally
    async fn serve(
        &self,
        engine: &Engine,
        config: &Config,
        port: u16,
    ) -> Result<()> {
        info!("Starting development server on port {}", port);

        // Build the site first
        self.build(engine, config, false).await?;

        // Placeholder for development server logic
        info!("Development server started");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_build_command() -> Result<()> {
        let temp = tempdir()?;
        let content_dir = temp.path().join("content");
        let output_dir = temp.path().join("public");
        let template_dir = temp.path().join("templates");

        // Ensure the output directory exists
        std::fs::create_dir_all(&output_dir)?;

        // Ensure the content and template directories exist
        std::fs::create_dir_all(&content_dir)?;
        std::fs::create_dir_all(&template_dir)?;

        let cmd = SsgCommand {
            content_dir,
            output_dir: output_dir.clone(),
            template_dir,
            config: None,
            command: SsgSubCommand::Build(BuildArgs { clean: true }),
        };

        cmd.execute().await?;

        // Verify that the output directory exists after the command execution
        assert!(output_dir.exists());

        Ok(())
    }

    #[tokio::test]
    async fn test_clean_build() -> Result<()> {
        let temp = tempdir()?;
        let output_dir = temp.path().join("public");

        std::fs::create_dir_all(&output_dir)?;
        std::fs::write(output_dir.join("old.html"), "old content")?;

        let cmd = SsgCommand {
            content_dir: temp.path().join("content"),
            output_dir: output_dir.clone(),
            template_dir: temp.path().join("templates"),
            config: None,
            command: SsgSubCommand::Build(BuildArgs { clean: true }),
        };

        std::fs::create_dir_all(&cmd.content_dir)?;
        std::fs::create_dir_all(&cmd.template_dir)?;

        cmd.execute().await?;
        assert!(!output_dir.join("old.html").exists());
        Ok(())
    }

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
}
