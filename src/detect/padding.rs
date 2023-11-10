use image::GenericImageView;
use imageproc::rect::Rect;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Padding {
    pub top: i32,
    pub right: i32,
    pub bottom: