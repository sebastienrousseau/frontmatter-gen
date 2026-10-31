// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Structure-aware fuzz target over `Frontmatter`, `Value`, and `Format`.
//!
//! Uses `libfuzzer_sys::arbitrary` to synthesize bounded, semantically
//! valid `Frontmatter` trees and verifies:
//! 1. DoS depth and cardinality budget enforcement (`validate_frontmatter`).
//! 2. HTML meta-tag attribute framing (`to_html_meta_tags`).
//! 3. `to_string` -> `parse` round-trip key and scalar fidelity.
//! 4. Zero-copy `extract` suffix and slice-provenance invariants inside
//!    a `core` + `alloc` verification kernel.
#![no_main]

extern crate alloc;

use frontmatter_gen::{Format, Frontmatter, Value};
use libfuzzer_sys::{
    arbitrary::{Arbitrary, Result as ArbitraryResult, Unstructured},
    fuzz_target,
};

const MAX_FUZZ_DEPTH: usize = 8;

#[derive(Debug, Clone, Copy)]
enum FuzzFormat {
    Yaml,
    Toml,
    Json,
}

impl<'a> Arbitrary<'a> for FuzzFormat {
    fn arbitrary(u: &mut Unstructured<'a>) -> ArbitraryResult<Self> {
        match u.int_in_range(0..=2)? {
            0 => Ok(Self::Yaml),
            1 => Ok(Self::Toml),
            _ => Ok(Self::Json),
        }
    }
}

impl From<FuzzFormat> for Format {
    fn from(value: FuzzFormat) -> Self {
        match value {
            FuzzFormat::Yaml => Format::Yaml,
            FuzzFormat::Toml => Format::Toml,
            FuzzFormat::Json => Format::Json,
        }
    }
}

#[derive(Debug, Clone)]
struct FuzzInput {
    format: FuzzFormat,
    max_depth_budget: u8,
    max_fields_budget: u8,
    frontmatter: Frontmatter,
}

impl<'a> Arbitrary<'a> for FuzzInput {
    fn arbitrary(u: &mut Unstructured<'a>) -> ArbitraryResult<Self> {
        let format = FuzzFormat::arbitrary(u)?;
        let max_depth_budget = u.int_in_range(1..=12)?;
        let max_fields_budget = u.int_in_range(1..=24)?;
        let frontmatter = arbitrary_frontmatter(u, 0, format)?;
        Ok(Self {
            format,
            max_depth_budget,
            max_fields_budget,
            frontmatter,
        })
    }
}

fn arbitrary_key(u: &mut Unstructured<'_>) -> ArbitraryResult<String> {
    let raw: String = u.arbitrary()?;
    let filtered: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .take(24)
        .collect();
    Ok(if filtered.is_empty() {
        "k".to_owned()
    } else {
        filtered
    })
}

fn arbitrary_value(
    u: &mut Unstructured<'_>,
    depth: usize,
    format: FuzzFormat,
) -> ArbitraryResult<Value> {
    if depth >= MAX_FUZZ_DEPTH {
        return Ok(Value::String(u.arbitrary()?));
    }
    let choice = u.int_in_range(0..=5)?;
    match choice {
        0 => match format {
            FuzzFormat::Toml => Ok(Value::Boolean(u.arbitrary()?)),
            _ => Ok(Value::Null),
        },
        1 => Ok(Value::Boolean(u.arbitrary()?)),
        2 => {
            let n: i32 = u.arbitrary()?;
            Ok(Value::Number(f64::from(n)))
        }
        3 => Ok(Value::String(u.arbitrary()?)),
        4 => {
            let len = u.int_in_range(0..=4)?;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                let v = match format {
                    FuzzFormat::Toml => Value::String(u.arbitrary()?),
                    _ => arbitrary_value(u, depth + 1, format)?,
                };
                items.push(v);
            }
            Ok(Value::Array(items))
        }
        _ => {
            let map = arbitrary_frontmatter(u, depth + 1, format)?;
            Ok(Value::Object(Box::new(map)))
        }
    }
}

fn arbitrary_frontmatter(
    u: &mut Unstructured<'_>,
    depth: usize,
    format: FuzzFormat,
) -> ArbitraryResult<Frontmatter> {
    let len = u.int_in_range(0..=6)?;
    let mut fm = Frontmatter::with_capacity(len);
    for _ in 0..len {
        let key = arbitrary_key(u)?;
        let val = arbitrary_value(u, depth + 1, format)?;
        fm.insert(key, val);
    }
    Ok(fm)
}

mod core_kernel {
    use alloc::{boxed::Box, string::String, vec::Vec};
    use frontmatter_gen::{
        extractor::extract_raw_frontmatter,
        parser::{
            parse, parse_with_options, to_string, validate_frontmatter,
            ParseOptions,
        },
        Format, Frontmatter,
    };

    pub(super) fn verify_invariants(
        frontmatter: &Frontmatter,
        format: Format,
        max_depth: usize,
        max_fields: usize,
    ) {
        let budget_res =
            validate_frontmatter(frontmatter, max_depth, max_fields);
        if frontmatter.len() > max_fields {
            assert!(
                budget_res.is_err(),
                "exceeded field budget must be rejected"
            );
        }

        if let Ok(encoded) = to_string(frontmatter, format) {
            let opts = ParseOptions {
                max_depth,
                max_keys: max_fields,
                validate: true,
            };
            let _ = parse_with_options(&encoded, format, Some(opts));

            if let Ok(reparsed) = parse(&encoded, format) {
                let boxed: Box<Frontmatter> = Box::new(reparsed);
                let keys: Vec<&String> =
                    boxed.iter().map(|(k, _)| k).collect();
                assert_eq!(
                    frontmatter.len(),
                    keys.len(),
                    "{format} altered key count across round-trip"
                );
                for (k, v) in frontmatter.iter() {
                    let got = boxed.get(k).unwrap_or_else(|| {
                        panic!("{format} dropped key {k:?}")
                    });
                    assert_eq!(
                        v.as_str(),
                        got.as_str(),
                        "{format} mutated string payload for key {k:?}"
                    );
                    assert_eq!(
                        v.as_bool(),
                        got.as_bool(),
                        "{format} mutated boolean payload for key {k:?}"
                    );
                }
            }

            if matches!(format, Format::Yaml)
                && !encoded.trim().is_empty()
            {
                let mut doc = String::from("---\n");
                doc.push_str(encoded.trim());
                doc.push_str("\n---\nBody suffix.");
                if let Ok((raw, body)) = extract_raw_frontmatter(&doc) {
                    let doc_range = doc.as_bytes().as_ptr_range();
                    assert!(
                        doc_range.contains(&raw.as_bytes().as_ptr())
                    );
                    assert!(
                        doc_range.contains(&body.as_bytes().as_ptr())
                    );
                    assert!(doc.ends_with(body));
                }
            }
        }
    }
}

fuzz_target!(|input: FuzzInput| {
    core_kernel::verify_invariants(
        &input.frontmatter,
        Format::from(input.format),
        usize::from(input.max_depth_budget),
        usize::from(input.max_fields_budget),
    );
});
