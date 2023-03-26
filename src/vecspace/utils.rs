use {
    std::{
        ops::Neg,
        iter::Sum,
    },
    num_traits as nt,
};

pub trait Num: nt::Num + nt::NumAssign + Into<f64> + Copy + Sum + Neg<Output=Self> {}
impl<T: nt::Num + nt::NumAssign + Into<f64> + Copy + Sum + Neg<Output=Self>> Num for T {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    pub x: usize,
    pub y: usize,
}

impl Pair {
    pub fn transpose(&mut self) {
        (self.x, self.y) = (self.y, self.x);
    }

    pub fn in_rect(&self, rtc: Self) -> bool {
        0 <= self.x && self.x < rtc.x && 0 <= self.y && self.y < rtc.y
    }

    pub fn is_vertical(&self) -> bool {
        self.y > self.x
    }
}

impl From<(usize, usize)> for Pair {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[inline]
pub fn pow_minus<T: Num>(deg: usize) -> T {
    match deg % 2 == 0 {
        true => T::one(),
        false => -T::one(),
    }
}
