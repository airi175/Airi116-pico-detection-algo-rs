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
        let threshold = i16::from_be_bytes(data[8..10].try_into().unwrap());
        Self {
            idx: (idx0, idx1),
            threshold,
        }
    }
}

impl From<ThresholdNode> for [u8; 10] {
    #[inline]
    fn from(node: ThresholdNode) -> Self {
        let idx0 = (node.idx.0 as u32).to_be_bytes(); // 4 bytes
        let idx1 = (node.idx.1 as u32).to_be_bytes(); // 4 bytes
        let threshold = node.threshold.to_be_bytes(); // 2 bytes
                                               