use criterion::{criterion_group, criterion_main, Criterion};
use rgit;

pub fn init_benchmark(c: &mut Criterion) {
    let repo = tempfile::tempdir().unwrap().path().to_owned();
    c.bench_function("init", |b| b.iter(|| rgit::init(&repo).unwrap()));
}

criterion_group!(benches, init_benchmark);
criterion_main!(benches);
