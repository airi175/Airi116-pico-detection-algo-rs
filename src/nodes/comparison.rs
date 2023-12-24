use image::{GenericImageView, Luma};
use nalgebra::Point2;

use pixelutil_image::clamp_pixel_unchecked;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct ComparisonNode(pub Point2<i8>, pub Point2<i8>);

impl From<[i8; 4]> for ComparisonNode {
    #[inline]
    fn from(data: [i8; 4]) -> Self {
        let [y0, x0, y1, x1] = d