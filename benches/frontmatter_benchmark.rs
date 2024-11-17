//! Benchmarks for the `frontmatter-gen` crate.
//!
//! This file includes benchmarks for extracting, parsing, and formatting frontmatter
//! in various formats such as YAML, TOML, and JSON. It uses the `criterion` crate
//! for accurate performance measurements.

#![allow(missing_docs)]
use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use frontmatter_gen::{extract, parser, Format, Frontmatter, Value};

// Benchmarks the `extract` function for extracting frontmatter from content.
//
// This benchmark measures the performance of extracting frontmatter
// from a Markdown-like file containing a YAML frontmatter block.
#[allow(dead_code)]
fn benchmark_extract(c: &mut Criterion) {
    let content = r#"---
title: My Post
date: 2025-09-09
tags:
  - rust
  - benchmarking
---
This is the content of the post."#;

    let _ = c.bench_function("extract frontmatter", |b| {
        b.iter(|| extract(black_box(content)))
    });
}

// Benchmarks the `parser::parse` function for parsing YAML frontmatter.
//
// This benchmark measures the performance of parsing frontmatter written in YAML format.
#[allow(dead_code)]
fn benchmark_parse_yaml(c: &mut Criterion) {
    let yaml = r#"
title: My Post
date: 2025-09-09
tags:
  - rust
  - benchmarking
"#;

    let _ = c.bench_function("parse YAML frontmatter", |b| {
        b.iter(|| parser::parse(black_box(yaml), Format::Yaml))
    });
}

// Benchmarks the `parser::parse` function for parsing TOML frontmatter.
//
// This benchmark measures the performance of parsing frontmatter written in TOML format.
#[allow(dead_code)]
fn benchmark_parse_toml(c: &mut Criterion) {
    let toml = r#"
title = "My Post"
date = 2025-09-09
tags = ["rust", "benchmarking"]
"#;

    let _ = c.bench_function("parse TOML frontmatter", |b| {
        b.iter(|| parser::parse(black_box(toml), Format::Toml))
    });
}

// Benchmarks the `parser::parse` function for parsing JSON frontmatter.
//
// This benchmark measures the performance of parsing frontmatter written in JSON format.
#[allow(dead_code)]
fn benchmark_parse_json(c: &mut Criterion) {
    let json = r#"
{
    "title": "My Post",
    "date": "2025-09-09",
    "tags": ["rust", "benchmarking"]
}
"#;

    let _ = c.bench_function("parse JSON frontmatter", |b| {
        b.iter(|| parser::parse(black_box(json), Format::Json))
    });
}

// Benchmarks the `to_format` function for converting frontmatter into different formats.
//
// This benchmark measures the performance of serializing a `Frontmatter` instance
// into YAML, TOML, and JSON formats.
#[allow(dead_code)]
fn benchmark_to_format(c: &mut Criterion) {
    let mut frontmatter = Frontmatter::new();
    let _ = frontmatter.insert(
        "title".to_string(),
        Value::String("My Post".to_string()),
    );
    let _ = frontmatter.insert(
        "date".to_string(),
        Value::String("2025-09-09".to_string()),
    );
    let _ = frontmatter.insert(
        "tags".to_string(),
        Value::Array(vec![
            Value::String("rust".to_string()),
            Value::String("benchmarking".to_string()),
        ]),
    );

    let _ = c.bench_function("convert to YAML", |b| {
        b.iter(|| {
            frontmatter_gen::to_format(
                black_box(&frontmatter),
                Format::Yaml,
            )
        })
    });

    let _ = c.bench_function("convert to TOML", |b| {
        b.iter(|| {
            frontmatter_gen::to_format(
                black_box(&frontmatter),
                Format::Toml,
            )
        })
    });

    let _ = c.bench_function("convert to JSON", |b| {
        b.iter(|| {
            frontmatter_gen::to_format(
                black_box(&frontmatter),
                Format::Json,
            )
        })
    });
}

// Defines the Criterion benchmark group for this crate.
//
// This group includes benchmarks for:
// - Extracting frontmatter
// - Parsing frontmatter in YAML, TOML, and JSON formats
// - Converting frontmatter to YAML, TOML, and JSON formats
criterion_group!(
    benches,
    benchmark_extract,
    benchmark_parse_yaml,
    benchmark_parse_toml,
    benchmark_parse_json,
    benchmark_to_format
);

// Defines the Criterion benchmark entry point.
//
// This function is required by the `criterion_main!` macro and acts as
// the entry point for running all defined benchmarks.
criterion_main!(benches);
