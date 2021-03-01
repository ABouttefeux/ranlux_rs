
use ranlux_rs::{ranlxs, ranlx_full_word};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand_core::{SeedableRng, RngCore};
use rand_xoshiro;

fn criterion_benchmark(c: &mut Criterion) {
    let mut thread_rng = rand::thread_rng();
    let mut ranlxs = ranlxs::Ranlxs::from_entropy();
    let mut ranlxfw = ranlx_full_word::RanlxFullWord32::from_entropy();
    let mut xo256pp = rand_xoshiro::Xoshiro256PlusPlus::from_entropy();
    
    let mut group_rng = c.benchmark_group("Sim creation deterministe");
    
    group_rng.throughput(Throughput::Bytes(8));
    group_rng.bench_function("ranlxs", |b| {
        b.iter(|| ranlxs.next_u64())
    });
    group_rng.bench_function("Ranlx full word 32", |b| {
        b.iter(|| ranlxfw.next_u64())
    });
    group_rng.bench_function("thread rng", |b| {
        b.iter(|| thread_rng.next_u64())
    });
    group_rng.bench_function("Xoshiro256PlusPlus", |b| {
        b.iter(|| xo256pp.next_u64())
    });
    
    group_rng.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
