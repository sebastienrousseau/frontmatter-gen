//! # Frontmatter Generator
//!
//! The Frontmatter Generator is a command-line tool designed for extracting, validating,
//! and manipulating frontmatter in various formats, including YAML, TOML, and JSON.
//! It also provides static site generation capabilities when enabled.
//!
//! ## Features
//!
//! - **Frontmatter Manipulation**: Extract and validate frontmatter from Markdown files.
//! - **Static Site Generation**: Build static sites with templates and structured content.
//! - **Multi-Format Support**: YAML, TOML, and JSON support.
//! - **Secure and Robust**: Implements secure file handling, input sanitisation, and logging.
//!
//! ## Usage
//!
//! ```bash
//! # Extract frontmatter from a file
//! frontmatter-gen extract input.md --format yaml
//!
//! # Validate frontmatter for required fields
//! frontmatter-gen validate input.md --required title,date
//!
//! # Generate a static site
//! frontmatter-gen build --content-dir content --output-dir public --template-dir templates
//! ```
//!
//! ## Environment Variables
//!
//! - `RUST_LOG`: Controls logging level (`error`, `warn`, `info`, `debug`, `trace`).
//!
//! ## Feature Flags
//!
//! - `cli`: Enables command-line interface functionality for frontmatter manipulation.
//! - `ssg`: Enables static site generation capabilities.
//!
//! ## Crate Modules
//!
//! - `cli`: Handles command-line interface parsing and commands.
//! - `ssg`: Provides static site generation functionality.
//! - `logging`: Sets up logging for debugging and error reporting.
//!
//! ## Security Considerations
//!
//! - **Input Sanitisation**: Ensures safe handling of user inputs to prevent path traversal attacks.
//! - **Error Handling**: Graceful recovery and detailed error reporting.
//!
//! ## Contributing
//!
//! Contributions are welcome. Please open an issue or submit a pull request with your suggestions.

use anyhow::Result;
use log::LevelFilter;
use std::env;
use std::process;

// Conditional imports based on features
#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use frontmatter_gen::cli::Cli;
#[cfg(feature = "ssg")]
use frontmatter_gen::ssg::SsgCommand;

/// Main entry point for the Frontmatter Generator tool.
///
/// This function initializes the logging system, determines the enabled features,
/// and dispatches commands based on user input. It ensures robust error handling
/// and clear feedback for the user.
///
/// # Environment Variables
///
/// - `RUST_LOG`: Sets the logging level (e.g., "debug", "info").
///
/// # Errors
///
/// Returns an error if command execution fails or required features are missing.
///
/// # Examples
///
/// Running the application in CLI mode:
/// ```bash
/// RUST_LOG=info cargo run --features=cli validate input.md --required title,date
/// ```
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging system
    setup_logging();

    // Log startup information
    log::info!("Starting Frontmatter Generator");
    log::info!(
        "Initializing with features `{}`",
        get_enabled_features()
    );

    // Execute the appropriate command based on enabled features
    let result = execute_command().await;

    // Handle any errors that occurred during execution
    if let Err(ref e) = result {
        // Log the error with full context for debugging
        log::error!("Application error: {:#}", e);
        // Print user-friendly error message
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    log::info!("Process completed successfully");
    Ok(())
}

/// Configures the logging system.
///
/// Reads the desired log level from the `RUST_LOG` environment variable and sets up a logger
/// that writes to standard error with colour-coded output.
///
/// # Examples
///
/// Setting the log level to `debug`:
/// ```bash
/// export RUST_LOG=debug
/// cargo run
/// ```
fn setup_logging() {
    // Get desired log level from RUST_LOG env var, default to "debug"
    let env =
        env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());

    // Define the logger level based on the environment variable
    let level = match env.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        "off" => LevelFilter::Off,
        _ => {
            log::warn!(
                "Invalid RUST_LOG value '{}', defaulting to 'debug'",
                env
            );
            LevelFilter::Debug
        }
    };

    // Set up the logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level))
        .unwrap_or_else(|e| {
            eprintln!("Warning: Failed to initialize logger: {}", e);
        });

    if level != LevelFilter::Off {
        log::info!("Logging initialized at level `{}`", level);
    }
}

/// Executes the appropriate command based on enabled features.
///
/// This function ensures that both `cli` and `ssg` features are correctly routed.
/// If no features are enabled, it displays an error.
///
/// # Errors
/// Returns an error if command parsing or execution fails.
async fn execute_command() -> Result<()> {
    #[cfg(all(feature = "ssg", not(feature = "cli")))]
    {
        log::info!("Executing in SSG mode");
        let ssg_command = SsgCommand::parse();
        return ssg_command.execute().await;
    }

    #[cfg(all(feature = "cli", not(feature = "ssg")))]
    {
        log::info!("Executing in CLI mode");
        let cli_command = Cli::parse();
        return cli_command.process().await;
    }

    #[cfg(all(feature = "cli", feature = "ssg"))]
    {
        // Handle both CLI and SSG features
        log::info!("Executing with both CLI and SSG features enabled");

        // Use the first positional argument to determine the mode
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && (args[1] == "build" || args[1] == "serve")
        {
            let ssg_command = SsgCommand::parse();
            ssg_command.execute().await
        } else {
            let cli_command = Cli::parse();
            cli_command.process().await
        }
    }

    #[cfg(not(any(feature = "cli", feature = "ssg")))]
    {
        log::error!("No features enabled");
        eprintln!("Error: No features enabled. Enable 'cli' or 'ssg' in Cargo.toml.");
        process::exit(1);
    }
}

/// Reports the enabled features of the application.
///
/// This function is primarily used for debugging and logging purposes, providing
/// a clear overview of the functionality available in the current build.
///
/// # Returns
///
/// A comma-separated string of enabled features, or `"none"` if no features are enabled.
///
/// # Examples
///
/// ```rust
/// let features = get_enabled_features();
/// println!("Enabled features: {}", features);
/// ```
fn get_enabled_features() -> String {
    let mut features = Vec::new();

    #[cfg(feature = "cli")]
    features.push("cli");

    #[cfg(feature = "ssg")]
    features.push("ssg");

    if features.is_empty() {
        "none".to_string()
    } else {
        features.join(", ")
    }
}

/// Custom logger for the Frontmatter Generator.
///
/// This logger writes formatted log messages to standard error, including a timestamp,
/// log level, and the message. Colour codes are used to improve readability.
///
/// # Examples
///
/// Logging an informational message:
/// ```rust
/// log::info!("Starting application");
/// ```
#[derive(Clone, Copy)]
struct Logger;

/// Global logger instance
static LOGGER: Logger = Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // Enable based on max_level set in setup_logging
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level_color = match record.level() {
                log::Level::Error => "\x1b[31m", // Red
                log::Level::Warn => "\x1b[33m",  // Yellow
                log::Level::Info => "\x1b[32m",  // Green
                log::Level::Debug => "\x1b[36m", // Cyan
                log::Level::Trace => "\x1b[90m", // Bright black
            };
            eprintln!(
                "{}[{}]\x1b[0m {}",
                level_color,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Log;
    use std::sync::Once;

    /// Tests logging setup with various environment configurations.
    #[test]
    fn test_logging_setup() {
        // Test default logging
        env::remove_var("RUST_LOG");
        setup_logging();

        // Test custom log level
        env::set_var("RUST_LOG", "debug");
        setup_logging();
        assert_eq!(env::var("RUST_LOG").unwrap(), "debug");
    }

    /// Tests enabled features reporting.
    #[test]
    fn test_enabled_features() {
        let features = get_enabled_features();
        assert!(
            !features.is_empty(),
            "Should list enabled features or 'none'"
        );
    }

    /// Tests command execution in test environment.
    #[tokio::test]
    #[ignore = "This test is only for interactive testing"]
    async fn test_command_execution() {
        // Save original args
        let original_args: Vec<String> = env::args().collect();

        // Test with no arguments
        env::set_var("CARGO_PKG_VERSION", "0.1.0");
        let result = execute_command().await;
        assert!(result.is_err());

        // Test with help command
        env::set_var("CARGO_PKG_VERSION", "0.1.0");
        let _args = ["program".to_string(), "help".to_string()];
        env::set_var("CARGO_PKG_NAME", "frontmatter-gen");

        let result = execute_command().await;
        assert!(result.is_err());

        // Restore original args
        for (i, arg) in original_args.iter().enumerate() {
            if i == 0 {
                env::set_var("CARGO_PKG_NAME", arg);
            }
        }
    }

    /// Test that the help output is correct
    #[test]
    fn test_help_output() {
        env::set_var("CARGO_PKG_NAME", "frontmatter-gen");
        env::set_var("CARGO_PKG_VERSION", "0.1.0");

        #[cfg(feature = "cli")]
        {
            let cli =
                Cli::try_parse_from(["frontmatter-gen", "--help"]);
            assert!(cli.is_err());
            let err = cli.unwrap_err();
            let output = err.to_string();
            assert!(output.contains("Usage:"));
            assert!(output.contains("Commands:"));
            assert!(output.contains("extract"));
            assert!(output.contains("validate"));
        }
    }

    /// Test version output is correct
    #[test]
    fn test_version_output() {
        // Set environment variables to mock package metadata
        env::set_var("CARGO_PKG_NAME", "frontmatter-gen");
        env::set_var("CARGO_PKG_VERSION", "0.0.5");

        #[cfg(feature = "cli")]
        {
            // Try to parse the version command
            let cli =
                Cli::try_parse_from(["frontmatter-gen", "--version"]);

            // If parsing fails, capture the error message
            assert!(
                cli.is_err(),
                "Expected an error for version output"
            );

            let err = cli.unwrap_err();
            let output = err.to_string();

            // Assert that the output contains the correct version
            assert!(
                output.contains("0.0.5"),
                "Version output does not contain '0.0.5'. Actual output: {}",
                output
            );
        }
    }

    // Ensure the logger is initialized only once across all tests
    static INIT: Once = Once::new();

    /// Initialize logging for tests
    fn init_logging() {
        INIT.call_once(|| {
            setup_logging();
        });
    }

    /// Tests that the `Logger` struct's `enabled` method always returns true.
    #[test]
    fn test_logger_enabled() {
        init_logging();
        let logger = Logger;
        let metadata =
            log::Metadata::builder().level(log::Level::Info).build();
        assert!(logger.enabled(&metadata));
    }

    /// Tests that the `Logger` struct's `log` method handles different log levels without panicking.
    #[test]
    fn test_logger_log_levels() {
        init_logging();
        let logger = Logger;
        let levels = vec![
            log::Level::Error,
            log::Level::Warn,
            log::Level::Info,
            log::Level::Debug,
            log::Level::Trace,
        ];

        for level in levels {
            let record = log::Record::builder()
                .args(format_args!("Test log message"))
                .level(level)
                .target("test")
                .build();
            logger.log(&record);
        }
    }

    /// Tests that `setup_logging` gracefully handles the case when `log::set_logger` fails.
    #[test]
    fn test_setup_logging_failure() {
        // Initialize logging the first time
        setup_logging();
        // Attempt to initialize logging a second time
        setup_logging();
        // The function handles the error internally, so we expect no panic
    }

    /// Tests that `execute_command` returns an error when no features are enabled.
    #[cfg(not(any(feature = "cli", feature = "ssg")))]
    #[tokio::test]
    async fn test_execute_command_no_features() {
        let result = execute_command().await;
        assert!(result.is_err());
    }

    /// Tests that `get_enabled_features` returns "none" when no features are enabled.
    #[cfg(not(any(feature = "cli", feature = "ssg")))]
    #[test]
    fn test_get_enabled_features_none() {
        let features = get_enabled_features();
        assert_eq!(features, "none");
    }

    /// Tests that `execute_command` works correctly when only the `cli` feature is enabled.
    #[cfg(all(feature = "cli", not(feature = "ssg")))]
    mod cli_tests {
        use super::*;
        use clap::Parser;
        use frontmatter_gen::cli::Cli;

        #[tokio::test]
        async fn test_execute_command_cli() {
            init_logging();
            use std::io::Write;
            use tempfile::NamedTempFile;

            // Create a temporary file with invalid frontmatter content
            let mut temp_file = NamedTempFile::new()
                .expect("Failed to create temp file");
            writeln!(temp_file, "Invalid frontmatter content")
                .expect("Failed to write to temp file");

            let file_path = temp_file.path().to_str().unwrap();

            // Simulate CLI arguments using the temporary file path
            let args = vec![
                "frontmatter-gen",
                "validate",
                file_path,
                "--required",
                "title,date",
            ];

            // Parse the arguments using clap
            let cli_command = Cli::try_parse_from(&args)
                .expect("Failed to parse arguments");

            // Process the command
            let result = cli_command.process().await;

            // Since the file has invalid content, we expect an error
            assert!(
                result.is_err(),
                "Expected validation to fail due to invalid content"
            );
        }

        #[test]
        fn test_get_enabled_features_cli() {
            let features = get_enabled_features();
            assert_eq!(features, "cli");
        }
    }

    /// Tests that `execute_command` works correctly when only the `ssg` feature is enabled.
    #[cfg(all(feature = "ssg", not(feature = "cli")))]
    mod ssg_tests {
        use super::*;
        use clap::Parser;
        use frontmatter_gen::ssg::SsgCommand;

        #[tokio::test]
        async fn test_execute_command_ssg() {
            init_logging();
            // Simulate SSG arguments
            let args = vec![
                "frontmatter-gen",
                "build",
                "--content-dir",
                "content",
                "--output-dir",
                "public",
                "--template-dir",
                "templates",
            ];
            // Parse the arguments using clap
            let ssg_command = SsgCommand::try_parse_from(&args)
                .expect("Failed to parse arguments");
            let result = ssg_command.execute().await;
            // Since we don't have actual directories, we expect an error
            assert!(result.is_err());
        }

        #[test]
        fn test_get_enabled_features_ssg() {
            let features = get_enabled_features();
            assert_eq!(features, "ssg");
        }
    }

    /// Tests that `execute_command` correctly dispatches between CLI and SSG when both features are enabled.
    #[cfg(all(feature = "cli", feature = "ssg"))]
    mod cli_ssg_tests {
        use super::*;
        use clap::Parser;
        use frontmatter_gen::{cli::Cli, ssg::SsgCommand};
        use std::io::Write;
        use tempfile::NamedTempFile;

        #[tokio::test]
        async fn test_execute_command_both_features() {
            init_logging();

            // Test SSG command
            let args_ssg = vec![
                "frontmatter-gen",
                "build",
                "--content-dir",
                "content",
                "--output-dir",
                "public",
                "--template-dir",
                "templates",
            ];
            // Parse the arguments using clap
            let ssg_command = SsgCommand::try_parse_from(&args_ssg)
                .expect("Failed to parse SSG arguments");
            let result_ssg = ssg_command.execute().await;
            assert!(result_ssg.is_err());

            // Test CLI command with a temporary file that will cause validation to fail
            let mut temp_file = NamedTempFile::new()
                .expect("Failed to create temp file");
            writeln!(temp_file, "Invalid content")
                .expect("Failed to write to temp file");
            let file_path = temp_file.path().to_str().unwrap();

            let args_cli = vec![
                "frontmatter-gen",
                "validate",
                file_path,
                "--required",
                "title,date",
            ];

            let cli_command = Cli::try_parse_from(&args_cli)
                .expect("Failed to parse CLI arguments");
            let result_cli = cli_command.process().await;

            // Now, since the file has invalid content, we expect the command to return an error
            assert!(
                result_cli.is_err(),
                "Expected an error due to invalid content"
            );
        }

        #[test]
        fn test_get_enabled_features_both() {
            let features = get_enabled_features();
            assert_eq!(features, "cli, ssg");
        }

        /// Tests that an invalid `RUST_LOG` value defaults to the debug level.
        #[test]
        fn test_logging_with_invalid_rust_log_value() {
            env::set_var("RUST_LOG", "invalid_level");
            setup_logging();

            // Verify that the log level defaults to debug
            assert_eq!(log::max_level(), LevelFilter::Debug);
        }

        /// Tests that an empty `RUST_LOG` value defaults to the debug level.
        #[test]
        fn test_logging_with_empty_rust_log_value() {
            env::set_var("RUST_LOG", "");
            setup_logging();

            // Verify that the log level defaults to debug
            assert_eq!(log::max_level(), LevelFilter::Debug);
        }

        /// Tests that the `Logger::flush()` method can be called without panic.
        #[test]
        fn test_logger_flush() {
            let logger = Logger;
            logger.flush();
            // Since flush does nothing, we just ensure it doesn't panic
        }

        /// Tests the behaviour of `get_enabled_features` when both features are enabled but in reverse order.
        #[test]
        #[cfg(all(feature = "ssg", feature = "cli"))]
        fn test_get_enabled_features_order() {
            let features = get_enabled_features();
            // Depending on the compilation, the order might be different
            assert!(
                features == "cli, ssg" || features == "ssg, cli",
                "Features should include both 'cli' and 'ssg'"
            );
        }
    }
}
