//! This modules defines math features like
//! 1. `Matrix` that is just `Grid<f32>`
//! 2. Precision features like roundation and approximate equality
//! 3. Types related to analytical geometry like `VectorSpace`, `Point`, `CoordSys`

// mod matrix;
// mod space;
mod euclide;

#[cfg(test)]
mod test;

use std::cmp::Ordering;
use std::ops::{Add, MulAssign, Sub};

pub use euclide::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

#[inline(always)]
pub(crate) fn pow_minus(x: usize) -> f32 {
    match x % 2 {
        0 => 1.0,
        1 => -1.0,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Float(pub f32);

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Into<f32> for Float {
    fn into(self) -> f32 {
        self.0
    }
}

impl Into<f32> for &Float {
    fn into(self) -> f32 {
        self.0
    }
}

impl Add<f32> for Float {
    type Output = Float;

    fn add(self, rhs: f32) -> Self::Output {
        Float(self.0 + rhs)
    }
}

impl Sub<f32> for Float {
    type Output = Float;

    fn sub(self, rhs: f32) -> Self::Output {
        Float(self.0 - rhs)
    }
}

impl MulAssign<f32> for Float {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs
    }
}

pub static mut EPSILON: f32 = f32::EPSILON;

pub fn set_precision(prec: u8) {
    unsafe { EPSILON *= ((255 - prec + 1) as f32) }
}

pub fn aeq(lhs: f32, rhs: f32) -> bool {
    unsafe { (lhs - rhs).abs() < EPSILON }
}
