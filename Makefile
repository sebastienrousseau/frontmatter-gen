# POSIX-compatible Makefile for frontmatter-gen
# Works on macOS, Linux, and WSL without modification.
#
# Usage:
#   make           — run check + clippy + test (default)
#   make test      — run all tests (all features)
#   make clippy    — run clippy lints, warnings denied
#   make fmt       — check formatting
#   make lint      — docs lint: markdownlint + codespell + REUSE
#   make deny      — cargo-deny supply-chain checks
#   make vet       — cargo-vet provenance check (--locked)
#   make audit     — cargo-audit advisory check
#   make doc       — build documentation with warnings denied
#   make coverage  — line/function coverage via cargo-llvm-cov (nightly)
#   make miri      — run the lib test suite under Miri (nightly)
#   make fuzz      — build every fuzz target and replay the corpus (nightly)
#   make examples  — run every example to completion
#   make bench-smoke — compile and run each bench once, no measurement
#   make versions  — every version-bearing file agrees with Cargo.toml
#   make sbom      — generate a software bill of materials
#   make clean     — remove build artifacts

.PHONY: all check clippy test fmt lint deny vet audit doc coverage miri fuzz examples bench-smoke versions sbom clean

all: check clippy test

check:
	cargo check --all-features --all-targets

clippy:
	cargo clippy --all-features --all-targets -- -D warnings

test:
	cargo test --all-features

fmt:
	cargo fmt --all -- --check

# Docs lint: structure only for Markdown, British spellings allowed,
# REUSE 3.3 compliance. `reuse` runs through uvx so the gate does not
# depend on a system Python install.
lint:
	npx --yes markdownlint-cli2 "**/*.md" "!target/**" "!fuzz/**" "!node_modules/**"
	uvx codespell
	uvx --with chardet reuse lint

deny:
	cargo deny check

vet:
	cargo vet --locked

audit:
	cargo audit --deny warnings

doc:
	RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features

# Coverage as CI measures it. The gate is 98 % lines; the rationale is
# in DEVELOPMENT.md.
# `src/main.rs` is excluded: it is the CLI binary's entry point,
# reachable only by running the binary, and the library surface behind
# it is covered directly. See DEVELOPMENT.md for the argument.
coverage:
	cargo +nightly llvm-cov --all-features \
	  --ignore-filename-regex 'src/main\.rs' --fail-under-lines 98

miri:
	cargo +nightly miri test --lib

# Build every target, then replay the seed corpus and the regression
# inputs without generating new ones. Mirrors the per-push CI gate.
fuzz:
	cd fuzz && cargo +nightly fuzz build
	cd fuzz && for t in $$(cargo +nightly fuzz list); do \
	  cargo +nightly fuzz run "$$t" -- -runs=0 "corpus/$$t" "regressions/$$t" || exit 1; \
	done

# Every declared [[example]] target. The names in Cargo.toml differ from
# the file stems (`error` vs `error_examples.rs`), so the list comes
# from cargo metadata rather than from the directory listing.
examples:
	@cargo metadata --no-deps --format-version 1 \
	  | python3 -c 'import json,sys; print("\n".join(t["name"] for p in json.load(sys.stdin)["packages"] for t in p["targets"] if "example" in t["kind"]))' \
	  | while read -r name; do \
	      echo "== $$name"; \
	      cargo run --quiet --all-features --example "$$name" >/dev/null || exit 1; \
	    done

bench-smoke:
	cargo bench --all-features -- --test

versions:
	./scripts/verify-release-versions.sh

sbom:
	cargo tree --edges normal --prefix depth --format '{p} {l}' > SBOM.txt
	@echo "SBOM written to SBOM.txt"

clean:
	cargo clean
