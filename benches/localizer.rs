#[path = "./common/macros.rs"]
mod macros;

use std::fs;

use criterion::{black_box, Criterion};

use image;
use pico_detect::{Localizer, Square};

pub fn bench_load(c: &mut Criterion) {
    let model_data = fs::read(model_path!(puploc)).unwrap();

    c.bench_function("Localizer::load", |b| {
        b.iter(|| Localizer::load(black_box(model_data.as_slice())).unwrap())
    });
}

pub fn bench_infere