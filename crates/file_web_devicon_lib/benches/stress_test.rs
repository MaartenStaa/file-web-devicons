use criterion::{black_box, criterion_group, criterion_main, Criterion};
use file_web_devicon_lib::*;

pub fn bench_small(c: &mut Criterion) {
    let input = include_str!("fd.txt");
    c.bench_function("fd.txt", |b| b.iter(|| bench_with_input(&input)));
}

pub fn bench_large(c: &mut Criterion) {
    let input = include_str!("find.txt");
    c.bench_function("find.txt", |b| b.iter(|| bench_with_input(&input)));
}

pub fn bench_rg(c: &mut Criterion) {
    let input = include_str!("rg.txt");
    c.bench_function("rg.txt", |b| b.iter(|| bench_with_input(&input)));
}

fn bench_with_input(input: &str) {
    for line in input.lines() {
        handle_input_line(black_box(line));
    }
}

criterion_group!(benches, bench_small, bench_large, bench_rg);
criterion_main!(benches);
