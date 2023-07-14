extern crate pico_detect;

#[macro_use]
mod models;

mod args;

mod face;
mod shape;
mod utils;

use rand::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;

use ab_glyph::FontRef;
use anyhow::{Context, Result};

use face::Face;
use shape::Sha