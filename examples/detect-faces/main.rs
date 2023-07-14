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
use shape::Shape5;
use utils::{draw_face, print_faces_data};

fn main() -> Result<()> {
    let args = args::parse();

    let font = FontRef::