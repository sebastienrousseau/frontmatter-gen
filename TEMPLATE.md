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
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.5-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust

## Changelog 📚
