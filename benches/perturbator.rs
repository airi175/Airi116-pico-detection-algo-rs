#[path = "./common/macros.rs"]
mod macros;

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use pico_detect::{Square, perturbate::Perturbator};
use rand::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;

pub fn bench_run(c: &mut Criterion) {
    let mut group = c.benchmark_group("Perturbator::run");

    group.sample_size(10000);

    let perturbator = Perturbator::default();
    let mut rng = Xoroshiro128PlusPlus::seed_from_u64(42);

    let init = Square::new(100, 100, 100).into();

    for n in [15, 19, 23, 27, 31].iter() {
        let id = Ben