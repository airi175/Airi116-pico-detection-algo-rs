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

    let font = FontRef::try_from_slice(include_bytes!("../../assets/DejaVuSansDigits.ttf"))
        .expect("Failed to load font.");

    let image = image::open(&args.input).context("Failed to load image file.")?;

    let (detector, localizer, shaper) = args.load_models()?;
    let (multiscale, localize) = args.init(&image)?;

    let mut rng = Xoroshiro128PlusPlus::seed_from_u64(42);

    let gray = image.to_owned().into_luma8();

    let faces: Vec<Face> = multiscale
        .run(&detector, 