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

impl From<ComparisonNode> for [i8; 4] {
    #[inline]
    fn from(node: ComparisonNode) -> [i8; 4] {
        [node.0.y, node.0.x, node.1.y, node.1.x]
    }
}

impl From<ComparisonNode> for [u8; 4] {
    #[inline]
    fn from(node: ComparisonNode) -> [u8; 4] {
        [
            node.0.y.to_le_bytes()[0],
            node.0.x.to_le_bytes()[0],
            node.1.y.to_le_bytes()[0],
            node.1.x.to_le_bytes()[0],
        ]
    }
}

impl ComparisonNode {
    #[inline]
    pub fn bintest<I: GenericImageView<Pixel = Luma<u8>>>(
        &self,
        image: &I,
        point: Point2<i32>,
        size: u32,
    ) -> bool {
        let p0 = transform(point, size, self.0.cast());
        let p1 = transform(point, size, self.1.cast());

        let lum0 = unsafe { clamp_pixel_unchecked(image, p0.x, p0.y) }.0[0];
        let lum1 = unsafe { clamp_pixel_unchecked(image, p1.x, p1.y) }.