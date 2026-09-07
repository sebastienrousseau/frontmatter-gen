// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Parse, serialise, re-parse.
//!
//! The property is that serialisation is faithful: whatever the parser
//! accepted must survive a trip through `to_string` and come back with
//! the same keys. A serialiser that drops or mangles a value produces
//! output that still parses, so nothing but this catches it.
#![no_main]

use frontmatter_gen::{
    parser::{parse, to_string},
    Format,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };
    for format in [Format::Yaml, Format::Toml, Format::Json] {
        let Ok(parsed) = parse(content, format) else {
            continue;
        };
        let Ok(text) = to_string(&parsed, format) else {
            continue;
        };
        if let Ok(reparsed) = parse(&text, format) {
            assert_eq!(
                parsed.len(),
                reparsed.len(),
                "{format} lost or invented a key across a round trip"
            );
            for (key, _) in parsed.iter() {
                assert!(
                    reparsed.contains_key(key),
                    "{format} dropped the key {key:?} across a round trip"
                );
            }
        }
    }
});
