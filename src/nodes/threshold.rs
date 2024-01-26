use std::convert::TryInto;
use std::mem::{transmute, MaybeUninit};

#[derive(Debug, Clone, Copy)]
pub struct ThresholdNode {
    pub idx: (usize, usize),
    pub threshold: i16,
}
