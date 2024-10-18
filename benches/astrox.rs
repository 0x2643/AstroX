// Public crates.
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

// Private crates.
use astrox::astrox;

fn astrox_bench(input: &[u8; 32]) {
    astrox::astrox_hash(input);
}

fn criterion_benchmark(c: &mut Criterion) {
    let input: [u8; 32] = [0; 32];
    c.bench_function("AstroX", |b| b.iter(|| astrox_bench(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
