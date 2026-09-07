# 0003. Three parallel converters, kept structurally identical

- **Status:** accepted
- **Date:** 2024-11-17 (recorded 2026-09-07)

## Context

YAML, TOML and JSON each arrive as a foreign value enum, and each needs
mapping onto this crate's `Value`. The tempting alternative is one
generic conversion through `serde_json::Value` as an intermediate,
which would mean one function instead of three.

## Decision

Keep three converters — `yaml_to_value`, `toml_to_value`,
`json_to_value` — written in the same shape, arm for arm, so a change
made to one is visibly missing from the others.

## Consequences

- No lossy intermediate: TOML datetimes and YAML tags reach `Value`
  directly rather than through a JSON round trip.
- Three places to change instead of one. That is the cost, and it is
  paid deliberately: a test asserts all three produce the same `Value`
  for the same logical document, so drift fails rather than hides.
- Each converter is a match over a foreign enum, which is exactly where
  a new upstream variant silently falls into a catch-all arm. Every arm
  is covered by a test naming its variant.
