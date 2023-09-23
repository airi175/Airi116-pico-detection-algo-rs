mod tree;

use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read};

use image::{GenericImageView, Luma};

use crate::geometry::Square;
use crate::traits::Region;

use super::Detection;

use tree::DetectorTree;

/// Implements object detection using a cascade of decision tree classifiers.
#[derive(Clone)]
pub struct Detector {
    depth: usize,
    dsize: usize,
    threshold: f32,
    forest: Vec<DetectorTree>,
}

impl Debug for Detector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Detector))
            .field("depth", &self.depth)
            .field("dsize", &self.dsize)
            .field("threshold", &self.threshold)
            .field("trees", &self.forest.len())
            .finish()
    }
}

impl Detector {
    /// Estimate detection score for the rectangular region.
    ///
    /// ### Arguments
    ///
    /// * `image` -- target image;
    ///
    /// ### Returns
    ///
    /// * `Some(f32)` passed region is an object with score;
    /// * `None` -- if passed region is not an object.
    #[inline]
    pub fn classify<I>(&self, image: &I, region: Square) -> Option<f