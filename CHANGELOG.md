# Changelog

All notable changes to `frontmatter-gen` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.12] - 2026-10-09

A security-driven dependency release: `noyalib` moves to `=0.0.56`,
which clears RUSTSEC-2026-0333 from every graph that depends on this
crate, and the declared Rust floor now matches what the dependency
graph actually requires.

### Security

- **`noyalib` pinned at `=0.0.56`**, from `=0.0.37`. Versions before
  0.0.53 are affected by
  [RUSTSEC-2026-0333](https://rustsec.org/advisories/RUSTSEC-2026-0333)
  (GHSA-4xcc-23fx-w2wj): `ParserConfig` budgets such as `max_events`
  and `max_nodes` were not enforced on the typed deserialization path.
  This crate parses with the default configuration and never tightens
  those budgets, and the default length, depth and alias caps always
  applied, so no input was unbounded here. The pin still put the
  affected version into every consumer's lockfile, where `cargo deny`
  and `cargo audit` fail on it. The bump also brings noyalib's
  0.0.54 to 0.0.56 hardening: readers stop one byte past
  `max_document_length`, and `Value` nesting is capped on every path.

### Changed (parse behaviour)

- Two inputs that parsed under noyalib 0.0.37 are now refused, both
  following noyalib: YAML nested deeper than 256 levels (the default
  `max_depth` of 128 already refused it, and this crate does not raise
  it), and an implicit mapping key holding a flow collection longer
  than 1024 characters, which YAML 1.2.2 forbids. Front matter that
  stays clear of both parses to the same values: every example's output
  was compared under 0.0.37 and 0.0.56 and differs only in hash-map key
  order, which was never stable.

### Fixed

- **The declared Rust floor was false.** `rust-version` said 1.85.0,
  but `noyalib` has required 1.86.0 since before 0.0.37, and the
  committed lockfile's `time` 0.3.55 and the optional `dtt` 0.0.11
  require 1.88.0, so Rust 1.85 failed with a dependency error rather
  than the clear refusal the README promised. The floor is now 1.88.0,
  the highest requirement in the graph, per the minimum-toolchain
  policy, and `cargo +1.88.0 test --all-features --locked` passes.
- `deny.toml` listed three licences no dependency uses (BSD-3-Clause,
  ISC, Unicode-DFS-2016) and ignored an advisory for a crate no longer
  in the graph (RUSTSEC-2026-0173, `proc-macro-error2`). cargo-deny
  warned on all four; they are removed, so a new licence or ignore
  arrives with its own review.
- `quality.yml` named two `taiki-e/install-action` pins as v2.87.4 when
  the SHAs are later releases, and repeated one comment twice.

### Added

- **An `msrv` CI job** that reads `rust-version` from `Cargo.toml` and
  runs `cargo check --all-features --all-targets --locked` on exactly
  that toolchain, so the floor cannot drift from the graph unnoticed
  again.

### Dependencies

- `clap` 4.6.7, `tera` 2.4.0, `thiserror` 2.0.21, `toml` 1.1.6 and
  `uuid` 1.26.1 (#50); `dtolnay/rust-toolchain`,
  `taiki-e/install-action` 2.87.15 and `astral-sh/setup-uv` 10.1.0 in
  CI (#48). The noyalib bump from Dependabot (#47) is superseded by the
  0.0.56 pin above.
- cargo-vet: the new versions take over their predecessors' exemptions,
  noyalib 0.0.56 is covered by the existing trusted-publisher entry, and
  `pastey` and `shlex` are now covered by imported audits, so the
  exemption baseline drops from 153 to 151.
- The fuzz workspace's lockfile follows the crate to 0.0.12 and noyalib
  0.0.56.

## [0.0.11] - 2026-09-07

The repository-standard release: the layout, gates and documents every
crate in the family shares, plus two real defects the new tests found.

### Changed (breaking: output)

- **YAML serialisation changed shape**, following the `noyalib` bump.
  Plain scalars are no longer quoted (`date: 2025-09-09`, not
  `date: '2025-09-09'`) and block sequences are indented under their
  key:

  ```yaml
  # before                # after
  tags:                   tags:
  - rust                    - rust
  - example                 - example
  ```

  Both remain valid YAML and re-parse to the same values, so nothing
  breaks at the data level. It is called out here because this crate
  treats what it produces as part of its API: anything comparing
  generated YAML byte for byte — a golden file, a checked-in site — will
  see a diff on upgrade. JSON and TOML output are unchanged.

### Fixed

- **`validate_path_safety` skipped its own last two rules for absolute
  paths.** The function returned `Ok` as soon as it saw an absolute
  path, so the symlink and Windows-reserved-name checks below never ran:
  a symlink passed validation whenever it arrived as an absolute path.
  The early return is gone and every rule now applies to every path.
- **The engine's `SizeCache` evicted on replacement.** `insert` made
  room whenever the map was at capacity, including when the key was
  already present, so replacing a value dropped an unrelated entry. And
  because the victim is whichever key the map iterates first, it
  sometimes dropped the entry being replaced and returned `None` for a
  key that existed a moment earlier. It now only evicts for a genuinely
  new key.

### Changed

- **`noyalib` pinned at `=0.0.37`**, from a caret `0.0.28`. A caret is
  wrong for a same-author 0.0.x line where any release may break
  (ADR-0005).
- **Dev-only dependencies moved into `[dev-dependencies]`.** `tempfile`
  (used only inside `#[cfg(test)]` modules) and `env_logger` (examples
  only) were declared as runtime dependencies, so every consumer built
  them. `simple_logger` and `euxis-commons` are removed outright:
  neither is referenced anywhere in the crate.
- **`build.rs` removed.** `rust-version` is the MSRV gate and Cargo
  enforces it without a build script.
- **`Cargo.lock` is committed** and CI builds `--locked`.

### Added

- **Fuzz harness** (`fuzz/`): `fuzz_extract` (the returned body must be
  a suffix of the input), `fuzz_parse` (all three parsers against every
  input) and `fuzz_roundtrip` (parse, serialise, re-parse, same keys,
  the only thing that catches a serialiser dropping a key). A committed
  seed corpus and a `regressions/` directory replay on every push.
- **`quality.yml`**, a second CI workflow holding the gates the shared
  pipeline does not cover: the coverage threshold, Miri, the fuzz build
  and corpus replay, the docs lint (markdownlint, codespell, REUSE),
  cargo-vet with an exemption ratchet, and release hygiene. Nothing in
  this release is enforced only by a local `make` target.
- **Coverage gate at 98% lines**, excluding `src/main.rs`, the CLI
  binary's entry point, reachable only by running the binary and with
  every library function behind it covered directly. The argument is in
  `DEVELOPMENT.md` rather than assumed. Coverage rose from 89.7% to
  98.0% through exhaustive suites over every error variant, every
  `Value` variant, every format-converter branch, every builder setter
  and every path-safety rule.
- Repository standard layout: `DEVELOPMENT.md`, `docs/ARCHITECTURE.md`,
  `docs/adr/` with five decisions, `CODE_OF_CONDUCT.md`,
  `GOVERNANCE.md`, `SECURITY.md`, `SUPPORT.md`, `AGENTS.md`,
  `CITATION.cff`, `KEYS.asc`, `REUSE.toml` (REUSE 3.3 compliant),
  `rust-toolchain.toml`, `.devcontainer/`, `.pre-commit-config.yaml`,
  `.codespellrc`, `.markdownlint.yaml`, issue and PR templates.
- `scripts/verify-release-versions.sh` and `supply-chain/` with
  cargo-vet trust entries and an exemption baseline.
- **README rewritten** to the family's structural template. The previous
  one was 103 lines and claimed a Rust floor of 1.56.0 while the
  manifest said 1.85.0.

### Removed

- `input.md` and `.deepsource.toml`: a stray fixture and configuration
  for a service the family does not use. Nothing references either.

## [0.0.10] - 2026-09-05

### Changed

- **YAML scalars are moved into `Value`, not copied.** `parse_yaml`
  consumes the parsed mapping by value, yet passed each entry by reference
  to a converter that allocated a fresh `String` for every scalar it had
  just been handed. A moving converter removes one allocation per value:
  on a 2,000-document corpus (~26,000 values) that is 202k → 178k
  allocations. Total bytes barely move (22.4 → 21.8 MB) because the
  remaining volume is allocated inside the YAML parser itself, a 24×
  amplification over the input — recorded here so the next person chasing
  heap in this path starts there rather than in this conversion.

### Fixed

- README install snippet said `0.0.6`; it had not been bumped since that
  release. A test now checks it against `Cargo.toml`.

## [0.0.9] - 2026-08-10

### Changed

- Bumped `noyalib` 0.0.17 -> 0.0.18, preserving
  `default-features = false, features = ["std"]`.
- Bumped the minor-and-patch group (9 updates), including clap 4.6.6.

> Note: 0.0.7 and 0.0.8 shipped without changelog entries. They are not
> reconstructed here rather than be given invented content.

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
- **Dependency bumps** (absorb open dependabot PRs #17–#20):
  - `toml` 0.8.19 → 1.1.2. Breaking semantic in 1.x:
    `toml::Value::from_str` parses a single TOML *value*
    (e.g. `42`, `"hi"`), not a document. `parse_toml` in
    `src/parser.rs` now parses into `toml::Table` instead, and
    `examples/error_examples.rs::validate_toml_parsing` does the same.
  - `dtt` 0.0.9 → 0.0.10 (security: closes RUSTSEC time
    stack-exhaustion DoS; transitively bumps MSRV to 1.88.0 when the
    `ssg` feature is enabled — see dtt's `docs/msrv-policy.md`).
  - `pulldown-cmark` 0.12.2 → 0.13.4 (fixes `TightParagraph` panic).
  - `criterion` 0.5.1 → 0.8.2 (dev-dep). `criterion::black_box` is now
    deprecated; the benchmark switches to `std::hint::black_box`.

### Fixed

- `cargo fmt --all -- --check` now passes — `use noyalib::…` import
  ordering in `src/error.rs` and `src/parser.rs` was the sole reason
  the `ci / Check & Test` job has been red on the dtt-0.0.10
  dependabot branch (#20). Rustfmt's default `StdExternalCrate` import
  groups put `noyalib` before `serde_json`.

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
