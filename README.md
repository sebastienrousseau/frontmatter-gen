<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://kura.pro/frontmatter-gen/images/logos/frontmatter-gen.svg" alt="frontmatter-gen logo" width="128" />
</p>

<h1 align="center">frontmatter-gen</h1>

<p align="center">
  Front matter in, typed values out. YAML, TOML and JSON, with zero
  <code>unsafe</code> code.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/frontmatter-gen/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/frontmatter-gen/ci.yml?branch=main&style=for-the-badge&label=build" alt="Build status" /></a>
  <a href="https://crates.io/crates/frontmatter-gen"><img src="https://img.shields.io/crates/v/frontmatter-gen.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="crates.io version" /></a>
  <a href="https://docs.rs/frontmatter-gen"><img src="https://img.shields.io/badge/docs.rs-frontmatter--gen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="API docs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/frontmatter-gen"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/frontmatter-gen?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/frontmatter-gen"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/frontmatter-gen?style=for-the-badge&label=scorecard" alt="OpenSSF Scorecard" /></a>
  <a href="https://www.bestpractices.dev/projects/14538"><img src="https://img.shields.io/cii/level/14538?style=for-the-badge&label=OpenSSF%20Best%20Practices&logo=openssf" alt="OpenSSF Best Practices" /></a>
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License" /></a>
  <a href="#minimum-toolchain-policy"><img src="https://img.shields.io/badge/MSRV-1.88.0-orange.svg?style=for-the-badge" alt="MSRV 1.88.0" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — Cargo, source, optional features
- [Requirements](#requirements) — toolchain floor, platforms
- [Quick Start](#quick-start) — front matter and body in five lines

**Library reference**

- [What it does](#what-it-does) — the three formats and how they are found
- [Two halves, one pass](#two-halves-one-pass) — extraction and parsing
- [Library Usage](#library-usage) — extract, parse, serialise, validate
- [Configuration](#configuration) — `ParseOptions` and the limits
- [The CLI](#the-cli) — `fmg`, behind the `cli` feature
- [Static site generation](#static-site-generation) — the `ssg` feature
- [Examples](#examples) — runnable example index

**Operational**

- [When not to use frontmatter-gen](#when-not-to-use-frontmatter-gen) — limitations
- [Development](#development) — make targets, fuzzing, Miri, CI
- [Security](#security) — limits, path safety, supply chain
- [Documentation](#documentation) — all reference docs
- [Stability guarantees](#stability-guarantees) — SemVer axis, output stability
- [Minimum-toolchain policy](#minimum-toolchain-policy)
- [License](#license)

---

## Install

```toml
[dependencies]
frontmatter-gen = "0.0.11"
```

Or from the command line:

```bash
cargo add frontmatter-gen
```

### Cargo features

All optional. The default build is the library alone, and its dependency
set is deliberately small.

| Feature | Pulls in | Adds |
| :--- | :--- | :--- |
| *(default, none)* | — | extraction, the three parsers, `Value` and `Frontmatter` |
| `cli` | `clap` | the `fmg` binary and its argument parsing |
| `ssg` | `cli` + `tera`, `pulldown-cmark`, `dtt`, `url` | the static-site surface: `Engine`, `Config`, filesystem helpers |
| `logging` | — | log output from the library paths |

```bash
cargo install frontmatter-gen --features cli   # the fmg binary
```

### Build from source

```bash
git clone https://github.com/sebastienrousseau/frontmatter-gen.git
cd frontmatter-gen
make          # check + clippy + test
```

---

## Requirements

- **Rust 1.88.0 or newer.** `rust-version` in `Cargo.toml` is the floor
  and Cargo enforces it; CI builds on stable across Linux, macOS and
  Windows. See the [minimum-toolchain policy](#minimum-toolchain-policy).
- **A `std` platform.** The crate uses `std` unconditionally; there is no
  `no_std` build.
- **No async runtime is required** for the library. The `ssg` feature's
  engine is async and uses Tokio.

---

## Quick Start

```rust
use frontmatter_gen::extract;

let content = "\
---
title: My Post
date: 2024-01-15
tags: [rust, parsing]
---
Hello World.

This is the body.";

let (frontmatter, body) = extract(content).expect("valid front matter");

assert_eq!(
    frontmatter.get("title").and_then(|v| v.as_str()),
    Some("My Post")
);
assert_eq!(
    frontmatter.get("tags").and_then(|v| v.array_len()),
    Some(2)
);
assert!(body.starts_with("Hello World."));
```

---

## What it does

`frontmatter-gen` finds the structured block at the top of a content
file, tells you what format it is, and turns it into values you can
index or deserialise.

Three shapes are recognised:

| Format | Delimiters | Opening line |
|---|---|---|
| YAML | `---` … `---` | `title: Hello` |
| TOML | `+++` … `+++` | `title = "Hello"` |
| JSON | a `{ … }` object at the top of the file | `{"title": "Hello"}` |

Detection is automatic and the body is returned as a **slice of the
input**, never a rebuilt string.

---

## Two halves, one pass

Finding the block and understanding it are separate operations, and both
are public.

| | `extractor` | `parser` |
|---|---|---|
| Answers | where is the front matter, and what format | what does it contain |
| Returns | the raw block and the body, as borrows | a `Frontmatter` of `Value`s |
| Costs | a scan | a parse |

A caller that only needs the format, or only needs the body, does not
pay for a parse. [ADR-0002](docs/adr/0002-extraction-separate-from-parsing.md)
records why.

---

## Library Usage

### Extract front matter and body

```rust
use frontmatter_gen::extract;

let (frontmatter, body) =
    extract("+++\ntitle = \"T\"\n+++\nBody").expect("valid front matter");

assert_eq!(frontmatter.get("title").and_then(|v| v.as_str()), Some("T"));
assert_eq!(body.trim(), "Body");
```

### Detect the format without parsing

```rust
use frontmatter_gen::extractor::{detect_format, extract_raw_frontmatter};
use frontmatter_gen::Format;

let (raw, body) =
    extract_raw_frontmatter("---\ntitle: T\n---\nBody").expect("a fence");
assert_eq!(detect_format(raw).expect("a known format"), Format::Yaml);
assert_eq!(body.trim(), "Body");
```

### Parse and serialise a known format

```rust
use frontmatter_gen::{parser::{parse, to_string}, Format, Value};

let frontmatter =
    parse("title: T\ncount: 2\n", Format::Yaml).expect("valid YAML");
assert_eq!(frontmatter.get("count"), Some(&Value::Number(2.0)));

// Round-trips through any of the three formats.
let json = to_string(&frontmatter, Format::Json).expect("serialise");
let back = parse(&json, Format::Json).expect("re-parse");
assert_eq!(back.get("title"), frontmatter.get("title"));
```

### Work with values

```rust
use frontmatter_gen::{Frontmatter, Value};

let mut frontmatter = Frontmatter::new();
let _ = frontmatter.insert("title".to_string(), Value::from("Hello"));
let _ = frontmatter.insert("draft".to_string(), Value::from(false));

assert!(frontmatter.contains_key("title"));
assert_eq!(frontmatter.get("draft"), Some(&Value::Boolean(false)));
assert_eq!(frontmatter.len(), 2);
```

`Value` is null, string, number, boolean, array, object, or a `Tagged`
pair for YAML tags. Each variant has an `is_*` predicate, an `as_*`
borrow and an `into_*` conversion that reports what it found instead.

---

## Configuration

`parse` applies no limits. `parse_with_options` does, **when you ask it
to**:

```rust
use frontmatter_gen::{parser::{parse_with_options, ParseOptions}, Format};

let options = ParseOptions {
    max_depth: 8,
    max_keys: 256,
    validate: true,
};

let deep = r#"{"a": {"b": {"c": 1}}}"#;
assert!(parse_with_options(deep, Format::Json, Some(options)).is_ok());
```

| Option | Default | Effect when `validate` is true |
|---|---|---|
| `max_depth` | see `ParseOptions::default` | rejects nesting deeper than this |
| `max_keys` | see `ParseOptions::default` | rejects documents with more keys |
| `validate` | `false` | whether either limit is applied at all |

Validation is opt-in on purpose: the common caller reads files its own
author wrote, where a spurious rejection is worse than the problem.
Untrusted input should set `validate: true`. See
[ADR-0004](docs/adr/0004-opt-in-validation-limits.md) and
[Security](#security).

---

## The CLI

Behind the `cli` feature, `fmg` extracts and validates from the shell:

```bash
cargo install frontmatter-gen --features cli

fmg extract post.md --format yaml
fmg validate post.md --required title,date
```

---

## Static site generation

Behind the `ssg` feature, `Engine` walks a content directory, splits
front matter from body, renders through Tera and writes output;
`Config` validates the site's settings before anything runs. Every path
it touches passes through the checks described under
[Security](#security).

This surface is optional and off by default; the library does not pull
Tera, a Markdown parser or a date library unless you ask for it.

---

## Examples

Run any of these with `cargo run --example <name>`:

| Example | Shows |
|---|---|
| `lib` | the high-level `extract` flow |
| `extractor` | format detection and raw extraction |
| `parser` | parsing and serialising each format |
| `types` | `Value` and `Frontmatter` in use |
| `fenced` | the three delimiter styles |
| `frontmatter` | building front matter programmatically |
| `error` | every error variant and how to recover |

The target names come from `Cargo.toml`, which drops the `_examples`
suffix the files carry.

`make examples` runs all of them; CI does the same on every push, so an
example that stops working fails the build.

---

## When not to use frontmatter-gen

Cases where something else fits better, listed because the honest answer
is "not yet" or "by design".

- **You need `no_std`.** The crate uses `std` unconditionally.
- **You need typed deserialisation straight into your struct.**
  `Frontmatter` is a map of `Value`. Parse with `serde` directly against
  the raw block from `extract_raw_frontmatter` if you want a struct.
- **You need `Display` output to be valid JSON.** It is a rendering, not
  a serialiser: `escape_str` escapes `"` and `\` only, so a control
  character in a string produces text that will not re-parse. Use
  `to_string(&fm, Format::Json)`.
- **You need to preserve comments, key order or quoting.** Parsing is
  lossy in all three; there is no round-trip-faithful editor here.
- **You want a full static-site generator.** The `ssg` feature is a
  useful core, not a finished product.

If you hit a case that should be on this list, please open an issue —
that is how it gets fixed or moved into the supported set.

---

## Development

```bash
make              # check + clippy + test
make test         # all tests, all features
make clippy       # lints, warnings denied
make fmt          # formatting check
make lint         # markdownlint + codespell + REUSE
make doc          # rustdoc with warnings denied
make coverage     # line coverage gate (98%, excluding src/main.rs)
make miri         # lib tests under Miri
make fuzz         # build every target, replay corpus and regressions
make examples     # run every example
make bench-smoke  # compile and run each bench once
make versions     # every version-bearing file agrees
make deny / vet / audit   # supply chain
```

[`DEVELOPMENT.md`](DEVELOPMENT.md) maps each CI job to its local
equivalent and explains the gotchas.

### Fuzzing

Three `cargo-fuzz` targets live under `fuzz/fuzz_targets/`:

```bash
cargo +nightly fuzz run fuzz_extract     # the body must be a suffix of the input
cargo +nightly fuzz run fuzz_parse       # every parser against every input
cargo +nightly fuzz run fuzz_roundtrip   # parse, serialise, re-parse, same keys
```

`fuzz_roundtrip` is the one that earns its keep: a serialiser that drops
a key still produces output that parses, so nothing but a round trip
notices. `fuzz/corpus/<target>` holds the committed seeds and
`fuzz/regressions/<target>` every fixed-bug input; both replay on each
push.

cargo-fuzz must be **installed from source** (`cargo install --locked
cargo-fuzz`): the prebuilt binary is a musl build and infers its own
build triple as the fuzz target.

### CI

| Workflow | Trigger | Purpose |
| :--- | :--- | :--- |
| `ci.yml` | push, PR | clippy, fmt, tests across three OSes, coverage, audit |
| `quality.yml` | push, PR | coverage gate, Miri, fuzz replay, docs lint, cargo-vet ratchet, release hygiene |

See [CONTRIBUTING.md](CONTRIBUTING.md) for signed commits and PR
guidelines.

---

## Security

**Reporting:** never open a public issue for a vulnerability. See
[`SECURITY.md`](SECURITY.md) for the private channel and disclosure
policy.

### Input limits

`ParseOptions` carries `max_depth`, `max_keys` and `max_size`, and
`parse_with_options` enforces them **only when `validate` is true**. A
caller handling untrusted input must opt in. That is said plainly here
because a limit nobody enables is not a limit.

### Path safety

Everything the `ssg` feature touches goes through
`utils::fs::validate_path_safety`, which rejects backslashes, null bytes
and control characters, `..` traversal, symlinks, and Windows reserved
device names. It deliberately does not decide which directory a caller
may write to: only the caller knows its own root.

A defect fixed in v0.0.11 is worth naming: an absolute path used to
return early from that function, skipping the symlink and reserved-name
checks below it, so a symlink passed validation whenever it arrived as
an absolute path. Every rule now applies to every path.

### Architectural posture

- `#![forbid(unsafe_code)]` — the compiler proves the absence of unsafe
  blocks.
- No C dependencies, no FFI, no network I/O.
- Test-only crates live in `[dev-dependencies]`, so a consumer's build
  does not pull them.

### Supply chain

- `cargo-deny` and `cargo-audit` in CI.
- `cargo-vet` provenance in `supply-chain/`, with an exemption baseline
  the CI ratchet cannot exceed.
- `noyalib` and `dtt` pinned exactly (`=0.0.X`); a bump is a deliberate
  release ([ADR-0005](docs/adr/0005-first-party-exact-pins.md)).
- `Cargo.lock` committed; CI builds `--locked`. Actions pinned by SHA.
- REUSE 3.3 compliant, linted in CI.
- Commits on `main` are signed; releases are signed tags
  ([`KEYS.asc`](KEYS.asc)).

---

## Documentation

The four entry points, identical across every repo in the family:

- **[API reference](https://docs.rs/frontmatter-gen)** — rustdoc on docs.rs
- **[Developer docs](DEVELOPMENT.md)** — toolchain, task map, reproducing
  every CI gate locally
- **[Architecture](docs/ARCHITECTURE.md)** — module map, the two halves,
  design decisions
- **[Decision records](docs/adr/README.md)** — the choices that would be
  expensive to reverse

| Document | Covers |
|---|---|
| [`CHANGELOG.md`](CHANGELOG.md) | per-release notes, Keep a Changelog format |
| [`SECURITY.md`](SECURITY.md) | disclosure policy, limits, path safety, supply chain |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | branch and commit conventions, PR expectations |
| [`GOVERNANCE.md`](GOVERNANCE.md) | who decides what, how changes land |
| [`SUPPORT.md`](SUPPORT.md) | where to ask, what to expect |
| [`AGENTS.md`](AGENTS.md) | invariants for AI-assisted contributions |

---

## Stability guarantees

- **Versioning.** [SemVer](https://semver.org), with the pre-1.0 posture
  that the patch number is the breaking axis during `0.0.x`. Releases
  increment by `+0.0.1` and every breaking change is called out in
  [`CHANGELOG.md`](CHANGELOG.md).
- **Output stability.** What the crate produces is part of the API: a
  change to how a document parses, which `Value` variant a scalar
  becomes, or what `to_string` emits is treated as breaking even when no
  Rust signature moves.
- **Deprecations** live for at least two releases with a `#[deprecated]`
  note naming the replacement before removal.
- **Version-bearing files** are checked against the manifest by
  `scripts/verify-release-versions.sh` before a tag exists, so an install
  snippet cannot go stale. That gate is why this README's version is
  right.

---

## Minimum-toolchain policy

The floor is **Rust 1.88.0**, declared as `rust-version` in
`Cargo.toml` so Cargo refuses older toolchains with a clear message.

- **When it may rise:** only on a release, never silently, and always
  with the reason in the changelog entry.
- **Why it is where it is:** the floor follows the highest requirement
  in the dependency graph, not an aspiration.
- **What is verified:** CI builds and tests on stable. The floor is the
  version Cargo enforces from the manifest.

No claim is made about distro-LTS toolchains. Making one would require a
table mapping current distro versions to this floor, and an aspirational
claim there is worse than none.

---

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT),
at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you shall be dual-licensed as
above, without any additional terms or conditions.

<p align="right"><a href="#frontmatter-gen">Back to top</a></p>
