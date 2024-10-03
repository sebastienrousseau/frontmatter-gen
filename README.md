<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg"
alt="FrontMatter Gen logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

# Frontmatter Gen (frontmatter-gen)

A robust Rust library for parsing and serializing frontmatter in various formats, including YAML, TOML, and JSON.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][02]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview

`frontmatter-gen` is a flexible Rust library that provides functionality for extracting, parsing, and serializing frontmatter in various formats. It's commonly used in static site generators and content management systems to handle metadata at the beginning of content files.

### Key Features

- **Multiple Format Support**: Parse and serialize frontmatter in YAML, TOML, and JSON formats.
- **Flexible Extraction**: Extract frontmatter from content, supporting different delimiters.
- **Robust Error Handling**: Comprehensive error types for detailed problem reporting.
- **Customizable Parsing**: Configure parsing options to suit your needs.
- **Efficient Conversions**: Convert between different frontmatter formats seamlessly.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
frontmatter-gen = "0.1.0"
```

## Usage

Here are some examples of how to use the library:

### Basic Usage

```rust
use frontmatter_gen::{extract, Frontmatter, Format};

let content = r#"---
title: My Post
date: 2023-05-20
---
Content here"#;

let (frontmatter, remaining_content) = extract(content).unwrap();
assert_eq!(frontmatter.get("title").unwrap().as_str().unwrap(), "My Post");
assert_eq!(remaining_content, "Content here");
```

### Converting Formats

```rust
use frontmatter_gen::{Frontmatter, Format, to_format};

let mut frontmatter = Frontmatter::new();
frontmatter.insert("title".to_string(), "My Post".into());
frontmatter.insert("date".to_string(), "2023-05-20".into());

let yaml = to_format(&frontmatter, Format::Yaml).unwrap();
assert!(yaml.contains("title: My Post"));
assert!(yaml.contains("date: '2023-05-20'"));
```

## Documentation

For full API documentation, please visit [docs.rs/frontmatter-gen](https://docs.rs/frontmatter-gen).

## Examples

To run the examples, clone the repository and use the following command:

```shell
cargo run --example example_name
```

Replace `example_name` with the name of the example you want to run.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Acknowledgements

Special thanks to all contributors who have helped build the `frontmatter-gen` library.

[00]: https://frontmatter-gen.com
[01]: https://lib.rs/crates/frontmatter-gen
[02]: https://github.com/sebastienrousseau/frontmatter-gen/issues
[03]: https://crates.io/crates/frontmatter-gen
[04]: https://docs.rs/frontmatter-gen
[05]: https://github.com/sebastienrousseau/frontmatter-gen/blob/main/CONTRIBUTING.md "Contributing Guidelines"
[06]: https://codecov.io/gh/sebastienrousseau/frontmatter-gen
[07]: https://github.com/sebastienrousseau/frontmatter-gen/actions?query=branch%3Amain
[08]: https://www.rust-lang.org/

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/frontmatter--gen/release.yml?branch=main&style=for-the-badge&logo=github "Build Status"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/frontmatter-gen?style=for-the-badge&token=Q9KJ6XXL67&logo=codecov "Codecov"
[crates-badge]: https://img.shields.io/crates/v/frontmatter-gen.svg?style=for-the-badge&color=fc8d62&logo=rust "Crates.io"
[docs-badge]: https://img.shields.io/badge/docs.rs-frontmatter--gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs "Docs.rs"
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/frontmatter--gen-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge "View on lib.rs"
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
