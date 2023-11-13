use image::GenericImageView;
use imageproc::rect::Rect;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Padding {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl Padding {
    #[inline]
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    #[inline]
    pub fn top(self, value: i32) -> Self {
        Self { top: value, ..self }
    }

    #[inline]
    pub fn right(self, value: i32) -> Self {
        Self { right: value, ..self }
    }

    #[inline]
    pub fn bottom(self, value: i32) -> Self {
        Self { b