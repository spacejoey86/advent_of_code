use std::{fs::read_to_string, hint::black_box};
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let inp = black_box(read_to_string("input.txt").unwrap());
    c.bench_function(&"benchmark", |b| b.iter(|| five::run(&inp)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);