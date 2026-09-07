# Security Policy

## Supported Versions

| Version | Supported |
|:--------|:---------:|
| 0.0.x   | Yes       |

## Reporting a Vulnerability

Report security vulnerabilities by emailing **sebastian.rousseau@gmail.com**.

Do not open a public issue for security reports.

Include:

- A description of the vulnerability.
- Steps to reproduce.
- Affected versions.
- Any suggested fix (optional).

Expect an initial response within 48 hours. A fix or mitigation plan will follow within 7 days of confirmation.

## Security Design

frontmatter-gen parses front matter from files a site's authors write,
and — with the `ssg` feature — reads templates and writes generated
output. Untrusted input and filesystem writes are the two surfaces that
matter.

- `#![forbid(unsafe_code)]` — zero unsafe blocks, guaranteed by the
  compiler.
- No C dependencies, no FFI, no network I/O.

### Input limits

Parsing is bounded, and the bounds are configurable rather than implied:

| Limit | Where | Purpose |
|:---|:---|:---|
| `ParseOptions::max_depth` | `parser::parse_with_options` | rejects deeply nested documents before they exhaust the stack |
| `ParseOptions::max_keys` | `parser::parse_with_options` | caps the number of keys a document may declare |
| `ParseOptions::max_size` | `lib` | caps the content a single call accepts |

Validation is **opt-in** on `parse_with_options`: with
`validate: false` the limits are not applied. Callers handling untrusted
input should set `validate: true` and choose limits for their own
deployment. That is stated here because a limit nobody enables is not a
limit.

### Path safety

`utils::fs::validate_path_safety` is the boundary for every path the
`ssg` feature touches. It rejects backslashes, null bytes and control
characters, `..` traversal, symlinks, and Windows reserved device names.
It deliberately does **not** decide which directory a caller may write
to: only the caller knows its own root, and containment is checked
there.

A defect fixed in v0.0.11 is worth naming: an absolute path used to
return early from that function, silently skipping the symlink and
reserved-name checks below it. A symlink passed validation whenever it
arrived as an absolute path. Every rule now applies to every path.

### Fuzzing

`fuzz/` holds three libFuzzer targets: extraction (whose result must be
a suffix of its input), the three format parsers against every input,
and a parse-serialise-reparse round trip that catches a serialiser
dropping a key. A committed seed corpus and every fixed-bug reproducer
replay on each push; see [`DEVELOPMENT.md`](DEVELOPMENT.md).

### Supply Chain

- `cargo-deny` (licences, advisories, sources) and `cargo-audit` in CI.
- Dependency provenance recorded with `cargo-vet` (`supply-chain/`);
  exemptions are regenerated on every dependency change and the CI
  ratchet lets the count shrink but never grow.
- The first-party crates `noyalib` and `dtt` are pinned exactly
  (`=0.0.X`); a bump is a deliberate release of this crate.
- Test-only crates live in `[dev-dependencies]`, so a consumer's build
  does not pull them.
- `Cargo.lock` committed for deterministic builds; CI builds `--locked`.
- All GitHub Actions SHA-pinned.
- REUSE/SPDX compliance linted in CI.

### Commit Integrity

All commits on the main branch are signed, and releases are signed
tags. The release-signing key is published in [`KEYS.asc`](KEYS.asc):

```text
4B7F16C909C7A8EE9BED338A4F047EDF5F90F638
```

Signing key `Sebastien Rousseau <sebastian.rousseau@gmail.com>`,
ed25519, signing-only, expires 2028-08-16. Verify the fingerprint out
of band before trusting it.
