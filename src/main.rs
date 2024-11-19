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
    log::debug!(
        "Initializing with features: {}",
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
    let level = match env.to_lowercase().as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Debug,
    };

    // Set up the logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level))
        .unwrap_or_else(|e| {
            eprintln!("Warning: Failed to initialize logger: {}", e);
        });

    log::debug!("Logging initialized at level: {}", level);
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
        log::debug!("Executing in SSG mode");
        let ssg_command = SsgCommand::parse();
        return ssg_command.execute().await;
    }

    #[cfg(all(feature = "cli", not(feature = "ssg")))]
    {
        log::debug!("Executing in CLI mode");
        let cli_command = Cli::parse();
        return cli_command.process().await;
    }

    #[cfg(all(feature = "cli", feature = "ssg"))]
    {
        // Handle both CLI and SSG features
        log::debug!("Executing with both CLI and SSG features enabled");

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
        env::set_var("CARGO_PKG_VERSION", "0.0.4");

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
                output.contains("0.0.4"),
                "Version output does not contain '0.0.4'. Actual output: {}",
                output
            );
        }
    }
}
