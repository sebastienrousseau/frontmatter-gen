# Frontmatter Gen (frontmatter-gen)

<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg"
alt="FrontMatter Gen logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

A high-performance Rust library for parsing and serialising frontmatter in YAML, TOML, and JSON formats. Engineered for safety, efficiency, and ease of use.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][02]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview 🚀

`frontmatter-gen` is a robust Rust library that provides comprehensive handling of frontmatter in content files. It delivers a type-safe, efficient solution for extracting, parsing, and serialising frontmatter in multiple formats. Whether you're building a static site generator, content management system, or any application requiring structured metadata, `frontmatter-gen` offers the tools you need with performance and safety at its core.

## Key Features 🎯

- **Zero-copy parsing**: Optimised for memory efficiency
- **Multi-format support**: Parse and serialise YAML, TOML, and JSON
- **Type-safe operations**: Comprehensive error handling with `Result` types
- **Secure processing**: Input validation and size limits
- **Async support**: Full asynchronous operations via the `ssg` feature flag
- **Command-line interface**: Direct frontmatter manipulation tools
- **Memory safety**: Guaranteed memory safety through Rust's ownership system
- **Comprehensive testing**: Extensive test coverage and validation
- **Rich documentation**: Detailed guides and examples

## Available Features 🛠️

This crate provides several feature flags to customise its functionality:

- **default**: Core frontmatter parsing functionality
- **cli**: Command-line interface tools for validation and extraction
- **ssg**: Static Site Generator functionality (includes CLI features)

Configure features in your `Cargo.toml`:

```toml
[dependencies]
# Enable CLI support for validation and extraction
frontmatter-gen = { version = "0.0.5", features = ["cli"] }

# Enable all features (validation, extraction and static site generation)
frontmatter-gen = { version = "0.0.5", features = ["ssg"] }
```

Installation via Cargo:

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
# Core library with command-line interface and SSG support
frontmatter-gen = { version = "0.0.5", features = ["cli", "ssg"] }
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

    // Access frontmatter fields safely
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
    frontmatter.insert(
        "title".to_string(), 
        Value::String("My Document".to_string())
    );

    // Convert to different formats
    let yaml = to_format(&frontmatter, Format::Yaml)?;
    let json = to_format(&frontmatter, Format::Json)?;
    let toml = to_format(&frontmatter, Format::Toml)?;

    println!("YAML output:\n{}", yaml);
    println!("JSON output:\n{}", json);
    println!("TOML output:\n{}", toml);

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

# Validate frontmatter
cargo run --features="cli" validate input.md --required title,date,author
```

## Static Site Generation 🌐

Build and serve your static site:

```bash
# Generate a static site
fmg build \
    --content-dir content \
    --output-dir public \
    --template-dir templates

# Or using cargo
cargo run --features="ssg" -- build \
    --content-dir content \
    --output-dir public \
    --template-dir templates
```

### Local Development Server

```bash
# Change to the output directory
cd public

# Start a local server (using Python for demonstration)
python -m http.server 8000 --bind 127.0.0.1
```

Visit `http://127.0.0.1:8000` in your browser to view the site.

## Error Handling 🚨

The library provides comprehensive error handling with context:

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

## Logging Support 📝

Enable detailed logging with the standard Rust logging facade:

```rust
use frontmatter_gen::extract;
use log::{debug, info, Level};
use simple_logger::SimpleLogger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise logging
    SimpleLogger::new()
        .with_level(Level::Debug.to_level_filter())
        .init()?;

    let content = r#"---
title: My Document
date: 2025-09-09
---
# Content"#;

    info!("Processing frontmatter");
    let (frontmatter, content) = extract(content)?;
    debug!("Extracted frontmatter: {:?}", frontmatter);
    debug!("Content: {:?}", content);

    Ok(())
}
```

### CLI Logging Configuration

Control logging levels via environment variables:

```bash
# Set log level for CLI operations
RUST_LOG=debug fmg extract input.md --format yaml

# Configure specific component logging
RUST_LOG=frontmatter_gen=debug,cli=info fmg validate input.md
```

## Documentation 📚

Comprehensive documentation is available at:

- [API Documentation][04]
- [User Guide and Examples][00]
- [Source Code and Examples][09]

## Contributing 🤝

We welcome contributions! Please see our [Contributing Guidelines][05] for:

- Code of Conduct
- Development Process
- Pull Request Guidelines
- Issue Reporting

## Licence 📝

This project is dual-licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE) ([http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- [MIT Licence](LICENSE-MIT) ([http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

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
[09]: https://github.com/sebastienrousseau/frontmatter-gen/

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/frontmatter-gen/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/frontmatter-gen?style=for-the-badge&token=Q9KJ6XXL67&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/frontmatter-gen.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-frontmatter--gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/frontmatter--gen-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.5-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
