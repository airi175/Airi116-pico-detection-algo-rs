use std::cmp::{min, max};

use crate::traits::Region;

#[inline]
pub fn intersection_over_union<R: Region>(r1: R, r2: R) -> Option<f32> {
    let left = max(r1.left(), r2.left());
    let top = max(r1.top