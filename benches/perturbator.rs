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
    let mut rng = Xoroshiro128PlusPlus::s