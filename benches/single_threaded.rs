use daily_programmer_386::calc_partition_count;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("partition_count 666", |b| b.iter(|| calc_partition_count(666)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);