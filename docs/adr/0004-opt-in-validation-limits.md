# 0004. Parse limits exist but are opt-in

- **Status:** accepted
- **Date:** 2026-06-28 (recorded 2026-09-07)

## Context

Deeply nested or enormous front matter can exhaust the stack or memory.
The obvious answer is to enforce depth and key limits on every parse.
But the overwhelmingly common caller is a static-site generator reading
files its own author wrote, where a limit that rejects a legitimate
document is a worse failure than the one it prevents.

## Decision

`ParseOptions` carries `max_depth`, `max_keys` and `max_size`, and
`parse_with_options` enforces them **only when `validate` is true**.
The plain `parse` entry point does not validate.

## Consequences

- Trusted-input callers pay nothing and are never surprised by a
  rejection.
- Untrusted-input callers must opt in, and `SECURITY.md` says so
  plainly rather than implying the limits are always active. A limit
  nobody enables is not a limit.
- The default values matter less than the fact that the choice is the
  caller's; they are documented on `ParseOptions`.
