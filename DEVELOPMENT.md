<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Developing frontmatter-gen

The single entry point for working on this repository. User-facing
documentation lives in the [README](README.md) and [`docs/`](docs/);
contribution etiquette and review expectations live in
[`CONTRIBUTING.md`](CONTRIBUTING.md). This file is the *how*: toolchain,
tasks, and reproducing every CI gate locally.

## Toolchain

| What | Version | Why |
| :--- | :--- | :--- |
| Rust stable | `rust-version` in `Cargo.toml` or later | MSRV, enforced by Cargo and CI |
| Rust nightly | any recent | Miri, cargo-fuzz, coverage (`cargo-llvm-cov`) |
| cargo-deny, cargo-vet, cargo-audit | cargo-vet **0.10.2 or later** | supply-chain gates; earlier cargo-vet rejects a `trusted-publisher` entry |
| cargo-llvm-cov | latest | coverage gate |
| cargo-fuzz | latest, **installed from source** (`cargo install --locked cargo-fuzz`) | the prebuilt musl binary infers its own build triple as the fuzz target and dies on "sanitizer is incompatible with statically linked libc" |
| uv (`uvx`) and npx | any | `reuse`, `codespell`, `markdownlint` for the docs lint |

```bash
git clone https://github.com/sebastienrousseau/frontmatter-gen
cd frontmatter-gen
make            # check + clippy + test — the default gate
```

## Features

| Feature | Adds |
| :--- | :--- |
| *(default, none)* | the library: extraction, the three parsers, `Value`/`Frontmatter` |
| `cli` | the `fmg` binary and its argument parsing (`clap`) |
| `ssg` | the static-site surface: `Engine`, `Config`, the filesystem helpers. Implies `cli`, and adds `tera`, `pulldown-cmark`, `dtt` and `url` |

`--all-features` is what CI runs; a change that compiles only with the
default set is not finished.

## Task map

| Task | Command |
| :--- | :--- |
| Everything a PR needs first | `make` |
| Full test suite | `make test` |
| Lints / formatting | `make clippy` / `make fmt` |
| Docs lint (markdownlint, codespell, REUSE) | `make lint` |
| Docs as CI builds them | `make doc` |
| Coverage gate | `make coverage` |
| Miri | `make miri` |
| Fuzz targets, corpus replay | `make fuzz` |
| All examples | `make examples` |
| Benches compile and run once | `make bench-smoke` |
| Version-bearing files agree | `make versions` |
| Supply chain | `make deny` / `make vet` / `make audit` |

## Reproducing the CI gates

CI has two workflows. [`ci.yml`](.github/workflows/ci.yml) calls the
shared pipelines from
[`sebastienrousseau/pipelines`](https://github.com/sebastienrousseau/pipelines);
[`quality.yml`](.github/workflows/quality.yml) holds the gates the
repository standard requires that the shared pipeline does not cover.

| CI job | Local reproduction | Gotcha |
| :--- | :--- | :--- |
| `ci` (fmt, clippy, test, cross-platform) | `make` | |
| `coverage-gate` | `make coverage` | nightly; 98 % lines, excluding `src/main.rs` |
| `miri` | `make miri` | lib tests; filesystem tests carry `#[cfg_attr(miri, ignore)]` |
| `fuzz-replay` | `make fuzz` | needs a source-installed cargo-fuzz (see above) |
| `docs-lint` | `make lint` | British spellings are house style; see `.codespellrc` |
| `cargo-vet` | `make vet` | after a dep change: `cargo vet regenerate exemptions`; the count must not exceed `supply-chain/exemptions-baseline.txt` |
| `release-hygiene` | `make versions && make examples && make bench-smoke && make doc` | |
| `cargo-audit` | `make audit` | if a local cargo alias named `audit` shadows the subcommand, run `cargo-audit audit` |

## Coverage: the threshold and why

The gate is **98 % lines**, measured with `cargo llvm-cov
--all-features` and **excluding `src/main.rs`**.

That exclusion is the only one. `main.rs` is the `fmg` binary's entry
point: argument dispatch, logging setup, feature reporting and process
exit. Reaching it means running the binary and inspecting its exit
status, which tests the harness rather than the crate — and every
library function it calls is covered directly, through `cli.rs` and the
modules below it. Counting it would push effort toward asserting on
`main`'s glue instead of on behaviour.

What remains uncovered inside the gate is mostly defensive: error arms
for conditions the surrounding code has already excluded.

## Test layout

- `src/**` `#[cfg(test)]` — unit tests next to the code, including the
  exhaustive suites that cover every enum variant, every builder setter
  and every path-safety rule.
- `benches/frontmatter_benchmark.rs` — Criterion, declared with
  `harness = false`; libtest's harness would collect zero tests and
  report success.
- `fuzz/fuzz_targets/` — `fuzz_extract` (the body must be a suffix of
  the input), `fuzz_parse` (every parser against every input) and
  `fuzz_roundtrip` (parse, serialise, re-parse, same keys).
  `fuzz/corpus/<target>` is the committed seed set and
  `fuzz/regressions/<target>` holds every fixed-bug reproducer. Both
  replay per push. A crash found by fuzzing lands as a regression input
  in the same commit as its fix.

## Release model

Versions increment strictly by `+0.0.1`. Before tagging, run
`make versions`: it checks `Cargo.toml`, `Cargo.lock`, the `noyalib`
and `dtt` pins, `CITATION.cff`, the `CHANGELOG.md` heading and every
install snippet. Tags are signed (`git tag -s vX.Y.Z`); the key is in
[`KEYS.asc`](KEYS.asc).

Publishing to crates.io is manual (`cargo publish` from the tagged
commit); the repository has no tag-triggered release workflow yet.

## House rules

- CI must be green in the same session that turned it red.
- Commits are signed; releases are signed tags.
- Structure cleanups never couple to code changes.
- New behaviour lands with its test in the same commit; a regression
  fix lands with the input that found it.
