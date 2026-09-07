// SPDX-License-Identifier: Apache-2.0 OR MIT
//! The three format parsers, each against every input.
//!
//! Every parser must be total on arbitrary bytes: a document meant for
//! one format is routinely handed to another by a mis-detected fence,
//! so each one has to reject cleanly rather than panic.
#![no_main]

use frontmatter_gen::{parser::parse, Format};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };
    for format in [Format::Yaml, Format::Toml, Format::Json] {
        let _ = parse(content, format);
    }
});
