// Copyright © 2024 Shokunin Static Site Generator. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Fenced Code Block Handling Example
//!
//! This example demonstrates how to use the `frontmatter-gen` library to handle content
//! with fenced code blocks containing path traversal patterns. The `validate_input`
//! function ensures that such patterns inside fenced blocks are ignored during validation.
//!
//! ## Usage
//!
//! To run this example:
//!
//! ```bash
//! cargo run --features default --example fenced
//! ```

use anyhow::{Context, Result};
use env_logger::{Builder, Env};
use frontmatter_gen::{extract, validate_input, ParseOptions};
use log::{error, info};

/// Example content with fenced code blocks that simulate a realistic blog post.
const CONTENT: &str = r#"---
title: CSS and trying to map the whole world
date: 2024-02-19 07:04:00 +0000
topics: ["CSS", "HTML"]
---

This is a post about utility-first CSS and its challenges.

```html
<article class="mv4 pa3 bg-light-gray ba b--mid-gray">
    <h2 class="f3 lh-title mb1">Card title</h2>
    <img class="db mv1" src="../img-path" alt="Alt text">
    <p class="ma0 f6">Card text...</p>
</article>
```

Conclusion: CSS utility frameworks can be powerful but have limitations.
"#;

/// Demonstrates handling content with fenced code blocks.
///
/// This function ensures that validation errors are skipped for content inside fenced blocks.
///
/// # Errors
/// Returns an error if validation fails for content outside fenced code blocks.
fn handle_fenced_code_example() -> Result<()> {
    info!("Running example for handling fenced code blocks");

    let options = ParseOptions::default();

    // Validate input content
    validate_input(CONTENT, &options)
        .context("Validation failed for input content")?;

    // Extract front matter and remaining content
    let (frontmatter, remaining) =
        extract(CONTENT).context("Failed to extract front matter")?;

    println!(
        "
📄 Fenced Code Block Example:"
    );
    println!("Front Matter:");
    println!("{:#?}", frontmatter);
    println!(
        "
Content:"
    );
    println!("{}", remaining);

    Ok(())
}

/// Main function demonstrating fenced code block handling.
///
/// # Errors
/// Returns an error if the example fails.
pub fn main() -> Result<()> {
    println!(
        "🧪 Fenced Code Block Handling Example
"
    );

    // Initialize logger
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).format_timestamp(None).init();

    if let Err(e) = handle_fenced_code_example() {
        error!("Example failed: {}", e);
        return Err(e);
    }

    info!("Example completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests validation for content with fenced code blocks.
    #[test]
    fn test_fenced_code_handling() {
        assert!(handle_fenced_code_example().is_ok());
    }

    /// Tests detection of path traversal outside fenced code blocks.
    #[test]
    fn test_path_traversal_outside_fenced_code() {
        let content = r#"---
title: Example
---

../malicious/path
"#;

        let options = ParseOptions::default();
        let result = validate_input(content, &options);

        assert!(result.is_err(), "Validation should detect path traversal outside fenced code blocks");
    }

    /// Tests path traversal handling inside fenced code blocks.
    #[test]
    fn test_path_traversal_inside_fenced_code() {
        let content = r#"---
title: Example
---

```html
<img src="../safe/path" alt="Example">
```
"#;

        let options = ParseOptions::default();
        let result = validate_input(content, &options);

        assert!(result.is_ok(), "Validation should skip path traversal patterns inside fenced code blocks");
    }
}
