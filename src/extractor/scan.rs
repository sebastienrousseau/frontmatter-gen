// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Line-anchored scanning for `---` and `+++` fenced front matter,
//! modelled on metadata-gen's `metadata::scan`. Shared by
//! [`super::extract_raw_frontmatter`] and the SSG engine so the two
//! cannot disagree on where a block starts and ends.

/// What [`fenced_block`] found at the top of a document.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Fenced<'a> {
    /// A complete block: the trimmed text between the fences, and the
    /// byte offset just past the closing fence's line.
    Found {
        /// The trimmed text between the fences.
        raw: &'a str,
        /// Byte offset just past the closing fence's line.
        body_offset: usize,
    },
    /// An opening fence with no closing fence.
    Unterminated,
    /// The document does not open with the fence.
    None,
}

/// Strips a leading UTF-8 byte order mark and whitespace, returning the
/// rest and its byte offset in `content`.
pub(super) fn skip_lead(content: &str) -> (usize, &str) {
    let rest = content
        .strip_prefix('\u{feff}')
        .unwrap_or(content)
        .trim_start();
    (content.len() - rest.len(), rest)
}

/// The byte offset just past the opening fence's line, or `None` when
/// `content` does not open with `fence` alone on its line.
fn opening_fence(content: &str, fence: &str) -> Option<usize> {
    let (lead, rest) = skip_lead(content);
    let after_fence = rest.strip_prefix(fence)?;
    let newline = after_fence.find('\n')?;
    after_fence[..newline]
        .trim()
        .is_empty()
        .then_some(lead + fence.len() + newline + 1)
}

/// Finds a block fenced by `fence` (`---` or `+++`) at the top of
/// `content`.
///
/// The rules, modelled on metadata-gen's scanner:
///
/// - A byte order mark and whitespace may precede the opening fence;
///   any other text before it means the document has no front matter.
/// - The opening fence is alone on its line (trailing whitespace and a
///   `\r` are allowed).
/// - The closing fence is the first later line that, apart from
///   trailing whitespace, is exactly `fence`. A fence inside a value
///   (`title: A --- B`) or a longer rule (`----`) does not close the
///   block.
pub(crate) fn fenced_block<'a>(
    content: &'a str,
    fence: &str,
) -> Fenced<'a> {
    let Some(raw_start) = opening_fence(content, fence) else {
        return Fenced::None;
    };
    let mut line_start = raw_start;
    for line in content[raw_start..].split_inclusive('\n') {
        if line.trim_end() == fence {
            return Fenced::Found {
                raw: content[raw_start..line_start].trim(),
                body_offset: line_start + line.len(),
            };
        }
        line_start += line.len();
    }
    Fenced::Unterminated
}
