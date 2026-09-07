# 0002. Extraction is separate from parsing

- **Status:** accepted
- **Date:** 2024-11-17 (recorded 2026-09-07)

## Context

Finding a front-matter block and understanding its contents are
different questions. A caller often wants the first without the second:
which format is this file, where does the body start, is there any
front matter at all.

## Decision

`extractor.rs` locates the block and detects its format without
parsing. It returns the raw block and the body as **slices of the
input**. `parser.rs` then turns a raw block into a `Frontmatter`.

## Consequences

- A caller can branch on format before paying for a parse.
- The body is a borrow, never a rebuilt string, so extraction cannot
  invent or lose content. `fuzz_extract` asserts exactly that: the body
  must be a suffix of the input.
- The two halves can fail independently, and their errors say which
  half failed.
