use crate::geometry::{intersection_over_union, Square, Target};
use crate::traits::Region;

use super::detection::Detection;

use nalgebra::Point2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clusterizer {
    pub intersection_th