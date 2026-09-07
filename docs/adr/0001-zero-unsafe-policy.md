# 0001. `#![forbid(unsafe_code)]`, pure Rust, no FFI

- **Status:** accepted
- **Date:** 2024-10-03 (recorded 2026-09-07)

## Context

The crate parses front matter written by a site's authors and, with the
`ssg` feature, reads templates and writes generated output. Untrusted
text and filesystem writes are the two risky surfaces, and nothing in
either needs raw pointers.

## Decision

`#![forbid(unsafe_code)]` at the crate root; no C dependencies, no FFI.
A dependency that requires `unsafe` in this crate's own code, or that
pulls in a C build, is a reason to pick a different dependency.

## Consequences

- The compiler proves the absence of unsafe blocks.
- Miri's job here is to check the interaction with dependencies that
  use `unsafe` internally, not this crate's own code.
