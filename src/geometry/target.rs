use imageproc::rect::Rect;
use nalgebra::Point2;

use crate::traits::Region;

use super::Square;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    pub(crate) point: Point2<f32>,
    pub(crate) size: f32,
}

impl Target {
    #[inline]
    pub fn new(x: f32, y: f32, s: f32) -> Self {
        Self {
            point: Point2::new(x, y),
            size: s,
        }
    }

    #[inline]
    pub fn size(&self) -> f32 {