
use anyhow::{Context, Result};
use clap::Parser;
use image::DynamicImage;
use pico_detect::{
    clusterize::Clusterizer, multiscale::Multiscaler, DetectMultiscale, Detector,
    LocalizePerturbate, Localizer, Padding, Shaper,
};
use std::path::PathBuf;

use crate::{load_model, localizer, shaper};

#[derive(Parser, Debug)]
#[command(author, version, about = "CLI human face detection using PICO models.")]
pub struct Args {
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub input: PathBuf,

    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub output: PathBuf,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
