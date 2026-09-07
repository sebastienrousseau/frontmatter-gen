# 0005. First-party crates are pinned exactly (`=0.0.X`)

- **Status:** accepted
- **Date:** 2026-09-07

## Context

`noyalib` and `dtt` are 0.0.x crates from the same maintainer. Under
Cargo's SemVer rules a 0.0.x release may break, and a caret requirement
on `0.0.28` silently accepts `0.0.29`. This crate carried exactly that
caret until v0.0.11 and took upgrades it had never tested against.

## Decision

Both are required as `=0.0.X`. A bump is a deliberate, tested change to
this crate, released with a changelog entry; the pin and the lockfile
move together, checked by `scripts/verify-release-versions.sh`.

## Consequences

- No surprise upgrades in consumers.
- Every upstream release needs a release here to reach consumers. The
  family accepts that cost; it is the same model noyalib's own
  satellites use.
