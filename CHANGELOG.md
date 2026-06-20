# Changelog

All notable changes to `frontmatter-gen` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.6] - 2026-06-20

### Changed

- **YAML backend swap**: replaced the hand-rolled `crates/serde_yml/`
  local fork with the published [`noyalib`](https://crates.io/crates/noyalib)
  crate (`0.0.8`, pure-Rust, `forbid(unsafe_code)`, zero unsafe
  blocks). Drops ~1250 lines of locally-maintained YAML parser
  (`crates/serde_yml/src/de.rs`) in favour of a maintained upstream.
- The public API surface this crate exposes is unchanged — `Value`,
  `Mapping`, `from_str`, `to_string`, and `Error` are name-for-name
  re-routed through `noyalib`'s equivalents.

### Security

- Resolves the upstream half of `GHSA-gfxp-f68g-8x78` (libyml) and the
  `serde_yml` unsoundness advisory as consumed transitively via
  `static-site-generator`. `noyalib` carries `#![forbid(unsafe_code)]`
  crate-wide.

### Internal

- `noyalib::Mapping` is keyed by `String` directly (rather than the
  `Value`-keyed map the old fork exposed), simplifying the YAML →
  `Frontmatter` lowering in `src/parser.rs`. Non-string keys are no
  longer possible at the type level, so the previous defensive
  `log::warn!` branches collapse to a straight iteration.
- `noyalib::Number::as_f64` returns `f64` directly (not `Option<f64>`).
  The number-coercion path in `yaml_to_value` was simplified
  accordingly.
- `TaggedValue::tag()` / `TaggedValue::value()` are now method calls
  rather than public fields.

## [0.0.5] - earlier

See git history for prior releases — no maintained CHANGELOG existed
before the `noyalib` migration.
