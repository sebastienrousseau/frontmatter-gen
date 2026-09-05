<p align="center">
  <img src="https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg" alt="Frontmatter Gen logo" width="128" />
</p>

<h1 align="center">Frontmatter Gen</h1>

<p align="center">
  <strong>A Rust library for generating and parsing frontmatter in YAML, TOML, and JSON formats.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/frontmatter-gen/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/frontmatter-gen/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/frontmatter-gen"><img src="https://img.shields.io/crates/v/frontmatter-gen.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/frontmatter-gen"><img src="https://img.shields.io/badge/docs.rs-frontmatter-gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/frontmatter-gen"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/frontmatter-gen?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/frontmatter-gen"><img src="https://img.shields.io/badge/lib.rs-v0.0.6-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Install

```bash
cargo add frontmatter-gen
```

Or add to `Cargo.toml`:

```toml
[dependencies]
frontmatter-gen = "0.0.10"
```

You need [Rust](https://rustup.rs/) 1.56.0 or later. Works on macOS, Linux, and Windows.

---

## Overview

Frontmatter Gen extracts and generates frontmatter blocks from content files. It supports YAML, TOML, and JSON delimiters out of the box.

- **Auto-detection** of frontmatter format from delimiters
- **Extraction** of frontmatter and body content in a single pass
- **Generation** of frontmatter blocks from Rust structs via Serde
- **Validation** of required fields and structure

---

## Features

| | |
| :--- | :--- |
| **Multi-format** | Parse and generate YAML, TOML, and JSON frontmatter |
| **Extraction** | Extract frontmatter from Markdown and other content files |
| **Validation** | Validate frontmatter structure and required fields |
| **Fenced blocks** | Support for fenced (`---`, `+++`, `{`) delimiters |
| **Serde integration** | Serialize/deserialize to Rust structs via Serde |

---

## Usage

```rust
use frontmatter_gen::extract;

fn main() {
    let content = r#"---
title: "My Post"
date: "2024-01-15"
---
# Hello World
This is the body."#;

    let (frontmatter, body) = extract(content).unwrap();
    println!("Title: {:?}", frontmatter.get("title"));
    println!("Body: {}", body);
}
```

---

## Development

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo clippy       # Lint with Clippy
cargo fmt          # Format with rustfmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

**THE ARCHITECT** \u1d2b [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** \u1d5e [EUXIS](https://euxis.co) \u1d2b Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#frontmatter-gen">Back to Top</a></p>