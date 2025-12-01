use criterion::{Criterion, criterion_group, criterion_main};
use one::part_two;
use std::{fs::read_to_string, hint::black_box};

fn criterion_benchmark(c: &mut Criterion) {
    let input = read_to_string("input.txt").unwrap();
    c.bench_function("part 2", |b| b.iter(|| part_two(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
