use nalgebra::Vector2;

#[derive(Debug, Clone)]
pub struct ShaperDelta {
    anchor: usize,
    value: Vector2<f32>,
}

impl ShaperDelta {
    #[inline]
    pub fn new(anchor: us