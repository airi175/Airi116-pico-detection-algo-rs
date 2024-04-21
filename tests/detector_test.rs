mod common;

use approx::assert_abs_diff_eq;
use image::GrayImage;
use rstest::rstest;

use pico_detect::{DetectMultiscale, Detector, Square, Target};

use common::{classify_case