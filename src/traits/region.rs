
use nalgebra::Point2;

pub trait Region {
    fn left(&self) -> i32;
    fn top(&self) -> i32;
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    #[inline]
    fn right(&self) -> i32 {
        self.left() + (self.width() - 1) as i32
    }

    #[inline]
    fn bottom(&self) -> i32 {
        self.top() + (self.height() - 1) as i32
    }

    #[inline]
    fn is_square(&self) -> bool {
        self.width() == self.height()
    }