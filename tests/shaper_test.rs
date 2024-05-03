mod common;

use rstest::rstest;
use approx::assert_abs_diff_eq;

use image::GrayImage;
use nalgebra::Point2;

use pico_detect: