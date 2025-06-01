use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use prefix_sort::{builtin_sort, prefix_sort, Record};
use rand::Rng;
use std::hint::black_box;

const RECORD_SIZE: i32 = 100;
const NUM_OF_RECORDS: i32 = 2_000_000;

fn benchmark_functions(c: &mut Criterion) {
    // Generate shared input
    let input: Vec<Record> = (0..NUM_OF_RECORDS)
        .map(|_| {
            let mut rng = rand::rng();
            let mut arr = vec![0; RECORD_SIZE as usize];
            rng.fill(arr.as_mut_slice());
            Record { data: arr }
        })
        .collect();

    let mut group = c.benchmark_group("sorts");
    group.bench_function("builtin", |b| {
        b.iter_batched(|| input.clone(), |mut input| builtin_sort(black_box(&mut input)), BatchSize::SmallInput);
    });

    group.bench_function("prefix", |b| {
        b.iter_batched(|| input.clone(), |mut input| prefix_sort(black_box(&mut input)), BatchSize::SmallInput);
    });
    group.finish();
}

criterion_group!(benches, benchmark_functions);
criterion_main!(benches);
