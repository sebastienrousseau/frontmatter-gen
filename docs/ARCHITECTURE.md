# `frontmatter-gen` architecture

How the crate is put together, for contributors. The user-facing story
is in the [README](../README.md); this page is about the shape of the
code and the decisions behind it.

## Layout

```text
frontmatter-gen/
├── src/
│   ├── lib.rs        # public surface: extract, ParseOptions, re-exports
│   ├── extractor.rs  # finding the front-matter block and its format
│   ├── parser.rs     # the three format parsers and serialisers
│   ├── types.rs      # Value, Frontmatter, Format
│   ├── error.rs      # Error, EngineError, Context
│   ├── config.rs     # Config + builder            (feature: ssg)
│   ├── engine.rs     # site generation             (feature: ssg)
│   ├── ssg.rs        # the ssg command surface     (feature: ssg)
│   ├── utils.rs      # path safety, temp files, logging (feature: ssg)
│   ├── cli.rs        # argument parsing            (feature: cli)
│   └── main.rs       # the `fmg` binary            (feature: cli)
├── benches/          # Criterion, harness = false
├── examples/         # runnable examples
├── fuzz/             # libFuzzer targets, seed corpus, regressions
├── docs/adr/         # architecture decision records
└── supply-chain/     # cargo-vet state
```

The library compiles with no features. `cli` adds the binary; `ssg`
implies `cli` and adds the site-generation surface. A module gated
behind a feature never appears in a default build, which is why the
default dependency set is small.

## Extraction, then parsing

The two halves are deliberately separate.

**`extractor.rs`** answers "where is the front matter and what format is
it?" without parsing it. `extract_raw_frontmatter` finds a `---` or
`+++` fence or a leading JSON object and returns the raw block plus the
remaining body; `detect_format` inspects the block. Splitting this out
means a caller can see the format before committing to a parse, and the
body is always a slice of the input rather than a rebuilt string.

**`parser.rs`** turns a raw block into a `Frontmatter`.
`parse_with_options` validates the shape against the format, dispatches
to `parse_yaml`, `parse_toml` or `parse_json`, and optionally enforces
the depth and key limits. Each format has a converter —
`yaml_to_value`, `toml_to_value`, `json_to_value` — that maps a foreign
value enum onto this crate's `Value`. The three are kept structurally
parallel so a change to one is visibly missing from the others; a test
asserts all three agree on the same document.

Serialisation runs the same way in reverse, with `to_json_optimised`
reserving a buffer from `estimate_json_size` before writing.

## The value model

`Value` is a small owned enum: null, string, number (`f64`), boolean,
array, object, and a `Tagged` variant for YAML tags. `Frontmatter`
wraps a `HashMap<String, Value>` and exposes a map surface plus
`merge`, in which the incoming map wins.

`Display` for both renders JSON-shaped text. It is a rendering, not a
serialiser: `escape_str` escapes `"` and `\` only, so a string
containing a control character produces output that is not parseable
JSON. Callers that need JSON use `to_string(&fm, Format::Json)`. A test
pins that limitation rather than asserting it is fine.

## Errors

`Error` has one variant per failure class, with `#[from]` conversions
for the three parsers and I/O. `Clone` is hand-written because several
variants hold `Arc`-wrapped or non-`Clone` sources; every arm is tested
to round-trip to its own variant. `Context` carries a line, column and
snippet, and `with_context` folds them into the variants that can
usefully carry them.

`EngineError` is separate and converts into `Error::ParseError`, so the
`ssg` surface can have its own vocabulary without leaking it into the
library's.

## The ssg surface

`Engine` loads templates into a size-bounded cache, walks the content
directory, splits front matter from body, renders through Tera and
writes output. `Config` validates itself on `build()`: language codes,
the base URL, directory paths, and the server port when the server is
enabled.

Every path the engine touches goes through
`utils::fs::validate_path_safety` first. That function checks the shape
of a path — no backslashes, no control characters, no `..`, no
symlinks, no Windows reserved names — and deliberately does not decide
which directory is allowed; only the caller knows its own root.

## Tests

- `src/**` `#[cfg(test)]` — unit tests next to the code, including
  exhaustive suites over every error variant, every `Value` variant,
  every converter branch, every builder setter and every path rule.
- `fuzz/` — three targets: extraction (the body must be a suffix of the
  input), the parsers against every input, and a round trip that
  catches a serialiser dropping a key.
- Doc tests on every public item run under `cargo test`.

## Coverage

The gate is 98% lines, excluding `src/main.rs`. The reasoning is in
[`DEVELOPMENT.md`](../DEVELOPMENT.md).

## Where to read next

- [`docs/adr/`](adr/README.md) for the decisions that shape the above.
- [`DEVELOPMENT.md`](../DEVELOPMENT.md) for reproducing every CI gate.
