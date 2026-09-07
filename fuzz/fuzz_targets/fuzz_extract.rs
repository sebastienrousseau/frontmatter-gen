// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Front-matter extraction over arbitrary input.
//!
//! Beyond not panicking, the returned body must be a suffix of the
//! input: extraction slices, it never fabricates content, and a body
//! longer than what it came from would mean an offset bug.
#![no_main]

use frontmatter_gen::extract;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok((_, body)) = extract(content) {
        assert!(
            body.len() <= content.len(),
            "the body outgrew the document it came from"
        );
        assert!(
            content.ends_with(body),
            "the body is not a suffix of the input"
        );
    }
});
