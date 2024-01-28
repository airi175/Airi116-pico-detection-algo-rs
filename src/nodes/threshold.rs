use std::convert::TryInto;
use std::mem::{transmute, MaybeUninit};

#[derive(Debug, Clone, Copy)]
pub struct ThresholdNode {
    pub idx: (usize, usize),
    pub threshold: i16,
}

impl From<[u8; 10]> for ThresholdNode {
    #[inline]
    fn from(data: [u8; 10]) -> Self {
        let idx0 = u32::from_be_bytes(data[0..4].try_into().unwrap()) as usize;
        let idx1 = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
  