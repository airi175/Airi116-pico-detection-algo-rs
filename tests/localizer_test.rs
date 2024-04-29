mod common;

use approx::assert_abs_diff_eq;
use image::GrayImage;
use rstest::rstest;

use nalgebra::Point2;

use common::{localize_case, lo