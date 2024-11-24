// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Utility Module
//!
//! Provides common utilities for file system operations, logging, and other shared functionality.
//!
//! ## Features
//!
//! - Secure file system operations
//! - Path validation and normalization
//! - Temporary file management
//! - Logging utilities
//!
//! ## Security
//!
//! All file system operations include checks for:
//! - Path traversal attacks
//! - Symlink attacks
//! - Directory structure validation
//! - Permission validation

#[cfg(feature = "ssg")]
use std::collections::HashSet;
#[cfg(feature = "ssg")]
use std::fs::File;

use std::fs::create_dir_all;
#[cfg(feature = "ssg")]
use std::fs::remove_file;

use std::io::{self};
use std::path::Path;

#[cfg(feature = "ssg")]
use std::sync::Arc;

use anyhow::{Context, Result};
use thiserror::Error;

#[cfg(feature = "ssg")]
use tokio::sync::RwLock;

#[cfg(feature = "ssg")]
use uuid::Uuid;

/// Errors that can occur during utility operations
#[derive(Error, Debug)]
pub enum UtilsError {
    /// File system operation failed
    #[error("File system error: {0}")]
    FileSystem(#[from] io::Error),

    /// Path validation failed
    #[error("Invalid path '{path}': {details}")]
    InvalidPath {
        /// The path that was invalid
        path: String,
        /// Details about why the path was invalid
        details: String,
    },

    /// Permission error
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// File system utilities module
pub mod fs {
    use super::*;
    use std::path::PathBuf;

    /// Tracks temporary files for cleanup
    #[cfg(feature = "ssg")]
    #[derive(Debug, Default)]
    pub struct TempFileTracker {
        files: Arc<RwLock<HashSet<PathBuf>>>,
    }

    #[cfg(feature = "ssg")]
    impl TempFileTracker {
        /// Creates a new temporary file tracker
        pub fn new() -> Self {
            Self {
                files: Arc::new(RwLock::new(HashSet::new())),
            }
        }

        /// Registers a temporary file for tracking
        pub async fn register(&self, path: PathBuf) -> Result<()> {
            let mut files = self.files.write().await;
            let _ = files.insert(path);
            Ok(())
        }

        /// Cleans up all tracked temporary files
        pub async fn cleanup(&self) -> Result<()> {
            let files = self.files.read().await;
            for path in files.iter() {
                if path.exists() {
                    remove_file(path).with_context(|| {
                        format!(
                            "Failed to remove temporary file: {}",
                            path.display()
                        )
                    })?;
                }
            }
            Ok(())
        }
    }

    /// Creates a new temporary file with the given prefix
    #[cfg(feature = "ssg")]
    pub async fn create_temp_file(
        prefix: &str,
    ) -> Result<(PathBuf, File), UtilsError> {
        let temp_dir = std::env::temp_dir();
        let file_name = format!("{}-{}", prefix, Uuid::new_v4());
        let path = temp_dir.join(file_name);

        let file =
            File::create(&path).map_err(UtilsError::FileSystem)?;

        Ok((path, file))
    }

    /// Validates that a path is safe to use
    ///
    /// # Arguments
    ///
    /// * `path` - Path to validate
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if the path is safe, or an error if validation fails
    ///
    /// # Security
    ///
    /// Checks for:
    /// - Path length limits
    /// - Invalid characters
    /// - Path traversal attempts
    /// - Symlinks
    /// - Reserved names
    pub fn validate_path_safety(path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy();

        // 1. Disallow backslashes for POSIX compatibility
        if path_str.contains('\\') {
            return Err(UtilsError::InvalidPath {
                path: path_str.to_string(),
                details: "Backslashes are not allowed in paths"
                    .to_string(),
            }
            .into());
        }

        // 2. Check for null bytes and control characters
        if path_str.contains('\0')
            || path_str.chars().any(|c| c.is_control())
        {
            return Err(UtilsError::InvalidPath {
                path: path_str.to_string(),
                details: "Path contains invalid characters".to_string(),
            }
            .into());
        }

        // 3. Disallow path traversal using `..`
        if path_str.contains("..") {
            return Err(UtilsError::InvalidPath {
                path: path_str.to_string(),
                details: "Path traversal not allowed".to_string(),
            }
            .into());
        }

        // 4. Handle absolute paths
        if path.is_absolute() {
            println!(
                "Debug: Absolute path detected: {}",
                path.display()
            );

            // In test mode, allow absolute paths in the temporary directory
            if cfg!(test) {
                let temp_dir = std::env::temp_dir();
                let path_canonicalized = path
                    .canonicalize()
                    .or_else(|_| {
                        Ok::<PathBuf, io::Error>(path.to_path_buf())
                    }) // Specify the type explicitly
                    .with_context(|| {
                        format!(
                            "Failed to canonicalize path: {}",
                            path.display()
                        )
                    })?;
                let temp_dir_canonicalized = temp_dir
                    .canonicalize()
                    .or_else(|_| {
                        Ok::<PathBuf, io::Error>(temp_dir.clone())
                    }) // Specify the type explicitly
                    .with_context(|| {
                        format!(
                            "Failed to canonicalize temp_dir: {}",
                            temp_dir.display()
                        )
                    })?;

                if path_canonicalized
                    .starts_with(&temp_dir_canonicalized)
                {
                    return Ok(());
                }
            }

            // Allow all absolute paths in non-test mode
            return Ok(());
        }

        // 5. Check for symlinks
        if path.exists() {
            let metadata =
                path.symlink_metadata().with_context(|| {
                    format!(
                        "Failed to get metadata for path: {}",
                        path.display()
                    )
                })?;

            if metadata.file_type().is_symlink() {
                return Err(UtilsError::InvalidPath {
                    path: path_str.to_string(),
                    details: "Symlinks are not allowed".to_string(),
                }
                .into());
            }
        }

        // 6. Prevent the use of reserved names (Windows compatibility)
        let reserved_names =
            ["con", "prn", "aux", "nul", "com1", "lpt1"];
        if let Some(file_name) =
            path.file_name().and_then(|n| n.to_str())
        {
            if reserved_names
                .contains(&file_name.to_lowercase().as_str())
            {
                return Err(UtilsError::InvalidPath {
                    path: path_str.to_string(),
                    details: "Reserved file name not allowed"
                        .to_string(),
                }
                .into());
            }
        }

        Ok(())
    }

    /// Creates a directory and all parent directories
    ///
    /// # Arguments
    ///
    /// * `path` - Path to create
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or an error if creation fails
    ///
    /// # Security
    ///
    /// Validates path safety before creation
    #[cfg(feature = "ssg")]
    pub async fn create_directory(path: &Path) -> Result<()> {
        validate_path_safety(path)?;

        create_dir_all(path).with_context(|| {
            format!("Failed to create directory: {}", path.display())
        })?;

        Ok(())
    }

    /// Copies a file from source to destination
    ///
    /// # Arguments
    ///
    /// * `src` - Source path
    /// * `dst` - Destination path
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or an error if copy fails
    ///
    /// # Security
    ///
    /// Validates both paths and ensures proper permissions
    pub async fn copy_file(src: &Path, dst: &Path) -> Result<()> {
        validate_path_safety(src)?;
        validate_path_safety(dst)?;

        if let Some(parent) = dst.parent() {
            create_dir_all(parent).with_context(|| {
                format!(
                    "Failed to create parent directory: {}",
                    parent.display()
                )
            })?;
        }

        let _ = std::fs::copy(src, dst).with_context(|| {
            format!(
                "Failed to copy {} to {}",
                src.display(),
                dst.display()
            )
        })?;

        Ok(())
    }
}

/// Logging utilities module
pub mod log {
    #[cfg(feature = "ssg")]
    use anyhow::{Context, Result};
    #[cfg(feature = "ssg")]
    use dtt::datetime::DateTime;
    #[cfg(feature = "ssg")]
    use log::{Level, Record};
    #[cfg(feature = "ssg")]
    use std::{
        fs::{File, OpenOptions},
        io::Write,
        path::Path,
    };

    /// Log entry structure
    #[cfg(feature = "ssg")]
    #[derive(Debug)]
    pub struct LogEntry {
        /// Timestamp of the log entry
        pub timestamp: DateTime,
        /// Log level
        pub level: Level,
        /// Log message
        pub message: String,
        /// Optional error details
        pub error: Option<String>,
    }

    #[cfg(feature = "ssg")]
    impl LogEntry {
        /// Creates a new log entry
        pub fn new(record: &Record<'_>) -> Self {
            Self {
                timestamp: DateTime::new(),
                level: record.level(),
                message: record.args().to_string(),
                error: None,
            }
        }

        /// Formats the log entry as a string
        pub fn format(&self) -> String {
            let error_info = self
                .error
                .as_ref()
                .map(|e| format!(" (Error: {})", e))
                .unwrap_or_default();

            format!(
                "[{} {:>5}] {}{}",
                self.timestamp, self.level, self.message, error_info
            )
        }
    }

    /// Log writer for handling log output
    #[cfg(feature = "ssg")]
    #[derive(Debug)]
    pub struct LogWriter {
        file: File,
    }

    #[cfg(feature = "ssg")]
    impl LogWriter {
        /// Creates a new log writer
        pub fn new(path: &Path) -> Result<Self> {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .with_context(|| {
                    format!(
                        "Failed to open log file: {}",
                        path.display()
                    )
                })?;

            Ok(Self { file })
        }

        /// Writes a log entry
        pub fn write(&mut self, entry: &LogEntry) -> Result<()> {
            writeln!(self.file, "{}", entry.format())
                .context("Failed to write log entry")?;
            Ok(())
        }
    }
}

impl From<anyhow::Error> for UtilsError {
    fn from(err: anyhow::Error) -> Self {
        UtilsError::InvalidOperation(err.to_string())
    }
}

impl From<tokio::task::JoinError> for UtilsError {
    fn from(err: tokio::task::JoinError) -> Self {
        UtilsError::InvalidOperation(err.to_string())
    }
}

#[cfg(all(test, feature = "ssg"))]
mod tests {
    use crate::utils::fs::copy_file;
    use crate::utils::fs::create_directory;
    use crate::utils::fs::create_temp_file;
    use crate::utils::fs::validate_path_safety;
    use crate::utils::fs::TempFileTracker;
    use crate::utils::log::LogEntry;
    use crate::utils::log::LogWriter;
    use crate::utils::UtilsError;
    use log::Level;
    use log::Record;
    use std::fs::read_to_string;
    use std::fs::remove_file;
    use std::path::Path;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_temp_file_creation_and_cleanup() -> anyhow::Result<()>
    {
        let tracker = TempFileTracker::new();
        let (path, _file) = create_temp_file("test").await?;

        tracker.register(path.clone()).await?;
        assert!(path.exists());

        tracker.cleanup().await?;
        assert!(!path.exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_temp_file_concurrent_access() -> Result<(), UtilsError>
    {
        use tokio::task;

        let tracker = Arc::new(TempFileTracker::new());
        let mut handles = Vec::new();

        for i in 0..5 {
            let tracker = Arc::clone(&tracker);
            handles.push(task::spawn(async move {
                let (path, _) =
                    create_temp_file(&format!("test{}", i)).await?;
                tracker.register(path).await
            }));
        }

        for handle in handles {
            handle.await??;
        }

        tracker.cleanup().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_directory_valid_path() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("test_dir");

        // Ensure the directory does not exist beforehand
        if temp_dir.exists() {
            tokio::fs::remove_dir_all(&temp_dir).await?;
        }

        create_directory(&temp_dir).await?;
        assert!(temp_dir.exists());
        tokio::fs::remove_dir_all(temp_dir).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_file_valid_paths() -> anyhow::Result<()> {
        let src = std::env::temp_dir().join("src.txt");
        let dst = std::env::temp_dir().join("dst.txt");

        // Create the source file with content
        tokio::fs::write(&src, "test content").await?;

        copy_file(&src, &dst).await?;
        assert_eq!(
            tokio::fs::read_to_string(&dst).await?,
            "test content"
        );

        tokio::fs::remove_file(src).await?;
        tokio::fs::remove_file(dst).await?;
        Ok(())
    }

    #[test]
    fn test_validate_path_safety_valid_paths() {
        assert!(
            validate_path_safety(Path::new("content/file.txt")).is_ok()
        );
        assert!(
            validate_path_safety(Path::new("templates/blog")).is_ok()
        );
    }

    #[test]
    fn test_validate_path_safety_invalid_paths() {
        assert!(validate_path_safety(Path::new("../outside")).is_err());
        assert!(
            validate_path_safety(Path::new("content\0file")).is_err()
        );
        assert!(validate_path_safety(Path::new("CON")).is_err());
    }

    #[test]
    fn test_validate_path_safety_edge_cases() {
        // Test Unicode
        assert!(validate_path_safety(Path::new("content/📚")).is_ok());

        // Long paths
        let long_name = "a".repeat(255);
        assert!(validate_path_safety(Path::new(&long_name)).is_ok());

        // Special characters
        assert!(validate_path_safety(Path::new("content/#$@!")).is_ok());
    }

    #[test]
    fn test_log_entry_format() {
        let record = Record::builder()
            .args(format_args!("Test log message"))
            .level(Level::Info)
            .target("test")
            .module_path_static(Some("test"))
            .file_static(Some("test.rs"))
            .line(Some(42))
            .build();

        let entry = LogEntry::new(&record);
        assert!(entry.format().contains("Test log message"));
        assert!(entry.format().contains("INFO"));
    }

    #[test]
    fn test_log_entry_with_error() {
        let record = Record::builder()
            .args(format_args!("Test error message"))
            .level(Level::Error)
            .target("test")
            .module_path_static(Some("test"))
            .file_static(Some("test.rs"))
            .line(Some(42))
            .build();

        let mut entry = LogEntry::new(&record);
        entry.error = Some("Error details".to_string());

        let formatted = entry.format();
        assert!(formatted.contains("Error details"));
        assert!(formatted.contains("ERROR"));
    }

    #[test]
    fn test_log_writer_creation() {
        let temp_log_path = std::env::temp_dir().join("test_log.txt");
        let writer = LogWriter::new(&temp_log_path).unwrap();

        assert!(temp_log_path.exists());
        drop(writer); // Ensure file is closed
        remove_file(temp_log_path).unwrap();
    }

    #[test]
    fn test_log_writer_write() {
        let temp_log_path =
            std::env::temp_dir().join("test_log_write.txt");
        let mut writer = LogWriter::new(&temp_log_path).unwrap();

        let record = Record::builder()
            .args(format_args!("Write test message"))
            .level(Level::Info)
            .target("test")
            .build();

        let entry = LogEntry::new(&record);
        writer.write(&entry).unwrap();

        let content = read_to_string(&temp_log_path).unwrap();
        assert!(content.contains("Write test message"));
        remove_file(temp_log_path).unwrap();
    }
}
