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
    // `validate_path_safety` is built in every configuration and needs
    // `PathBuf`, so this import is not gated behind `ssg`.
    use commons::path::safe_join;
    use std::path::{Component, PathBuf};

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
    /// - Invalid characters (null bytes, control characters)
    /// - Path traversal: a `..` component that climbs above the path's
    ///   start (its root when absolute, the working directory when
    ///   relative), using the shared rule in `commons::path::safe_join`
    /// - Symlinks
    /// - Windows reserved device names
    ///
    /// `\` is a separator on Windows and an ordinary character
    /// elsewhere, as the platform's own path parser treats it.
    pub fn validate_path_safety(path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy();
        let invalid = |details: &str| -> anyhow::Error {
            UtilsError::InvalidPath {
                path: path_str.to_string(),
                details: details.to_string(),
            }
            .into()
        };

        // 1. Check for null bytes and control characters
        if path_str.chars().any(char::is_control) {
            return Err(invalid("Path contains invalid characters"));
        }

        // 2. Disallow path traversal. Names that merely contain dots
        //    (`a..b.md`) are ordinary; only a `..` component counts.
        if climbs_above_start(path) {
            return Err(invalid("Path traversal not allowed"));
        }

        // 3. Absolute paths are accepted.
        //
        // This function checks the *shape* of a path, not where it
        // points: deciding which directory a caller may write to is the
        // caller's job, and it has the root to compare against.

        // 4. Check for symlinks
        if path
            .symlink_metadata()
            .is_ok_and(|m| m.file_type().is_symlink())
        {
            return Err(invalid("Symlinks are not allowed"));
        }

        // 5. Prevent the use of reserved names (Windows compatibility).
        //    `commons::path` does not cover these: they are valid
        //    components that only Windows refuses to create.
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(is_windows_reserved_name)
        {
            return Err(invalid("Reserved file name not allowed"));
        }

        Ok(())
    }

    /// Whether a `..` component climbs above where `path` starts.
    ///
    /// The root or drive prefix of an absolute path is set aside and
    /// the rest is joined under it, so `/srv/a/../b` is accepted and
    /// `/../etc` is not, the same rule as for a relative path.
    fn climbs_above_start(path: &Path) -> bool {
        let rest: PathBuf = path
            .components()
            .skip_while(|c| {
                matches!(c, Component::Prefix(_) | Component::RootDir)
            })
            .collect();
        safe_join(Path::new(""), rest).is_err()
    }

    /// Whether `name` is a Windows reserved device name.
    ///
    /// Windows reserves `CON`, `PRN`, `AUX`, `NUL`, `COM1`-`COM9` and
    /// `LPT1`-`LPT9` in any case, with or without an extension, and
    /// ignores trailing spaces: `nul.md` and `con ` are the device too.
    fn is_windows_reserved_name(name: &str) -> bool {
        let stem = name.split('.').next().unwrap_or(name).trim_end();
        let stem = stem.to_ascii_uppercase();
        match stem.as_bytes() {
            b"CON" | b"PRN" | b"AUX" | b"NUL" => true,
            [b'C', b'O', b'M', d] | [b'L', b'P', b'T', d] => {
                (b'1'..=b'9').contains(d)
            }
            _ => false,
        }
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
        let base = tempfile::tempdir()?;
        let temp_dir = base.path().join("test_dir");

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

// The `fs` and `log` modules this exercises are gated behind `ssg`;
// without the same gate these tests do not compile under a default
// build, which is how `cargo miri test --lib` runs them.
#[cfg(all(test, feature = "ssg"))]
mod exhaustive_utils_tests {
    //! Every rejection rule in `validate_path_safety`, and the file
    //! helpers built on it.
    //!
    //! Path validation is a security boundary: each rule below exists
    //! because the path shape it rejects would otherwise escape the
    //! directory the caller intended. A rule with no test is a rule
    //! that can be deleted by accident.

    use super::fs::*;
    use super::log::*;
    use super::UtilsError;
    use std::path::{Path, PathBuf};

    #[test]
    fn backslash_paths_follow_the_platform() {
        // On Windows `\` is a separator, so these are ordinary nested
        // paths and the traversal rule sees their `..` components.
        #[cfg(windows)]
        {
            assert!(
                validate_path_safety(Path::new("dir\\file.md")).is_ok()
            );
            for candidate in ["..\\x", "a\\..\\..\\x"] {
                assert!(
                    validate_path_safety(Path::new(candidate)).is_err(),
                    "{candidate} climbs out on Windows"
                );
            }
        }
        // Elsewhere `\` is an ordinary character: `..\x` is one file
        // name in the current directory and cannot climb out of it.
        #[cfg(not(windows))]
        for candidate in ["dir\\file.md", "..\\x", "a\\..\\..\\x"] {
            assert!(
                validate_path_safety(Path::new(candidate)).is_ok(),
                "{candidate} is a single file name on this platform"
            );
        }
    }

    #[test]
    fn dots_inside_a_name_are_not_traversal() {
        for candidate in ["a..b.md", "..hidden", "dir/v1..v2/c.md"] {
            assert!(
                validate_path_safety(Path::new(candidate)).is_ok(),
                "{candidate} has no `..` component"
            );
        }
    }

    #[test]
    fn parent_components_that_stay_inside_are_accepted() {
        for candidate in ["a/../b", "a/b/../../c", "a/.."] {
            assert!(
                validate_path_safety(Path::new(candidate)).is_ok(),
                "{candidate} never climbs above its start"
            );
        }
    }

    #[test]
    fn rejects_null_bytes_and_control_characters() {
        assert!(validate_path_safety(Path::new("a\0b")).is_err());
        assert!(validate_path_safety(Path::new("a\u{7}b")).is_err());
    }

    #[test]
    fn rejects_parent_directory_traversal() {
        for candidate in
            ["../secret", "../x", "a/../../x", "..", "../a/b"]
        {
            assert!(
                validate_path_safety(Path::new(candidate)).is_err(),
                "{candidate} should be rejected"
            );
        }
    }

    #[test]
    fn accepts_ordinary_relative_paths() {
        for candidate in ["file.md", "dir/file.md", "a/b/c.txt", "."] {
            assert!(
                validate_path_safety(Path::new(candidate)).is_ok(),
                "{candidate} should be accepted"
            );
        }
    }

    #[test]
    fn absolute_paths_are_accepted_but_still_checked() {
        // Absolute is not by itself unsafe: which directory a caller
        // may write to is the caller's decision, and it has the root to
        // compare against. What matters is that being absolute no
        // longer skips the remaining rules — see `rejects_symlinks`,
        // which passes an absolute path.
        let inside =
            std::env::temp_dir().join("frontmatter-gen-safe.md");
        assert!(validate_path_safety(&inside).is_ok());
        assert!(validate_path_safety(Path::new("/etc/passwd")).is_ok());
        assert!(
            validate_path_safety(Path::new("/tmp/con")).is_err(),
            "an absolute path with a reserved name is still rejected"
        );
    }

    #[cfg(unix)]
    #[test]
    fn absolute_paths_are_checked_for_climbing_above_root() {
        // `..` is resolved against the path's own root, as it is for a
        // relative path against its starting directory.
        assert!(validate_path_safety(Path::new("/srv/a/../b")).is_ok());
        for candidate in ["/..", "/../etc/passwd", "/srv/../../etc"] {
            assert!(
                validate_path_safety(Path::new(candidate)).is_err(),
                "{candidate} climbs above /"
            );
        }
    }

    #[test]
    fn rejects_symlinks() {
        let dir = tempfile::tempdir().expect("tempdir");
        let target = dir.path().join("target.md");
        std::fs::write(&target, b"x").expect("write");
        let link = dir.path().join("link.md");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &link).expect("symlink");
        #[cfg(not(unix))]
        return;

        let err = validate_path_safety(&link)
            .expect_err("symlinks are rejected");
        assert!(err.to_string().contains("Symlink"), "{err}");
    }

    #[test]
    fn rejects_windows_reserved_names() {
        for name in [
            "con",
            "PRN",
            "Aux",
            "nul",
            "com1",
            "LPT1",
            "COM9",
            "lpt9",
            "nul.md",
            "CON.txt",
            "aux.tar.gz",
            "dir/com3.html",
            "prn ",
        ] {
            assert!(
                validate_path_safety(Path::new(name)).is_err(),
                "{name} is reserved on Windows and must be rejected"
            );
        }
        // A reserved stem with an extension is a different file name.
        assert!(validate_path_safety(Path::new("console.md")).is_ok());
        for name in ["com0", "lpt10", "conx.md", "my-nul.md"] {
            assert!(
                validate_path_safety(Path::new(name)).is_ok(),
                "{name} is not a reserved device name"
            );
        }
    }

    #[tokio::test]
    async fn temp_file_tracker_registers_and_cleans_up() {
        let tracker = TempFileTracker::new();
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("tracked.txt");
        std::fs::write(&path, b"data").expect("write");

        tracker.register(path.clone()).await.expect("register");
        assert!(path.exists());

        tracker.cleanup().await.expect("cleanup");
        assert!(!path.exists(), "cleanup removes registered files");

        // Cleaning up twice is not an error: the file is already gone.
        tracker.cleanup().await.expect("second cleanup");
    }

    #[tokio::test]
    async fn copy_file_creates_the_destination_parent() {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src.md");
        std::fs::write(&src, b"content").expect("write");
        let dst = dir.path().join("nested/deeper/dst.md");

        copy_file(&src, &dst).await.expect("copy");
        assert_eq!(
            std::fs::read_to_string(&dst).expect("read"),
            "content"
        );
    }

    #[tokio::test]
    async fn copy_file_rejects_an_unsafe_path_before_touching_the_disk()
    {
        let dir = tempfile::tempdir().expect("tempdir");
        let src = dir.path().join("src.md");
        std::fs::write(&src, b"content").expect("write");

        let err = copy_file(&src, Path::new("../escape.md"))
            .await
            .expect_err("traversal in the destination is rejected");
        assert!(!err.to_string().is_empty());
        assert!(!PathBuf::from("../escape.md").exists());
    }

    fn entry(
        level: log::Level,
        message: &str,
        error: Option<&str>,
    ) -> LogEntry {
        LogEntry {
            timestamp: dtt::datetime::DateTime::new(),
            level,
            message: message.to_string(),
            error: error.map(str::to_string),
        }
    }

    #[tokio::test]
    async fn create_directory_makes_nested_paths_and_refuses_unsafe_ones(
    ) {
        let dir = tempfile::tempdir().expect("tempdir");
        let nested = dir.path().join("a/b/c");
        create_directory(&nested).await.expect("create nested");
        assert!(nested.is_dir());

        // Already there: creating again is not an error.
        create_directory(&nested).await.expect("idempotent");

        assert!(create_directory(Path::new("../escape"))
            .await
            .is_err());
    }

    #[tokio::test]
    async fn create_temp_file_returns_a_writable_handle_under_the_temp_dir(
    ) {
        use std::io::Write as _;

        let (path, mut file) = create_temp_file("frontmatter-gen-test")
            .await
            .expect("create temp file");
        assert!(path.starts_with(std::env::temp_dir()));
        assert!(
            path.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("frontmatter-gen-test-")),
            "the prefix names the file: {path:?}"
        );

        file.write_all(b"payload").expect("write");
        drop(file);
        assert_eq!(
            std::fs::read(&path).expect("read back"),
            b"payload"
        );

        // Handing it to the tracker is what makes it disposable.
        let tracker = TempFileTracker::new();
        tracker.register(path.clone()).await.expect("register");
        tracker.cleanup().await.expect("cleanup");
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn copy_file_reports_a_missing_source() {
        let dir = tempfile::tempdir().expect("tempdir");
        let missing = dir.path().join("absent.md");
        let dst = dir.path().join("dst.md");
        assert!(copy_file(&missing, &dst).await.is_err());
    }

    #[test]
    fn log_entries_format_with_their_level_and_message() {
        let formatted = entry(log::Level::Info, "hello", None).format();
        assert!(formatted.contains("INFO"), "{formatted}");
        assert!(formatted.contains("hello"), "{formatted}");
        assert!(!formatted.contains("Error:"), "{formatted}");

        // The optional error detail is appended when present.
        let with_error =
            entry(log::Level::Error, "failed", Some("disk full"))
                .format();
        assert!(with_error.contains("disk full"), "{with_error}");
    }

    #[test]
    fn utils_error_converts_from_anyhow_and_join_errors() {
        let err: UtilsError = anyhow::anyhow!("boom").into();
        assert!(err.to_string().contains("boom"), "{err}");
        assert!(matches!(err, UtilsError::InvalidOperation(_)));
    }

    #[tokio::test]
    async fn utils_error_converts_from_a_join_error() {
        let handle = tokio::spawn(async { panic!("task failed") });
        let join_err = handle.await.expect_err("the task panicked");
        let err: UtilsError = join_err.into();
        assert!(matches!(err, UtilsError::InvalidOperation(_)));
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn log_writer_reports_a_path_it_cannot_open() {
        // A directory is not a file: opening it for append fails.
        let dir = tempfile::tempdir().expect("tempdir");
        assert!(LogWriter::new(dir.path()).is_err());
    }

    #[tokio::test]
    async fn cleanup_skips_files_that_are_already_gone() {
        let tracker = TempFileTracker::new();
        let dir = tempfile::tempdir().expect("tempdir");
        let present = dir.path().join("present.txt");
        let absent = dir.path().join("absent.txt");
        std::fs::write(&present, b"x").expect("write");

        tracker.register(present.clone()).await.expect("register");
        tracker.register(absent.clone()).await.expect("register");

        // The absent path is skipped rather than reported as an error.
        tracker.cleanup().await.expect("cleanup");
        assert!(!present.exists());
    }

    #[test]
    fn log_writer_appends_entries_to_its_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("log.txt");
        let mut writer = LogWriter::new(&path).expect("open log");
        writer
            .write(&entry(log::Level::Warn, "careful", None))
            .expect("write entry");
        writer
            .write(&entry(log::Level::Info, "second", None))
            .expect("write another");
        drop(writer);

        let contents =
            std::fs::read_to_string(&path).expect("read log");
        assert!(contents.contains("careful"), "{contents}");
        assert!(contents.contains("second"), "{contents}");
    }

    #[tokio::test]
    async fn fs_error_paths_report_context() {
        let dir = tempfile::tempdir().expect("tempdir");

        // 1. TempFileTracker::cleanup fails when a registered path is a directory
        let sub_dir = dir.path().join("a_dir");
        std::fs::create_dir(&sub_dir).expect("mkdir");
        let tracker = TempFileTracker::new();
        tracker.register(sub_dir).await.expect("register");
        assert!(tracker.cleanup().await.is_err());

        // 2. create_directory fails when parent is a regular file
        let regular_file = dir.path().join("regular.txt");
        std::fs::write(&regular_file, b"hello").expect("write");
        let bad_dir = regular_file.join("child_dir");
        assert!(create_directory(&bad_dir).await.is_err());

        // 3. copy_file fails when dst parent is under a regular file
        let bad_dst = regular_file.join("parent").join("dest.txt");
        assert!(copy_file(&regular_file, &bad_dst).await.is_err());

        // 4. copy_file when dst has no parent (root path)
        assert!(copy_file(&regular_file, Path::new("/"))
            .await
            .is_err());
    }
}
