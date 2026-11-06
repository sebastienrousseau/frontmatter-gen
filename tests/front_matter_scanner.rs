// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Front-matter scanner regressions: the body starts where the block
//! ends, a block opens only at the top of the document, and only a
//! fence on a line of its own opens or closes it.

#[cfg(feature = "ssg")]
use frontmatter_gen::engine::Engine;
use frontmatter_gen::{
    extractor::{
        extract_delimited_frontmatter, extract_json_frontmatter,
        extract_raw_frontmatter,
    },
    Error,
};

#[test]
fn json_body_starts_after_leading_whitespace() {
    let (raw, body) =
        extract_raw_frontmatter("  {\"a\":1}\nBody").unwrap();
    assert_eq!(raw, "{\"a\":1}");
    assert_eq!(body, "Body");
}

#[test]
fn json_after_multibyte_whitespace_does_not_panic() {
    // With the old `content[json.len()..]` slice, a 3-byte
    // lead puts the cut inside `é` and panics.
    let json = "{\"a\":\"\u{e9}\"}";
    for lead in ["\u{3000}", "\u{feff}", "\u{2003}\n"] {
        let content = format!("{lead}{json}\nBody");
        let result = std::panic::catch_unwind(|| {
            extract_raw_frontmatter(&content)
                .map(|(r, b)| (r.to_owned(), b.to_owned()))
        });
        let (raw, body) =
            result.expect("no panic").expect("front matter found");
        assert_eq!(raw, json);
        assert_eq!(body, "Body");
    }
}

#[test]
fn json_slice_is_taken_from_the_input() {
    let content = "\u{3000}{\"a\":1}\nBody";
    let raw = extract_json_frontmatter(content).unwrap();
    assert_eq!(raw, "{\"a\":1}");
}

#[test]
fn opening_fence_must_start_the_document() {
    for content in [
        "Intro ---\ntitle: x\n---\nBody",
        "Intro\n\n---\ntitle: x\n---\nBody",
        "Intro +++\ntitle = 1\n+++\nBody",
    ] {
        assert!(
            extract_raw_frontmatter(content).is_err(),
            "{content:?} has no front matter"
        );
    }
    assert!(extract_delimited_frontmatter(
        "Intro ---\ntitle: x\n---\nBody",
        "---\n",
        "\n---"
    )
    .is_none());
}

#[test]
fn closing_fence_must_be_a_whole_line() {
    let (raw, body) = extract_raw_frontmatter(
        "---\ntitle: \"A --- B\"\nnote: x\n---- not\n---\nBody",
    )
    .unwrap();
    assert_eq!(raw, "title: \"A --- B\"\nnote: x\n---- not");
    assert_eq!(body, "Body");
}

#[test]
fn fences_tolerate_bom_crlf_and_trailing_space() {
    let (raw, body) = extract_raw_frontmatter(
        "\u{feff}---  \r\ntitle: x\r\n---\t\r\nBody",
    )
    .unwrap();
    assert_eq!(raw, "title: x");
    assert_eq!(body, "Body");
}

#[test]
fn empty_block_and_missing_body() {
    assert_eq!(
        extract_raw_frontmatter("---\n---\nBody").unwrap(),
        ("", "Body")
    );
    assert_eq!(
        extract_raw_frontmatter("---\ntitle: x\n---").unwrap(),
        ("title: x", "")
    );
}

#[test]
fn opening_fence_must_be_alone_on_its_line() {
    for content in ["---", "--- x\n---\n", "----\n---\n"] {
        assert!(
            extract_raw_frontmatter(content).is_err(),
            "{content:?} has no front matter"
        );
    }
}

#[test]
fn unterminated_block_is_reported() {
    assert!(matches!(
        extract_raw_frontmatter("---\ntitle: x\nBody"),
        Err(Error::ExtractionError(_))
    ));
    assert!(matches!(
        extract_raw_frontmatter("+++\ntitle = 1\nBody"),
        Err(Error::ExtractionError(_))
    ));
}

#[cfg(feature = "ssg")]
#[test]
fn fence_text_inside_a_value_is_not_a_delimiter() {
    let engine = Engine::new().expect("engine");
    let (metadata, body) = engine
        .extract_front_matter("---\ntitle: \"A --- B\"\n---\nBody")
        .expect("front matter parses");
    assert_eq!(
        metadata.get("title").and_then(|v| v.as_str()),
        Some("A --- B")
    );
    assert_eq!(body, "Body");
}

#[cfg(feature = "ssg")]
#[test]
fn horizontal_rules_are_not_front_matter() {
    let engine = Engine::new().expect("engine");
    let content = "Intro\n\n---\n\nMiddle\n\n---\n\nEnd";
    let (metadata, body) = engine
        .extract_front_matter(content)
        .expect("no front matter is not an error");
    assert!(metadata.is_empty());
    assert_eq!(body, content);
}

#[cfg(feature = "ssg")]
#[test]
fn body_keeps_its_own_horizontal_rules() {
    let engine = Engine::new().expect("engine");
    let (metadata, body) = engine
        .extract_front_matter("---\ntitle: T\n---\nA\n\n---\n\nB")
        .expect("front matter parses");
    assert_eq!(
        metadata.get("title").and_then(|v| v.as_str()),
        Some("T")
    );
    assert_eq!(body, "A\n\n---\n\nB");
}
