#[path = "./common/macros.rs"]
mod macros;

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use pico_detect::{Square, perturbate::Perturbator};
use rand::Seed