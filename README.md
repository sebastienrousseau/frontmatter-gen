# Frontmatter Gen (frontmatter-gen)

<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg"
alt="FrontMatter Gen logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

A high-performance Rust library for parsing and serialising frontmatter in YAML, TOML, and JSON formats. Built for safety, efficiency, and ease of use.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][02]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview 🚀

`frontmatter-gen` is a comprehensive Rust library that provides robust handling of frontmatter in content files. It delivers a type-safe, efficient solution for extracting, parsing, and serialising frontmatter in multiple formats. Whether you're building a static site generator, content management system, or any application requiring structured metadata, `frontmatter-gen` offers the tools you need.

## Key Features 🎯

- **Zero-Copy Parsing**: Parse YAML, TOML, and JSON frontmatter efficiently with zero memory copying
- **Safe Extraction**: Extract frontmatter using standard delimiters (`---` for YAML, `+++` for TOML) with comprehensive error handling
- **Type Safety**: Leverage Rust's type system with the `Value` enum for safe frontmatter manipulation
- **High Performance**: Optimised for speed with minimal allocations and efficient algorithms
- **Memory Safety**: Guaranteed memory safety through Rust's ownership system
- **Rich Error Handling**: Detailed error types with context for effective debugging
- **Async Support**: First-class asynchronous operation support
- **Flexible Configuration**: Customisable parsing behaviour to match your needs

## Available Features 🛠️

This crate provides several feature flags to customise its functionality:

- **default**: Core frontmatter parsing functionality only
- **cli**: Command-line interface tools for quick operations
- **ssg**: Static Site Generator functionality (includes CLI features)

Configure features in your `Cargo.toml`:

```toml
[dependencies]

# Enable CLI support for validation and extraction
frontmatter-gen = { version = "0.0.4", features = ["cli"] }

# Enable all features (validation, extraction and static site generation)
frontmatter-gen = { version = "0.0.4", features = ["ssg"] }
```

Installation via cargo:

```bash
# Install with CLI support
cargo install frontmatter-gen --features="cli"

# Install with SSG support
cargo install frontmatter-gen --features="ssg"
```

## Getting Started 📦

### Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]

# Basic functionality
frontmatter-gen = "0.0.4"

# With CLI support
frontmatter-gen = { version = "0.0.4", features = ["cli"] }

# All features (CLI and SSG)
frontmatter-gen = { version = "0.0.4", features = ["ssg"] }
```

## Basic Usage 🔨

### Extract and Parse Frontmatter

```rust
use frontmatter_gen::extract;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = r#"---
title: My Document
date: 2025-09-09
tags:
  - documentation
  - rust
---
# Content begins here"#;

    let (frontmatter, content) = extract(content)?;

    if let Some(title) = frontmatter.get("title").and_then(|v| v.as_str()) {
        println!("Title: {}", title);
    }

    println!("Content: {}", content);
    Ok(())
}
```

### Format Conversion

```rust
use frontmatter_gen::{Frontmatter, Format, Value, to_format};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut frontmatter = Frontmatter::new();
    frontmatter.insert("title".to_string(), Value::String("My Document".to_string()));

    let json = to_format(&frontmatter, Format::Json)?;
    println!("JSON output: {}", json);
    assert!(json.contains(r#""title":"My Document""#));

    Ok(())
}
```

## CLI Tool 🛠️

The `fmg` command provides comprehensive frontmatter operations:

```bash
# Extract frontmatter in various formats
fmg extract input.md --format yaml
fmg extract input.md --format toml
fmg extract input.md --format json

# Save extracted frontmatter
fmg extract input.md --format yaml --output frontmatter.yaml

# Validate frontmatter
fmg validate input.md --required title,date,author
```

You can also use the CLI directly from the source code:

```bash
# Extract frontmatter in various formats
cargo run --features="cli" extract input.md --format yaml
cargo run --features="cli" extract input.md --format toml
cargo run --features="cli" extract input.md --format json

# Save extracted frontmatter
cargo run --features="cli" extract input.md --format yaml --output frontmatter.yaml
```

## Static Site Generation 🌐

Build and serve your static site:

```bash
# Generate a static site with the fmg CLI
fmg build \
    --content-dir content \
    --output-dir public \
    --template-dir templates

or from the source code:

# Generate a static site using cargo
cargo run --features="ssg" -- build \
    --content-dir content \
    --output-dir public \
    --template-dir templates
```

### Serve locally (using Python for demonstration)

```bash
# Change to the output directory
cd public

# Serve the site
python -m http.server 8000 --bind 127.0.0.1
```

Then visit `http://127.0.0.1:8000` in your favourite browser.

## Error Handling 🚨

The library provides comprehensive error handling:

```rust
use frontmatter_gen::{extract, error::Error};

fn process_content(content: &str) -> Result<(), Error> {
    let (frontmatter, _) = extract(content)?;
    
    // Validate required fields
    for field in ["title", "date", "author"].iter() {
        if !frontmatter.contains_key(*field) {
            return Err(Error::ValidationError(
                format!("Missing required field: {}", field)
            ));
        }
    }
    
    Ok(())
}
```

## Logging Support 📝

When the `logging` feature is enabled, the library integrates with Rust's `log` crate for detailed debug output. You can use any compatible logger implementation (e.g., `env_logger`, `simple_logger`).

### Basic Logging Setup

```rust
use frontmatter_gen::extract;
use log::{debug, info, Level, Metadata, Record, set_logger, set_max_level};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} [{}] - {}",
                record.target(),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

fn init_logger() {
    // Explicitly handle logger initialization error
    if let Err(e) = set_logger(&LOGGER).map(|()| set_max_level(Level::Debug.to_level_filter())) {
        eprintln!("Failed to initialize logger: {}", e);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the custom logger
    init_logger();
    info!("Starting frontmatter extraction");

    let content = r#"---
title: My Document
date: 2025-09-09
---
# Content"#;

    // Extract frontmatter and remaining content
    let (frontmatter, content) = extract(content)?;
    debug!("Extracted frontmatter: {:?}", frontmatter);
    debug!("Remaining content: {:?}", content);

    Ok(())
}
```

#### Advanced Logging Configuration

For more control over logging:

```rust
use frontmatter_gen::{parser, Format};
use log::{debug, info, Level, Metadata, Record, set_logger, set_max_level};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

fn init_logger() {
    set_logger(&LOGGER).expect("Failed to set logger");
    set_max_level(Level::Debug.to_level_filter());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the custom logger
    init_logger();
    info!("Starting frontmatter processing");

    let yaml = r#"title: Test Document"#;
    let frontmatter = parser::parse(yaml, Format::Yaml)?;
    debug!("Parsed frontmatter: {:?}", frontmatter);

    Ok(())
}
```

## CLI Logging 📝

When using the CLI with logging enabled:

```bash
# Set log level via environment variable
RUST_LOG=debug frontmatter_gen extract input.md --format yaml

# Or for more specific control
RUST_LOG=frontmatter_gen=debug,cli=info frontmatter_gen validate input.md
```

The library provides detailed error handling with context:

```rust
use frontmatter_gen::{extract, error::Error};

fn process_content(content: &str) -> Result<(), Error> {
    // Extract frontmatter and content
    let (frontmatter, _) = extract(content)?;
    
    // Validate required fields
    for field in ["title", "date", "author"].iter() {
        if !frontmatter.contains_key(*field) {
            return Err(Error::ValidationError(
                format!("Missing required field: {}", field)
            ));
        }
    }
    
    // Validate field types
    if let Some(date) = frontmatter.get("date") {
        if !date.is_string() {
            return Err(Error::ValidationError(
                "Date field must be a string".to_string()
            ));
        }
    }
    
    Ok(())
}
```

## Documentation 📚

For comprehensive API documentation and examples, visit:

- [API Documentation on docs.rs][04]
- [User Guide and Tutorials][00]
- [Example Code Repository][02]

## Contributing 🤝

We welcome contributions! Please see our [Contributing Guidelines][05] for details on:

- Code of Conduct
- Development Process
- Submitting Pull Requests
- Reporting Issues

## Licence 📝

This project is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT licence ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Acknowledgements 🙏

Special thanks to all contributors and the Rust community for their invaluable support and feedback.

[00]: https://frontmatter-gen.com
[01]: https://lib.rs/crates/frontmatter-gen
[02]: https://github.com/sebastienrousseau/frontmatter-gen/issues
[03]: https://crates.io/crates/frontmatter-gen
[04]: https://docs.rs/frontmatter-gen
[05]: https://github.com/sebastienrousseau/frontmatter-gen/blob/main/CONTRIBUTING.md
[06]: https://codecov.io/gh/sebastienrousseau/frontmatter-gen
[07]: https://github.com/sebastienrousseau/frontmatter-gen/actions?query=branch%3Amain
[08]: https://www.rust-lang.org/

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/frontmatter-gen/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/frontmatter-gen?style=for-the-badge&token=Q9KJ6XXL67&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/frontmatter-gen.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-frontmatter--gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/frontmatter--gen-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.4-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
