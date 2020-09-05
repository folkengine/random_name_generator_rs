use criterion::{criterion_group, criterion_main, Criterion};
use rnglib::{Language, RNG};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("RNG Fantasy", |b| {
        b.iter(|| RNG::new(&Language::Fantasy).unwrap().generate_name())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
