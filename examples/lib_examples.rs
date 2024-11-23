// Copyright © 2024 FrontMatterGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # FrontMatterGen Library Examples
//!
//! This example demonstrates the core functionality of the FrontMatterGen library,
//! including frontmatter extraction and conversion to various formats.

#![allow(missing_docs)]

use frontmatter_gen::error::Error;
use frontmatter_gen::{extract, to_format, Format, Frontmatter};

/// Entry point for the FrontMatterGen library examples.
///
/// This function runs various examples demonstrating frontmatter extraction and
/// conversion for different scenarios in the FrontMatterGen library.
///
/// # Errors
///
/// Returns an error if any of the example functions fail.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 FrontMatterGen Library Examples\n");

    // Core functionality examples
    extract_example()?;
    to_format_example()?;

    // SSG-specific examples
    #[cfg(feature = "ssg")]
    ssg_examples()?;

    println!("\n🎉  All library examples completed successfully!");
    Ok(())
}

/// Demonstrates extracting frontmatter from content.
fn extract_example() -> Result<(), Error> {
    println!("🦀 Frontmatter Extraction Example");
    println!("---------------------------------------------");

    let yaml_content = r#"---
title: My Post
date: 2025-09-09
---
Content here"#;

    let (frontmatter, remaining_content) = extract(yaml_content)?;
    println!("    ✅  Extracted frontmatter: {:?}", frontmatter);
    println!("    Remaining content: {}", remaining_content);

    assert_eq!(
        frontmatter.get("title").unwrap().as_str().unwrap(),
        "My Post"
    );
    assert_eq!(remaining_content, "Content here");

    Ok(())
}

/// Demonstrates converting frontmatter to a specific format.
fn to_format_example() -> Result<(), Error> {
    println!("\n🦀 Frontmatter Conversion Example");
    println!("---------------------------------------------");

    let mut frontmatter = Frontmatter::new();
    let _ = frontmatter.insert("title".to_string(), "My Post".into());
    let _ = frontmatter.insert("date".to_string(), "2025-09-09".into());

    let yaml = to_format(&frontmatter, Format::Yaml)?;
    println!("    ✅  Converted frontmatter to YAML:\n{}", yaml);

    let json = to_format(&frontmatter, Format::Json)?;
    println!("    ✅  Converted frontmatter to JSON:\n{}", json);

    assert!(yaml.contains("title: My Post"));
    assert!(yaml.contains("date: '2025-09-09'"));
    assert!(json.contains("\"title\": \"My Post\""));
    assert!(json.contains("\"date\": \"2025-09-09\""));

    Ok(())
}

/// SSG-specific examples that are only available with the "ssg" feature
#[cfg(feature = "ssg")]
fn ssg_examples() -> Result<(), Error> {
    println!("\n🦀 SSG-Specific Frontmatter Examples");
    println!("---------------------------------------------");

    let content = r#"---
title: My Blog Post
date: 2025-09-09
template: blog
layout: post
tags:
  - rust
  - ssg
---
# Blog Content Here"#;

    let (frontmatter, content) = extract(content)?;
    println!("    ✅  Extracted SSG frontmatter: {:?}", frontmatter);
    println!("    Content with markdown: {}", content);

    // Convert to different formats
    let yaml = to_format(&frontmatter, Format::Yaml)?;
    let toml = to_format(&frontmatter, Format::Toml)?;
    let json = to_format(&frontmatter, Format::Json)?;

    println!("\n    Formats:");
    println!("    YAML:\n{}", yaml);
    println!("    TOML:\n{}", toml);
    println!("    JSON:\n{}", json);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Core functionality tests
    #[test]
    fn test_basic_extraction() -> Result<(), Error> {
        let content = r#"---
title: Test
---
Content"#;
        let (frontmatter, content) = extract(content)?;
        assert_eq!(
            frontmatter.get("title").unwrap().as_str().unwrap(),
            "Test"
        );
        assert_eq!(content, "Content");
        Ok(())
    }

    #[test]
    fn test_format_conversion() -> Result<(), Error> {
        let mut frontmatter = Frontmatter::new();
        frontmatter.insert("title".to_string(), "Test".into());

        let yaml = to_format(&frontmatter, Format::Yaml)?;
        assert!(yaml.contains("title: Test"));

        let json = to_format(&frontmatter, Format::Json)?;
        assert!(json.contains("\"title\": \"Test\""));

        Ok(())
    }

    // SSG-specific tests
    #[cfg(feature = "ssg")]
    mod ssg_tests {
        use super::*;

        #[test]
        fn test_ssg_metadata() -> Result<(), Error> {
            let content = r#"---
title: Test
template: post
layout: blog
tags:
  - rust
  - ssg
---
Content"#;
            let (frontmatter, _) = extract(content)?;
            assert!(frontmatter.get("template").is_some());
            assert!(frontmatter.get("layout").is_some());
            assert!(frontmatter.get("tags").is_some());
            Ok(())
        }
    }
}
