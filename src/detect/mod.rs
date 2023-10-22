mod detection;
mod detector;
mod padding;

pub mod clusterize;
pub mod multiscale;

use image::{GenericImageView, Luma};
use derive_builder::Builder;

use crate::geome