use criterion::{criterion_group, criterion_main, Criterion};

fn bench_psi(_c: &mut Criterion) {
    todo!("Phase 1: add PSI benchmark vs scipy baseline")
}

fn bench_wasserstein(_c: &mut Criterion) {
    todo!("Phase 1: add Wasserstein benchmark vs scipy baseline")
}

criterion_group!(benches, bench_psi, bench_wasserstein);
criterion_main!(benches);
