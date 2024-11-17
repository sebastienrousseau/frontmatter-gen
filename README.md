<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg"
alt="FrontMatter Gen logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

# Frontmatter Gen (frontmatter-gen)

A high-performance Rust library for parsing and serialising frontmatter in YAML, TOML, and JSON formats. Built for safety, efficiency, and ease of use.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][02]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview

`frontmatter-gen` is a Rust library that provides robust handling of frontmatter in content files. It offers a type-safe, efficient solution for extracting, parsing and serialising frontmatter in multiple formats. Whether you're building a static site generator, content management system, or any application requiring structured metadata, `frontmatter-gen` delivers the tools you need.

### Key Features

- **Zero-Copy Parsing**: Parse YAML, TOML and JSON frontmatter efficiently with zero memory copying
- **Safe Extraction**: Extract frontmatter using standard delimiters (`---` for YAML, `+++` for TOML) with comprehensive error handling
- **Type Safety**: Leverage Rust's type system with the `Value` enum for safe frontmatter manipulation
- **High Performance**: Optimised for speed with minimal allocations and efficient algorithms
- **Memory Safety**: Guaranteed memory safety through Rust's ownership system
- **Rich Error Handling**: Detailed error types with context for effective debugging
- **Async Support**: First-class asynchronous operation support
- **Flexible Configuration**: Customisable parsing behaviour to match your needs

### Available Features

This crate provides several feature flags to customise its functionality:

- **default**: Core frontmatter parsing functionality only
- **cli**: Command-line interface tools for quick operations
- **ssg**: Static Site Generator functionality (includes CLI features)
- **logging**: Debug logging capabilities

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
# Basic frontmatter parsing only
frontmatter-gen = "0.0.3"

# With Static Site Generator functionality
frontmatter-gen = { version = "0.0.3", features = ["ssg"] }
```

### Basic Usage

#### Extract and parse frontmatter from content

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
println!("Title: {}", frontmatter.get("title").unwrap().as_str().unwrap());
println!("Content: {}", content);
Ok(())
}
```

#### Convert between formats

```rust
use frontmatter_gen::{Frontmatter, Format, Value, to_format};

fn main() -> Result<(), Box<dyn std::error::Error>> {

let mut frontmatter = Frontmatter::new();
frontmatter.insert("title".to_string(), "My Document".into());
frontmatter.insert("draft".to_string(), false.into());

// Convert to YAML
let yaml = to_format(&frontmatter, Format::Yaml)?;

// Convert to TOML
let toml = to_format(&frontmatter, Format::Toml)?;

// Convert to JSON
let json = to_format(&frontmatter, Format::Json)?;

Ok(())
}
```

### Advanced Features

#### Handle complex nested structures

```rust
use frontmatter_gen::{parser, Format, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
let yaml = r#"
title: My Document
metadata:
  author:
    name: Jane Smith
    email: jane@example.com
  categories:
    - technology
    - rust
settings:
  template: article
  published: true
"#;

let frontmatter = parser::parse(yaml, Format::Yaml)?;
Ok(())
}
```

## Documentation

For comprehensive API documentation and examples, visit:

- [API Documentation on docs.rs][04]
- [User Guide and Tutorials][00]
- [Example Code Repository][02]

## CLI Tool

The library includes a command-line interface for quick frontmatter operations:

```bash
# Extract frontmatter from 'input.md' and output it in YAML format
frontmatter-gen extract input.md --format yaml

# Extract frontmatter from 'input.md' and output it in TOML format
frontmatter-gen extract input.md --format toml

# Extract frontmatter from 'input.md' and output it in JSON format
frontmatter-gen extract input.md --format json

# Validate frontmatter from 'input.md' and check for custom required fields
frontmatter-gen validate input.md --required title,date,author
```

## Error Handling

The library provides detailed error handling:

```rust
use frontmatter_gen::extract;
use frontmatter_gen::error::FrontmatterError;

fn process_content(content: &str) -> Result<(), FrontmatterError> {
    let (frontmatter, _) = extract(content)?;
    
    // Validate required fields
    if !frontmatter.contains_key("title") {
        return Err(FrontmatterError::ValidationError(
            "Missing required field: title".to_string()
        ));
    }
    
    Ok(())
}
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines][05] for details on:

- Code of Conduct
- Development Process
- Submitting Pull Requests
- Reporting Issues

## Licence

This project is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT licence ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Acknowledgements

Special thanks to all contributors and the Rust community for their support and feedback.

[00]: https://frontmatter-gen.com
[01]: https://lib.rs/crates/frontmatter-gen
[02]: https://github.com/sebastienrousseau/frontmatter-gen/issues
[03]: https://crates.io/crates/frontmatter-gen
[04]: https://docs.rs/frontmatter-gen
[05]: https://github.com/sebastienrousseau/frontmatter-gen/blob/main/CONTRIBUTING.md
[06]: https://codecov.io/gh/sebastienrousseau/frontmatter-gen
[07]: https://github.com/sebastienrousseau/frontmatter-gen/actions?query=branch%3Amain
[08]: https://www.rust-lang.org/

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/frontmatter--gen/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/frontmatter-gen?style=for-the-badge&token=Q9KJ6XXL67&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/frontmatter-gen.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-frontmatter--gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/frontmatter--gen-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.3-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
