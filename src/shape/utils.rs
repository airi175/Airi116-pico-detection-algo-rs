use std::io::{Error, Read};

use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, Dyn, OMatrix, UninitMatrix, U2};

#[inline]
pub fn read_shape<R: Read>(mut reader: R, size: usize) -> Result<OMatrix<f32, U2, Dyn>, Error>
where
  