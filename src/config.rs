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

    /// Tests for default value functions
    mod default_values_tests {
        use super::*;

        #[test]
        fn test_default_site_title() {
            assert_eq!(default_site_title(), "My Shokunin Site");
        }

        #[test]
        fn test_default_site_description() {
            assert_eq!(
                default_site_description(),
                "A site built with Shokunin"
            );
        }

        #[test]
        fn test_default_language() {
            assert_eq!(default_language(), "en-GB");
        }

        #[test]
        fn test_default_base_url() {
            assert_eq!(default_base_url(), "http://localhost:8000");
        }

        #[test]
        fn test_default_content_dir() {
            assert_eq!(default_content_dir(), PathBuf::from("content"));
        }

        #[test]
        fn test_default_output_dir() {
            assert_eq!(default_output_dir(), PathBuf::from("public"));
        }

        #[test]
        fn test_default_template_dir() {
            assert_eq!(
                default_template_dir(),
                PathBuf::from("templates")
            );
        }
    }

    /// Tests for the `ConfigBuilder` functionality
    mod builder_tests {
        use super::*;

        #[test]
        fn test_builder_initialization() {
            let builder = Config::builder();
            assert_eq!(builder.site_name, None);
            assert_eq!(builder.site_title, None);
            assert_eq!(builder.site_description, None);
            assert_eq!(builder.language, None);
            assert_eq!(builder.base_url, None);
            assert_eq!(builder.content_dir, None);
            assert_eq!(builder.output_dir, None);
            assert_eq!(builder.template_dir, None);
            assert_eq!(builder.serve_dir, None);
            assert!(!builder.server_enabled);
            assert_eq!(builder.server_port, None);
        }

        #[test]
        fn test_builder_defaults_applied() {
            let config = Config::builder()
                .site_name("Test Site")
                .build()
                .unwrap();

            assert_eq!(config.site_title, default_site_title());
            assert_eq!(
                config.site_description,
                default_site_description()
            );
            assert_eq!(config.language, default_language());
            assert_eq!(config.base_url, default_base_url());
            assert_eq!(config.content_dir, default_content_dir());
            assert_eq!(config.output_dir, default_output_dir());
            assert_eq!(config.template_dir, default_template_dir());
            assert_eq!(config.server_port, default_port());
            assert!(!config.server_enabled);
            assert!(config.serve_dir.is_none());
        }

        #[test]
        fn test_builder_missing_site_name() {
            let result = Config::builder().build();
            assert!(
                result.is_err(),
                "Builder should fail without site_name"
            );
        }

        #[test]
        fn test_builder_empty_values() {
            let result =
                Config::builder().site_name("").site_title("").build();
            assert!(
                result.is_err(),
                "Empty values should fail validation"
            );
        }

        #[test]
        fn test_unique_id_generation() -> Result<()> {
            let config1 =
                Config::builder().site_name("Site 1").build()?;
            let config2 =
                Config::builder().site_name("Site 2").build()?;
            assert_ne!(
                config1.id(),
                config2.id(),
                "IDs should be unique"
            );
            Ok(())
        }

        #[test]
        fn test_builder_long_values() {
            let long_string = "a".repeat(256);
            let result = Config::builder()
                .site_name(&long_string)
                .site_title(&long_string)
                .build();
            assert!(
                result.is_ok(),
                "Long values should not cause validation errors"
            );
        }
    }

    /// Tests for configuration validation
    mod validation_tests {
        use super::*;

        #[test]
        fn test_empty_site_name() {
            let result = Config::builder()
                .site_name("")
                .content_dir("content")
                .build();
            assert!(
                result.is_err(),
                "Empty site name should fail validation"
            );
        }

        #[test]
        fn test_invalid_url_format() {
            let invalid_urls = vec![
                "not-a-url",
                "http://",
                "://invalid",
                "http//missing-colon",
            ];
            for url in invalid_urls {
                let result = Config::builder()
                    .site_name("Test Site")
                    .base_url(url)
                    .build();
                assert!(
                    result.is_err(),
                    "URL '{}' should fail validation",
                    url
                );
            }
        }

        #[test]
        fn test_validate_path_safety_mocked() {
            let path = PathBuf::from("valid/path");
            let result = Config::builder()
                .site_name("Test Site")
                .content_dir(path)
                .build();
            assert!(
                result.is_ok(),
                "Valid path should pass validation"
            );
        }
    }

    /// Tests for `ConfigError` variants
    mod config_error_tests {
        use super::*;

        #[test]
        fn test_config_error_display() {
            let error =
                ConfigError::InvalidSiteName("Empty name".to_string());
            assert_eq!(
                format!("{}", error),
                "Invalid site name: Empty name"
            );
        }

        #[test]
        fn test_invalid_path_error() {
            let error = ConfigError::InvalidPath {
                path: "invalid/path".to_string(),
                details: "Unsafe path detected".to_string(),
            };
            assert_eq!(
                format!("{}", error),
                "Invalid directory path 'invalid/path': Unsafe path detected"
            );
        }

        #[test]
        fn test_file_error_conversion() {
            let io_error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            );
            let error: ConfigError = io_error.into();
            assert_eq!(
                format!("{}", error),
                "Configuration file error: File not found"
            );
        }
    }

    /// Tests for helper methods
    mod helper_method_tests {
        use super::*;

        #[test]
        fn test_is_valid_language_code() {
            let config =
                Config::builder().site_name("Test").build().unwrap();
            assert!(config.is_valid_language_code("en-US"));
            assert!(!config.is_valid_language_code("invalid-code"));
        }

        #[test]
        fn test_is_valid_port() {
            let config =
                Config::builder().site_name("Test").build().unwrap();
            assert!(config.is_valid_port(1024));
            assert!(!config.is_valid_port(1023));
        }
    }

    /// Tests for serialization and deserialization
    mod serialization_tests {
        use super::*;

        #[test]
        fn test_serialization_roundtrip() -> Result<()> {
            let original = Config::builder()
                .site_name("Test Site")
                .site_title("Roundtrip Test")
                .build()?;

            let serialized = toml::to_string(&original)?;
            let deserialized: Config = toml::from_str(&serialized)?;

            assert_eq!(original.site_name, deserialized.site_name);
            assert_eq!(original.site_title, deserialized.site_title);
            assert_eq!(original.id(), deserialized.id());
            Ok(())
        }
    }

    /// Tests for file operations
    mod file_tests {
        use super::*;

        #[test]
        fn test_missing_config_file() {
            let result =
                Config::from_file(Path::new("nonexistent.toml"));
            assert!(
                result.is_err(),
                "Missing file should fail to load"
            );
        }

        #[test]
        fn test_invalid_toml_file() -> Result<()> {
            let dir = tempdir()?;
            let config_path = dir.path().join("invalid_config.toml");

            std::fs::write(&config_path, "invalid_toml_syntax")?;

            let result = Config::from_file(&config_path);
            assert!(result.is_err(), "Invalid TOML syntax should fail");
            Ok(())
        }
    }

    /// Miscellaneous utility tests
    mod utility_tests {
        use super::*;

        #[test]
        fn test_config_display_format() {
            let config = Config::builder()
                .site_name("Test Site")
                .site_title("Display Title")
                .content_dir("test_content")
                .output_dir("test_output")
                .template_dir("test_templates")
                .build()
                .unwrap();

            let display = format!("{}", config);
            assert!(display.contains("Site: Test Site (Display Title)"));
            assert!(display.contains("Content: test_content"));
            assert!(display.contains("Output: test_output"));
            assert!(display.contains("Templates: test_templates"));
        }

        #[test]
        fn test_clone_retains_all_fields() -> Result<()> {
            let original = Config::builder()
                .site_name("Original")
                .site_title("Clone Test")
                .build()?;

            let cloned = original.clone();

            assert_eq!(original.site_name, cloned.site_name);
            assert_eq!(original.site_title, cloned.site_title);
            assert_eq!(original.id(), cloned.id());
            Ok(())
        }
    }
}
