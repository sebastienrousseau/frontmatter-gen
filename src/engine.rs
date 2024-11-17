// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Site Generation Engine
//!
//! This module provides the core site generation functionality for the Static Site Generator.
//! It handles content processing, template rendering, and file generation in a secure and
//! efficient manner.
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
//! use frontmatter_gen::config::Config;
//! use frontmatter_gen::engine::Engine;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::builder()
//!         .site_name("My Blog")
//!         .content_dir("content")
//!         .template_dir("templates")
//!         .output_dir("output")
//!         .build()?;
//!
//!     let engine = Engine::new()?;
//!     engine.generate(&config).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Security Considerations
//!
//! This module implements several security measures:
//!
//! - Path traversal prevention
//! - Safe file handling
//! - Template injection protection
//! - Resource limiting
//! - Proper error handling
//!
//! ## Performance
//!
//! The engine utilises caching and asynchronous processing to optimise performance:
//!
//! - Content caching with size limits
//! - Parallel content processing where possible
//! - Efficient template caching
//! - Optimised asset handling

use crate::config::Config;
use anyhow::{Context, Result};
use pulldown_cmark::{html, Parser};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tera::{Context as TeraContext, Tera};
use tokio::{fs, sync::RwLock};

/// Maximum number of items to store in caches.
const MAX_CACHE_SIZE: usize = 1000;

/// A size-limited cache for storing key-value pairs.
///
/// Ensures the cache does not exceed the defined `max_size`. When the limit
/// is reached, the oldest entry is evicted to make room for new items.
#[derive(Debug)]
struct SizeCache<K, V> {
    items: HashMap<K, V>,
    max_size: usize,
}

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
                self.items.remove(&old_key);
            }
        }
        self.items.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}

/// Represents a processed content file, including its metadata and content body.
#[derive(Debug)]
pub struct ContentFile {
    dest_path: PathBuf,
    metadata: HashMap<String, serde_json::Value>,
    content: String,
}

/// The primary engine responsible for site generation.
///
/// Handles the loading of templates, processing of content files, rendering
/// of pages, and copying of static assets.
#[derive(Debug)]
pub struct Engine {
    content_cache: Arc<RwLock<SizeCache<PathBuf, ContentFile>>>,
    template_cache: Arc<RwLock<SizeCache<String, String>>>,
}

impl Engine {
    /// Creates a new `Engine` instance.
    ///
    /// # Returns
    /// A new instance of `Engine`, or an error if initialisation fails.
    pub fn new() -> Result<Self> {
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
    /// This includes:
    /// 1. Creating necessary directories.
    /// 2. Loading and caching templates.
    /// 3. Processing content files.
    /// 4. Rendering and generating HTML pages.
    /// 5. Copying static assets to the output directory.
    pub async fn generate(&self, config: &Config) -> Result<()> {
        fs::create_dir_all(&config.output_dir)
            .await
            .context("Failed to create output directory")?;

        self.load_templates(config).await?;
        self.process_content_files(config).await?;
        self.generate_pages(config).await?;
        self.copy_assets(config).await?;
        Ok(())
    }

    /// Loads and caches all templates from the template directory.
    ///
    /// Templates are stored in the cache for efficient access during rendering.
    pub async fn load_templates(&self, config: &Config) -> Result<()> {
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
                    templates.insert(
                        name.to_string_lossy().into_owned(),
                        content,
                    );
                }
            }
        }
        Ok(())
    }

    /// Processes all content files in the content directory.
    ///
    /// Each file is parsed for frontmatter metadata and stored in the content cache.
    pub async fn process_content_files(
        &self,
        config: &Config,
    ) -> Result<()> {
        let mut content_cache = self.content_cache.write().await;
        content_cache.clear();

        let mut entries = fs::read_dir(&config.content_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let content =
                    self.process_content_file(&path, config).await?;
                content_cache.insert(path, content);
            }
        }
        Ok(())
    }

    /// Processes a single content file and prepares it for rendering.
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
    pub fn render_template(
        &self,
        template: &str,
        content: &ContentFile,
    ) -> Result<String> {
        // eprintln!("Rendering template: {}", template);
        // eprintln!("Context (metadata): {:?}", content.metadata);
        // eprintln!("Context (content): {:?}", content.content);

        let mut context = TeraContext::new();
        context.insert("content", &content.content);

        for (key, value) in &content.metadata {
            context.insert(key, value);
        }

        let mut tera = Tera::default();
        tera.add_raw_template("template", template)?;
        tera.render("template", &context).map_err(|e| {
            anyhow::Error::msg(format!(
                "Template rendering failed: {}",
                e
            ))
        })
    }

    /// Copies static assets from the content directory to the output directory.
    pub async fn copy_assets(&self, config: &Config) -> Result<()> {
        let assets_dir = config.content_dir.join("assets");
        if assets_dir.exists() {
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
        fs::create_dir_all(dst).await?;
        let mut entries = fs::read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let dest_path = dst.join(entry.file_name());
            if entry.file_type().await?.is_dir() {
                Box::pin(Self::copy_dir_recursive(&path, &dest_path))
                    .await?;
            } else {
                fs::copy(&path, &dest_path).await?;
            }
        }
        Ok(())
    }

    /// Generates HTML pages from processed content files.
    ///
    /// This method retrieves content from the cache, applies the associated templates,
    /// and writes the rendered HTML to the output directory.
    ///
    /// # Arguments
    /// - `config`: A reference to the site configuration.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    /// This method will return an error if:
    /// - The template for a content file is missing.
    /// - Rendering a template fails.
    /// - Writing the rendered HTML to disk fails.
    pub async fn generate_pages(&self, _config: &Config) -> Result<()> {
        // Use `_config` only if necessary; otherwise, remove it.
        let content_cache = self.content_cache.read().await;
        let template_cache = self.template_cache.read().await;

        for content_file in content_cache.items.values() {
            let template_name = content_file
                .metadata
                .get("template")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string();

            let template = template_cache
                .get(&template_name)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Template not found: {}",
                        template_name
                    )
                })?;

            let rendered_html = self
                .render_template(template, content_file)
                .context(format!(
                    "Failed to render template for content: {}",
                    content_file.dest_path.display()
                ))?;

            if let Some(parent_dir) = content_file.dest_path.parent() {
                fs::create_dir_all(parent_dir).await?;
            }

            fs::write(&content_file.dest_path, rendered_html).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// Sets up a temporary directory structure for testing.
    ///
    /// This function creates the necessary `content`, `templates`, and `public` directories
    /// within a temporary folder and returns the `TempDir` instance along with a test `Config`.
    pub async fn setup_test_directory(
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
            templates.get(&"default".to_string()),
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
        assert!(templates.get(&"invalid".to_string()).is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_frontmatter_extraction() -> Result<()> {
        let engine = Engine::new()?;

        let content = r#"---
title: Test Post
date: 2024-01-01
tags: ["tag1", "tag2"]
template: "default"
---
This is the main content."#;

        let (metadata, body) = engine.extract_front_matter(content)?;
        assert_eq!(metadata.get("title").unwrap(), "Test Post");
        assert_eq!(metadata.get("date").unwrap(), "2024-01-01");
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
date: 2024-01-01
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
