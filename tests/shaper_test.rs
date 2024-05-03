mod common;

use rstest::rstest;
use approx::assert_abs_diff_eq;

use image::GrayImage;
use nalgebra::Point2;

use pico_detect::{Shaper, Square};

use common::{shaper, shaper_case};

#[rstest]
fn test_shaper_predict(shaper: Shaper, shaper_case: (GrayImage, Square, 