use std::cmp::{min, max};

use crate::traits::Region;

#[inline]
pub fn intersection_over_union<R: Region>(r1: R, r2: R) -> Option<f32> {
    let left = max(r1.left(), r2.left());
    let top = max(r1.top(), r2.top());
    let right = min(r1.right(), r2.right());
    let bottom = min(r1.bottom(), r2.bottom());

    if right < left || bottom < top {
        return None;
    }

    let width = (right - left) as u32 + 1;
    let height = (bottom - top)