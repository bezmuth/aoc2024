use aoc2024::days::day03;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::prelude::*;

fn harness() {
    let mut input = File::open("inputs/input3").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();

    day03::solve(contents);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day3", |b| b.iter(|| harness()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
