mod common;

use approx::assert_abs_diff_eq;
use image::GrayImage;
use rstest::rstest;

use nalgebra::Point2;

use common::{localize_case, localize_perturbate_case, localizer, localize_perturbate, rng};

use pico_detect::{Localizer, Square, LocalizePerturbate};

#[rstest]
fn test_localizer_localize(
    localizer: Localizer,
    localize_case: (GrayImage, [(Square, Point2<f32>); 2]),
) {
    let (image, tests) = localize_case;

    for (region, point) in tests.iter() {
        assert_abs_diff_eq!(
       