use image::{GenericImageView, Luma};
use nalgebra::Point2;

use pixelutil_image::clamp_pixel_unchecked;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct ComparisonNode(pub Point2<i8>, pub Point2<i8>);

impl From<[i8; 4]> for ComparisonNode {
    #[inline]
    fn from(data: [i8; 4]) -> Self {
        let [y0, x0, y1, x1] = data;
        Self(Point2::new(x0, y0), Point2::new(x1, y1))
    }
}

impl From<[u8; 4]> for ComparisonNode {
    #[inline]
    fn from(data: [u8; 4]) -> Self {
        data.map(|value| i8::from_le_bytes(value.to_le_bytes()))
            .into()
    }
}

impl From<