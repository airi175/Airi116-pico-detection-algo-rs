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
    pub fn classify<I>(&self, image: &I, region: Square) -> Option<f32>
    where
        I: GenericImageView<Pixel = Luma<u8>>,
    {
        let mut result = 0.0f32;
        let point = region.center();

        for tree in self.forest.iter() {
            let idx = (0..self.depth).fold(1, |idx, _| {
                2 * idx + !tree.nodes[idx].bintest(image, point, region.size()) as usize
            });
            let lutidx = idx - self.dsize;
            result += tree.predictions[lutidx];

            if result < tree.threshold {
                return None;
            }
        }
        Some(result - self.threshold)
    }

    #[inline]
    pub fn detect<I>(&self, image: &I, region: Square) -> Option<Detection<Square>>
    where
        I: GenericImageView<Pixel = Luma<u8>>,
    {
        self.classify(image, region)
            .map(|score| Detection { region, score })
    }

    /// Create a detector object from a readable source.
    #[inline]
    pub fn load(mut readable: impl Read) -> Result<Self, Error> {
        let mut buffer: [u8; 4] = [0u8; 4];
        // skip first 8 bytes;
     