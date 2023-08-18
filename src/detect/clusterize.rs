use crate::geometry::{intersection_over_union, Square, Target};
use crate::traits::Region;

use super::detection::Detection;

use nalgebra::Point2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clusterizer {
    pub intersection_threshold: f32,
    pub score_threshold: f32,
}

impl Clusterizer {
    #[inline]
    pub fn intersection_threshold(self, value: f32) -> Self {
        Self {
            intersection_threshold: value,
            ..self
        }
    }

    #[inline]
    pub fn score_threshold(sel