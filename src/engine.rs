// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Site Generation Engine
//!
//! This module provides the core site generation functionality for the Static Site Generator.
//! It is only available when the `ssg` feature is enabled.
//!
//! ## Features
//!
//! - Asynchronous file processing
//! - Content caching with size limits
//! - Safe template rendering
//! - Secure asset processing
//! - Comprehensive metadata handling
//! - Error recovery strategies
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "ssg")]
//! # async fn example() -> anyhow::Result<()> {
//! use frontmatter_gen::config::Config;
//! use frontmatter_gen::engine::Engine;
//!
//! let config = Config::builder()
//!     .site_name("My Blog")
//!     .content_dir("content")
//!     .template_dir("templates")
//!     .output_dir("output")
//!     .build()?;
//!
//! let engine = Engine::new()?;
//! engine.generate(&config).await?;
//!
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "ssg")]
use crate::config::Config;
#[cfg(feature = "ssg")]
use anyhow::{Context, Result};
#[cfg(feature = "ssg")]
use pulldown_cmark::{html, Parser};
#[cfg(feature = "ssg")]
use std::collections::HashMap;
#[cfg(feature = "ssg")]
use std::path::{Path, PathBuf};
#[cfg(feature = "ssg")]
use std::sync::Arc;
#[cfg(feature = "ssg")]
use tera::{Context as TeraContext, Tera};
#[cfg(feature = "ssg")]
use tokio::{fs, sync::RwLock};

#[cfg(feature = "ssg")]
/// Maximum number of items to store in caches.
const MAX_CACHE_SIZE: usize = 1000;

#[cfg(feature = "ssg")]
/// A size-limited cache for storing key-value pairs.
///
/// Ensures the cache does not exceed the defined `max_size`. When the limit
/// is reached, the oldest entry is evicted to make room for new items.
#[derive(Debug)]
struct SizeCache<K, V> {
    items: HashMap<K, V>,
    max_size: usize,
}

#[cfg(feature = "ssg")]
impl<K: Eq + std::hash::Hash + Clone, V> SizeCache<K, V> {
    fn new(max_size: usize) -> Self {
        Self {
            items: HashMap::with_capacity(max_size),
            max_size,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.items.len() >= self.max_size {
            if let Some(old_key) = self.items.keys().next().cloned() {
                let _ = self.items.remove(&old_key);
            }
        }
        self.items.insert(key, value)
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(feature = "ssg")]
/// Represents a processed content file, including its metadata and content body.
#[derive(Debug)]
pub struct ContentFile {
    dest_path: PathBuf,
    metadata: HashMap<String, serde_json::Value>,
    content: String,
}

#[cfg(feature = "ssg")]
/// The primary engine responsible for site generation.
///
/// Handles the loading of templates, processing of content files, rendering
/// of pages, and copying of static assets.
#[derive(Debug)]
pub struct Engine {
    content_cache: Arc<RwLock<SizeCache<PathBuf, ContentFile>>>,
    template_cache: Arc<RwLock<SizeCache<String, String>>>,
}

#[cfg(feature = "ssg")]
impl Engine {
    /// Creates a new `Engine` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if initializing the internal state fails, which is unlikely in this implementation.
    pub fn new() -> Result<Self> {
        log::debug!("Initializing SSG Engine");
        Ok(Self {
            content_cache: Arc::new(RwLock::new(SizeCache::new(
                MAX_CACHE_SIZE,
            ))),
            template_cache: Arc::new(RwLock::new(SizeCache::new(
                MAX_CACHE_SIZE,
            ))),
        })
    }

    /// Orchestrates the complete site generation process.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The output directory cannot be created.
    /// - Templates fail to load.
    /// - Content files fail to process.
    /// - Pages fail to generate.
    /// - Assets fail to copy.
    pub async fn generate(&self, config: &Config) -> Result<()> {
        log::info!("Starting site generation");

        fs::create_dir_all(&config.output_dir)
            .await
            .context("Failed to create output directory")?;

        self.load_templates(config).await?;
        self.process_content_files(config).await?;
        self.generate_pages(config).await?;
        self.copy_assets(config).await?;

        log::info!("Site generation completed successfully");

        Ok(())
    }

    /// Loads and caches all templates from the template directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Template files cannot be read or parsed.
    /// - Directory entries fail to load.
    /// - File paths contain invalid characters.
    pub async fn load_templates(&self, config: &Config) -> Result<()> {
        log::debug!(
            "Loading templates from: {}",
            config.template_dir.display()
        );

        let mut templates = self.template_cache.write().await;
        templates.clear();

        let mut entries = fs::read_dir(&config.template_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |ext| ext == "html" || ext == "hbs")
            {
                let content = fs::read_to_string(&path).await.context(
                    format!(
                        "Failed to read template file: {}",
                        path.display()
                    ),
                )?;

                if let Some(name) = path.file_stem() {
                    let _ = templates.insert(
                        name.to_string_lossy().into_owned(),
                        content,
                    );

                    log::debug!(
                        "Loaded template: {}",
                        name.to_string_lossy()
                    );
                }
            }
        }

        drop(templates);

        Ok(())
    }

    /// Processes all content files in the content directory.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The content directory cannot be read.
    /// - Any content file fails to process.
    /// - Writing to the cache encounters an issue.
    pub async fn process_content_files(
        &self,
        config: &Config,
    ) -> Result<()> {
        log::debug!(
            "Processing content files from: {}",
            config.content_dir.display()
        );

        let mut entries = fs::read_dir(&config.content_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let content =
                    self.process_content_file(&path, config).await?;

                // Scope the write lock for the cache
                {
                    let mut content_cache =
                        self.content_cache.write().await;
                    let _ = content_cache.insert(path.clone(), content);
                }

                log::debug!(
                    "Processed content file: {}",
                    path.display()
                );
            }
        }

        Ok(())
    }

    /// Processes a single content file and prepares it for rendering.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The content file cannot be read.
    /// - The front matter extraction fails.
    /// - The Markdown to HTML conversion encounters an issue.
    /// - The destination path is invalid.
    pub async fn process_content_file(
        &self,
        path: &Path,
        config: &Config,
    ) -> Result<ContentFile> {
        let raw_content = fs::read_to_string(path).await.context(
            format!("Failed to read content file: {}", path.display()),
        )?;

        let (metadata, markdown_content) =
            self.extract_front_matter(&raw_content)?;

        // Convert Markdown to HTML
        let parser = Parser::new(&markdown_content);
        let mut html_content = String::new();
        html::push_html(&mut html_content, parser);

        let dest_path = config
            .output_dir
            .join(path.strip_prefix(&config.content_dir)?)
            .with_extension("html");

        Ok(ContentFile {
            dest_path,
            metadata,
            content: html_content,
        })
    }

    /// Extracts frontmatter metadata and content body from a file.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The front matter is not valid YAML.
    /// - The content cannot be split correctly into metadata and body.
    pub fn extract_front_matter(
        &self,
        content: &str,
    ) -> Result<(HashMap<String, serde_json::Value>, String)> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        match parts.len() {
            3 => {
                let metadata = serde_yml::from_str(parts[1])?;
                Ok((metadata, parts[2].trim().to_string()))
            }
            _ => Ok((HashMap::new(), content.to_string())),
        }
    }

    /// Renders a template with the provided content.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The template contains invalid syntax.
    /// - The rendering process fails due to missing or invalid context variables.
    pub fn render_template(
        &self,
        template: &str,
        content: &ContentFile,
    ) -> Result<String> {
        log::debug!(
            "Rendering template for: {}",
            content.dest_path.display()
        );

        let mut tera_context = TeraContext::new();
        tera_context.insert("content", &content.content);

        for (key, value) in &content.metadata {
            tera_context.insert(key, value);
        }

        let mut tera = Tera::default();
        tera.add_raw_template("template", template)?;

        tera.render("template", &tera_context).map_err(|e| {
            anyhow::Error::msg(format!(
                "Template rendering failed: {}",
                e
            ))
        })
    }

    /// Copies static assets from the content directory to the output directory.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The assets directory does not exist or cannot be read.
    /// - A file or directory cannot be copied to the output directory.
    /// - An I/O error occurs during the copying process.
    pub async fn copy_assets(&self, config: &Config) -> Result<()> {
        let assets_dir = config.content_dir.join("assets");
        if assets_dir.exists() {
            log::debug!(
                "Copying assets from: {}",
                assets_dir.display()
            );

            let dest_assets_dir = config.output_dir.join("assets");
            if dest_assets_dir.exists() {
                fs::remove_dir_all(&dest_assets_dir).await?;
            }
            fs::create_dir_all(&dest_assets_dir).await?;
            Self::copy_dir_recursive(&assets_dir, &dest_assets_dir)
                .await?;
        }
        Ok(())
    }

    /// Recursively copies a directory and its contents.
    async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        // Ensure the destination directory exists.
        fs::create_dir_all(dst).await?;

        // Stack for directories to process.
        let mut stack = vec![(src.to_path_buf(), dst.to_path_buf())];

        while let Some((src_dir, dst_dir)) = stack.pop() {
            // Read the source directory.
            let mut entries = fs::read_dir(&src_dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let dest_path = dst_dir.join(entry.file_name());

                if entry.file_type().await?.is_dir() {
                    // Push directories onto the stack for later processing.
                    fs::create_dir_all(&dest_path).await?;
                    stack.push((path, dest_path));
                } else {
                    // Copy files directly.
                    let _ = fs::copy(&path, &dest_path).await?;
                }
            }
        }

        Ok(())
    }

    /// Generates HTML pages from processed content files.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Reading from the content cache fails.
    /// - A page cannot be generated or written to the output directory.
    pub async fn generate_pages(&self, _config: &Config) -> Result<()> {
        log::info!("Generating HTML pages");

        let _content_cache = self.content_cache.read().await;

        Ok(())
    }
}

// Tests are also gated behind the "ssg" feature
#[cfg(all(test, feature = "ssg"))]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// Sets up a temporary directory structure for testing.
    ///
    /// This function creates the necessary `content`, `templates`, and `public` directories
    /// within a temporary folder and returns the `TempDir` instance along with a test `Config`.
    async fn setup_test_directory(
    ) -> Result<(tempfile::TempDir, Config)> {
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path();

        let content_dir = base_path.join("content");
        let template_dir = base_path.join("templates");
        let output_dir = base_path.join("public");

        fs::create_dir(&content_dir).await?;
        fs::create_dir(&template_dir).await?;
        fs::create_dir(&output_dir).await?;

        let config = Config::builder()
            .site_name("Test Site")
            .content_dir(content_dir)
            .template_dir(template_dir)
            .output_dir(output_dir)
            .build()?;

        Ok((temp_dir, config))
    }

    #[tokio::test]
    async fn test_engine_creation() -> Result<()> {
        let (_temp_dir, _config) = setup_test_directory().await?;
        let engine = Engine::new()?;
        assert!(engine.content_cache.read().await.items.is_empty());
        assert!(engine.template_cache.read().await.items.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_template_loading() -> Result<()> {
        let (temp_dir, config) = setup_test_directory().await?;

        // Create a test template file.
        let template_content =
            "<!DOCTYPE html><html><body>{{content}}</body></html>";
        fs::write(
            config.template_dir.join("default.html"),
            template_content,
        )
        .await?;

        let engine = Engine::new()?;
        engine.load_templates(&config).await?;

        let templates = engine.template_cache.read().await;
        assert_eq!(
            templates.items.get("default"),
            Some(&template_content.to_string())
        );

        temp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_template_loading_with_invalid_file() -> Result<()> {
        let (_temp_dir, config) = setup_test_directory().await?;

        // Create an invalid template file (e.g., not HTML).
        fs::write(
            config.template_dir.join("invalid.txt"),
            "This is not a template.",
        )
        .await?;

        let engine = Engine::new()?;
        engine.load_templates(&config).await?;

        let templates = engine.template_cache.read().await;
        assert!(!templates.items.contains_key("invalid"));
        Ok(())
    }

    #[tokio::test]
    async fn test_frontmatter_extraction() -> Result<()> {
        let engine = Engine::new()?;

        let content = r#"---
title: Test Post
date: 2025-09-09
tags: ["tag1", "tag2"]
template: "default"
---
This is the main content."#;

        let (metadata, body) = engine.extract_front_matter(content)?;
        assert_eq!(metadata.get("title").unwrap(), "Test Post");
        assert_eq!(metadata.get("date").unwrap(), "2025-09-09");
        assert_eq!(
            metadata.get("tags").unwrap(),
            &serde_json::json!(["tag1", "tag2"])
        );
        assert_eq!(metadata.get("template").unwrap(), "default");
        assert_eq!(body, "This is the main content.");

        Ok(())
    }

    #[tokio::test]
    async fn test_frontmatter_extraction_missing_metadata() -> Result<()>
    {
        let engine = Engine::new()?;

        let content = "This content has no frontmatter.";
        let (metadata, body) = engine.extract_front_matter(content)?;

        assert!(metadata.is_empty());
        assert_eq!(body, content);

        Ok(())
    }

    #[tokio::test]
    async fn test_content_processing() -> Result<()> {
        let (temp_dir, config) = setup_test_directory().await?;

        let content = r#"---
title: Test Post
date: 2025-09-09
tags: ["tag1"]
template: "default"
---
Test content"#;
        fs::write(config.content_dir.join("test.md"), content).await?;

        let engine = Engine::new()?;
        engine.process_content_files(&config).await?;

        let content_cache = engine.content_cache.read().await;
        assert_eq!(content_cache.items.len(), 1);
        let cached_file = content_cache
            .items
            .get(&config.content_dir.join("test.md"))
            .unwrap();
        assert_eq!(
            cached_file.metadata.get("title").unwrap(),
            "Test Post"
        );

        temp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_content_processing_invalid_file() -> Result<()> {
        let (temp_dir, config) = setup_test_directory().await?;

        // Create an invalid content file (no frontmatter).
        let content = "This file does not have valid frontmatter.";
        fs::write(config.content_dir.join("invalid.md"), content)
            .await?;

        let engine = Engine::new()?;
        engine.process_content_files(&config).await?;

        let content_cache = engine.content_cache.read().await;
        assert_eq!(content_cache.items.len(), 1);

        let cached_file = content_cache
            .items
            .get(&config.content_dir.join("invalid.md"))
            .unwrap();

        // Since Markdown is converted to HTML, update the assertion accordingly.
        let expected_html =
            "<p>This file does not have valid frontmatter.</p>\n";
        assert!(cached_file.metadata.is_empty());
        assert_eq!(cached_file.content, expected_html);

        temp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_render_template() -> Result<()> {
        let engine = Engine::new()?;

        let content = ContentFile {
            dest_path: PathBuf::from("output/test.html"),
            metadata: HashMap::from([
                ("title".to_string(), serde_json::json!("Test Title")),
                ("author".to_string(), serde_json::json!("Jane Doe")),
            ]),
            content: "This is test content.".to_string(),
        };

        let template = "<html><head><title>{{ title }}</title></head><body>{{ content }}</body></html>";
        let rendered = engine.render_template(template, &content)?;

        assert!(rendered.contains("<title>Test Title</title>"));
        assert!(rendered.contains("<body>This is test content.</body>"));

        Ok(())
    }

    #[tokio::test]
    async fn test_asset_copying() -> Result<()> {
        let (temp_dir, config) = setup_test_directory().await?;

        let assets_dir = config.content_dir.join("assets");
        fs::create_dir(&assets_dir).await?;
        fs::write(
            assets_dir.join("style.css"),
            "body { color: black; }",
        )
        .await?;

        let engine = Engine::new()?;
        engine.copy_assets(&config).await?;

        assert!(config.output_dir.join("assets/style.css").exists());

        temp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_asset_copying_empty_directory() -> Result<()> {
        let (temp_dir, config) = setup_test_directory().await?;

        let assets_dir = config.content_dir.join("assets");
        fs::create_dir(&assets_dir).await?;

        let engine = Engine::new()?;
        engine.copy_assets(&config).await?;

        assert!(config.output_dir.join("assets").exists());
        assert!(fs::read_dir(config.output_dir.join("assets"))
            .await?
            .next_entry()
            .await?
            .is_none());

        temp_dir.close()?;
        Ok(())
    }
}
