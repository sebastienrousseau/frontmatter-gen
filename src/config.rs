// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Configuration Module
//!
//! This module provides a robust and type-safe configuration system for the Static Site Generator.
//! It handles validation, serialization, and secure management of all configuration settings.
//!
//! ## Features
//!
//! - Type-safe configuration management
//! - Comprehensive validation of all settings
//! - Secure path handling and normalization
//! - Flexible Builder pattern for configuration creation
//! - Serialization support via serde
//! - Default values for optional settings
//!
//! ## Examples
//!
//! ```rust
//! use frontmatter_gen::config::Config;
//!
//! # fn main() -> anyhow::Result<()> {
//! let config = Config::builder()
//!     .site_name("My Blog")
//!     .site_title("My Awesome Blog")
//!     .content_dir("content")
//!     .build()?;
//!
//! assert_eq!(config.site_name(), "My Blog");
//! # Ok(())
//! # }
//! ```

use std::fmt;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use crate::utils::fs::validate_path_safety;

/// Errors specific to configuration operations
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid site name provided
    #[error("Invalid site name: {0}")]
    InvalidSiteName(String),

    /// Invalid directory path with detailed context
    #[error("Invalid directory path '{path}': {details}")]
    InvalidPath {
        /// The path that was invalid
        path: String,
        /// Details about why the path was invalid
        details: String,
    },

    /// Invalid URL format
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    /// Invalid language code
    #[error("Invalid language code '{0}': must be in format 'xx-XX'")]
    InvalidLanguage(String),

    /// Configuration file error
    #[error("Configuration file error: {0}")]
    FileError(#[from] std::io::Error),

    /// TOML parsing error
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Server configuration error
    #[error("Server configuration error: {0}")]
    ServerError(String),
}

/// Core configuration structure for the Static Site Generator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Unique identifier for this configuration
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,

    /// Name of the site (required)
    pub site_name: String,

    /// Site title used in metadata
    #[serde(default = "default_site_title")]
    pub site_title: String,

    /// Site description used in metadata
    #[serde(default = "default_site_description")]
    pub site_description: String,

    /// Primary language code (format: xx-XX)
    #[serde(default = "default_language")]
    pub language: String,

    /// Base URL for the site
    #[serde(default = "default_base_url")]
    pub base_url: String,

    /// Directory containing content files
    #[serde(default = "default_content_dir")]
    pub content_dir: PathBuf,

    /// Directory for generated output
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    /// Directory containing templates
    #[serde(default = "default_template_dir")]
    pub template_dir: PathBuf,

    /// Optional directory for development server
    #[serde(default)]
    pub serve_dir: Option<PathBuf>,

    /// Whether the development server is enabled
    #[serde(default)]
    pub server_enabled: bool,

    /// Port for development server
    #[serde(default = "default_port")]
    pub server_port: u16,
}

// Default value functions for serde
fn default_site_title() -> String {
    "My Shokunin Site".to_string()
}

fn default_site_description() -> String {
    "A site built with Shokunin".to_string()
}

fn default_language() -> String {
    "en-GB".to_string()
}

fn default_base_url() -> String {
    "http://localhost:8000".to_string()
}

fn default_content_dir() -> PathBuf {
    PathBuf::from("content")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("public")
}

fn default_template_dir() -> PathBuf {
    PathBuf::from("templates")
}

fn default_port() -> u16 {
    8000
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Site: {} ({})\nContent: {}\nOutput: {}\nTemplates: {}",
            self.site_name,
            self.site_title,
            self.content_dir.display(),
            self.output_dir.display(),
            self.template_dir.display()
        )
    }
}

impl Config {
    /// Creates a new ConfigBuilder instance for fluent configuration creation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use frontmatter_gen::config::Config;
    ///
    /// let config = Config::builder()
    ///     .site_name("My Site")
    ///     .content_dir("content")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Loads configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Returns
    ///
    /// Returns a Result containing the loaded Config or an error
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - File cannot be read
    /// - TOML parsing fails
    /// - Configuration validation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use frontmatter_gen::config::Config;
    /// use std::path::Path;
    ///
    /// let config = Config::from_file(Path::new("config.toml")).unwrap();
    /// ```
    pub fn from_file(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| {
                format!(
                    "Failed to read config file: {}",
                    path.display()
                )
            })?;

        let mut config: Config = toml::from_str(&content)
            .context("Failed to parse TOML configuration")?;

        // Ensure we have a unique ID
        config.id = Uuid::new_v4();

        // Validate the loaded configuration
        config.validate()?;

        Ok(config)
    }

    /// Validates the configuration settings
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if validation passes, or an error if validation fails
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - Required fields are empty
    /// - Paths are invalid or unsafe
    /// - URLs are malformed
    /// - Language code format is invalid
    pub fn validate(&self) -> Result<()> {
        // Validate site name
        if self.site_name.trim().is_empty() {
            return Err(ConfigError::InvalidSiteName(
                "Site name cannot be empty".to_string(),
            )
            .into());
        }

        // Validate paths with consistent error handling
        self.validate_path(&self.content_dir, "content_dir")?;
        self.validate_path(&self.output_dir, "output_dir")?;
        self.validate_path(&self.template_dir, "template_dir")?;

        // Validate serve_dir if present
        if let Some(serve_dir) = &self.serve_dir {
            self.validate_path(serve_dir, "serve_dir")?;
        }

        // Validate base URL
        Url::parse(&self.base_url).map_err(|_| {
            ConfigError::InvalidUrl(self.base_url.clone())
        })?;

        // Validate language code format (xx-XX)
        if !self.is_valid_language_code(&self.language) {
            return Err(ConfigError::InvalidLanguage(
                self.language.clone(),
            )
            .into());
        }

        // Validate server port if enabled
        if self.server_enabled && !self.is_valid_port(self.server_port)
        {
            return Err(ConfigError::ServerError(format!(
                "Invalid port number: {}",
                self.server_port
            ))
            .into());
        }

        Ok(())
    }

    /// Validates a path for safety and accessibility
    fn validate_path(&self, path: &Path, name: &str) -> Result<()> {
        validate_path_safety(path).with_context(|| {
            format!("Invalid {} path: {}", name, path.display())
        })
    }

    /// Checks if a language code is valid (format: xx-XX)
    fn is_valid_language_code(&self, code: &str) -> bool {
        let parts: Vec<&str> = code.split('-').collect();
        if parts.len() != 2 {
            return false;
        }

        let (lang, region) = (parts[0], parts[1]);
        lang.len() == 2
            && region.len() == 2
            && lang.chars().all(|c| c.is_ascii_lowercase())
            && region.chars().all(|c| c.is_ascii_uppercase())
    }

    /// Checks if a port number is valid
    fn is_valid_port(&self, port: u16) -> bool {
        port >= 1024
    }

    /// Gets the unique identifier for this configuration
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Gets the site name
    pub fn site_name(&self) -> &str {
        &self.site_name
    }

    /// Gets whether the development server is enabled
    pub fn server_enabled(&self) -> bool {
        self.server_enabled
    }

    /// Gets the server port if the server is enabled
    pub fn server_port(&self) -> Option<u16> {
        if self.server_enabled {
            Some(self.server_port)
        } else {
            None
        }
    }
}

/// Builder for creating Config instances
#[derive(Default)]
pub struct ConfigBuilder {
    site_name: Option<String>,
    site_title: Option<String>,
    site_description: Option<String>,
    language: Option<String>,
    base_url: Option<String>,
    content_dir: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    template_dir: Option<PathBuf>,
    serve_dir: Option<PathBuf>,
    server_enabled: bool,
    server_port: Option<u16>,
}

impl ConfigBuilder {
    /// Sets the site name
    pub fn site_name<S: Into<String>>(mut self, name: S) -> Self {
        self.site_name = Some(name.into());
        self
    }

    /// Sets the site title
    pub fn site_title<S: Into<String>>(mut self, title: S) -> Self {
        self.site_title = Some(title.into());
        self
    }

    /// Sets the site description
    pub fn site_description<S: Into<String>>(
        mut self,
        desc: S,
    ) -> Self {
        self.site_description = Some(desc.into());
        self
    }

    /// Sets the language code
    pub fn language<S: Into<String>>(mut self, lang: S) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Sets the base URL
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the content directory
    pub fn content_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.content_dir = Some(path.into());
        self
    }

    /// Sets the output directory
    pub fn output_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.output_dir = Some(path.into());
        self
    }

    /// Sets the template directory
    pub fn template_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.template_dir = Some(path.into());
        self
    }

    /// Sets the serve directory
    pub fn serve_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.serve_dir = Some(path.into());
        self
    }

    /// Enables or disables the development server
    pub fn server_enabled(mut self, enabled: bool) -> Self {
        self.server_enabled = enabled;
        self
    }

    /// Sets the server port
    pub fn server_port(mut self, port: u16) -> Self {
        self.server_port = Some(port);
        self
    }

    /// Builds the Config instance
    ///
    /// # Returns
    ///
    /// Returns a Result containing the built Config or an error
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - Required fields are missing
    /// - Validation fails
    pub fn build(self) -> Result<Config> {
        let config = Config {
            id: Uuid::new_v4(),
            site_name: self.site_name.unwrap_or_default(),
            site_title: self
                .site_title
                .unwrap_or_else(default_site_title),
            site_description: self
                .site_description
                .unwrap_or_else(default_site_description),
            language: self.language.unwrap_or_else(default_language),
            base_url: self.base_url.unwrap_or_else(default_base_url),
            content_dir: self
                .content_dir
                .unwrap_or_else(default_content_dir),
            output_dir: self
                .output_dir
                .unwrap_or_else(default_output_dir),
            template_dir: self
                .template_dir
                .unwrap_or_else(default_template_dir),
            serve_dir: self.serve_dir,
            server_enabled: self.server_enabled,
            server_port: self.server_port.unwrap_or_else(default_port),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_builder_basic() -> Result<()> {
        let config = Config::builder()
            .site_name("Test Site")
            .site_title("Test Title")
            .build()?;

        assert_eq!(config.site_name(), "Test Site");
        assert_eq!(config.site_title, "Test Title");
        Ok(())
    }

    #[test]
    fn test_invalid_language_code() {
        // Test invalid language code formats
        let invalid_codes = vec![
            "en",     // Too short
            "eng-US", // First part too long
            "en-USA", // Second part too long
            "EN-US",  // First part uppercase
            "en-us",  // Second part lowercase
            "en_US",  // Wrong separator
        ];

        for code in invalid_codes {
            let result = Config::builder()
                .site_name("Test Site")
                .language(code)
                .build();
            assert!(
                result.is_err(),
                "Language code '{}' should be invalid",
                code
            );
        }
    }

    #[test]
    fn test_valid_language_code() {
        // Test valid language codes
        let valid_codes = vec!["en-US", "fr-FR", "de-DE", "ja-JP"];

        for code in valid_codes {
            let result = Config::builder()
                .site_name("Test Site")
                .language(code)
                .build();
            assert!(
                result.is_ok(),
                "Language code '{}' should be valid",
                code
            );
        }
    }

    #[test]
    fn test_valid_urls() -> Result<()> {
        // Test valid URLs
        let valid_urls = vec![
            "http://localhost",
            "https://example.com",
            "http://localhost:8080",
            "https://sub.domain.com/path",
        ];

        for url in valid_urls {
            let config = Config::builder()
                .site_name("Test Site")
                .base_url(url)
                .build()?;
            assert_eq!(config.base_url, url);
        }
        Ok(())
    }

    #[test]
    fn test_server_port_validation() {
        // Test invalid ports (only those below 1024, as those are the restricted ones)
        let invalid_ports = vec![0, 22, 80, 443, 1023];

        for port in invalid_ports {
            let result = Config::builder()
                .site_name("Test Site")
                .server_enabled(true)
                .server_port(port)
                .build();
            assert!(result.is_err(), "Port {} should be invalid", port);
        }

        // Test valid ports
        let valid_ports = vec![1024, 3000, 8080, 8000, 65535];

        for port in valid_ports {
            let result = Config::builder()
                .site_name("Test Site")
                .server_enabled(true)
                .server_port(port)
                .build();
            assert!(result.is_ok(), "Port {} should be valid", port);
        }
    }

    #[test]
    fn test_path_validation() {
        // Test invalid paths
        let invalid_paths = vec![
            "../../outside",
            "/absolute/path",
            "path\\with\\backslashes",
            "path\0with\0nulls",
        ];

        for path in invalid_paths {
            let result = Config::builder()
                .site_name("Test Site")
                .content_dir(path)
                .build();
            assert!(
                result.is_err(),
                "Path '{}' should be invalid",
                path
            );
        }
    }

    #[test]
    fn test_config_serialization() -> Result<()> {
        let config = Config::builder()
            .site_name("Test Site")
            .site_title("Test Title")
            .content_dir("content")
            .build()?;

        // Test TOML serialization
        let toml_str = toml::to_string(&config)?;
        let deserialized: Config = toml::from_str(&toml_str)?;
        assert_eq!(config.site_name, deserialized.site_name);
        assert_eq!(config.site_title, deserialized.site_title);

        Ok(())
    }

    #[test]
    fn test_config_display() -> Result<()> {
        let config = Config::builder()
            .site_name("Test Site")
            .site_title("Test Title")
            .build()?;

        let display = format!("{}", config);
        assert!(display.contains("Test Site"));
        assert!(display.contains("Test Title"));
        Ok(())
    }

    #[test]
    fn test_config_clone() -> Result<()> {
        let config = Config::builder()
            .site_name("Test Site")
            .site_title("Test Title")
            .build()?;

        let cloned = config.clone();
        assert_eq!(config.site_name, cloned.site_name);
        assert_eq!(config.site_title, cloned.site_title);
        assert_eq!(config.id(), cloned.id());
        Ok(())
    }

    #[test]
    fn test_from_file() -> Result<()> {
        let dir = tempdir()?;
        let config_path = dir.path().join("config.toml");

        let config_content = r#"
            site_name = "Test Site"
            site_title = "Test Title"
            language = "en-US"
            base_url = "http://localhost:8000"
        "#;

        std::fs::write(&config_path, config_content)?;

        let config = Config::from_file(&config_path)?;
        assert_eq!(config.site_name, "Test Site");
        assert_eq!(config.site_title, "Test Title");
        assert_eq!(config.language, "en-US");

        Ok(())
    }

    #[test]
    fn test_default_values() -> Result<()> {
        let config =
            Config::builder().site_name("Test Site").build()?;

        assert_eq!(config.site_title, default_site_title());
        assert_eq!(config.site_description, default_site_description());
        assert_eq!(config.language, default_language());
        assert_eq!(config.base_url, default_base_url());
        assert_eq!(config.content_dir, default_content_dir());
        assert_eq!(config.output_dir, default_output_dir());
        assert_eq!(config.template_dir, default_template_dir());
        assert_eq!(config.server_port, default_port());
        assert!(!config.server_enabled);
        assert!(config.serve_dir.is_none());

        Ok(())
    }

    #[test]
    fn test_server_configuration() -> Result<()> {
        let config = Config::builder()
            .site_name("Test Site")
            .server_enabled(true)
            .server_port(9000)
            .build()?;

        assert!(config.server_enabled());
        assert_eq!(config.server_port(), Some(9000));

        // Test disabled server
        let config = Config::builder()
            .site_name("Test Site")
            .server_enabled(false)
            .server_port(9000)
            .build()?;

        assert!(!config.server_enabled());
        assert_eq!(config.server_port(), None);

        Ok(())
    }
}
