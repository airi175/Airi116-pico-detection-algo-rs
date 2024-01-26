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
        let idx0 = u32::from_