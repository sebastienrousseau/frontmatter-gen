use criterion::{black_box, criterion_group, criterion_main, Criterion};
use frontmatter_gen::{extract, parser, Format, Frontmatter, Value};

fn benchmark_extract(c: &mut Criterion) {
    let content = r#"---
title: My Post
date: 2023-05-20
tags:
  - rust
  - benchmarking
---
This is the content of the post."#;

    c.bench_function("extract frontmatter", |b| {
        b.iter(|| extract(black_box(content)))
    });
}

fn benchmark_parse_yaml(c: &mut Criterion) {
    let yaml = r#"
title: My Post
date: 2023-05-20
tags:
  - rust
  - benchmarking
"#;

    c.bench_function("parse YAML frontmatter", |b| {
        b.iter(|| parser::parse(black_box(yaml), Format::Yaml))
    });
}

fn benchmark_parse_toml(c: &mut Criterion) {
    let toml = r#"
title = "My Post"
date = 2023-05-20
tags = ["rust", "benchmarking"]
"#;

    c.bench_function("parse TOML frontmatter", |b| {
        b.iter(|| parser::parse(black_box(toml), Format::Toml))
    });
}

fn benchmark_parse_json(c: &mut Criterion) {
    let json = r#"
{
    "title": "My Post",
    "date": "2023-05-20",
    "tags": ["rust", "benchmarking"]
}
"#;

    c.bench_function("parse JSON frontmatter", |b| {
        b.iter(|| parser::parse(black_box(json), Format::Json))
    });
}

fn benchmark_to_format(c: &mut Criterion) {
    let mut frontmatter = Frontmatter::new();
    frontmatter.insert("title".to_string(), Value::String("My Post".to_string()));
    frontmatter.insert("date".to_string(), Value::String("2023-05-20".to_string()));
    frontmatter.insert("tags".to_string(), Value::Array(vec![
        Value::String("rust".to_string()),
        Value::String("benchmarking".to_string()),
    ]));

    c.bench_function("convert to YAML", |b| {
        b.iter(|| frontmatter_gen::to_format(black_box(&frontmatter), Format::Yaml))
    });

    c.bench_function("convert to TOML", |b| {
        b.iter(|| frontmatter_gen::to_format(black_box(&frontmatter), Format::Toml))
    });

    c.bench_function("convert to JSON", |b| {
        b.iter(|| frontmatter_gen::to_format(black_box(&frontmatter), Format::Json))
    });
}

criterion_group!(
    benches,
    benchmark_extract,
    benchmark_parse_yaml,
    benchmark_parse_toml,
    benchmark_parse_json,
    benchmark_to_format
);
criterion_main!(benches);
