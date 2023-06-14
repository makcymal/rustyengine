//! This modules defines math features like
//! 1. `Matrix` that is just `Grid<f64>`
//! 2. Precision features like roundation and approximate equality
//! 3. Types related to analytical geometry like `VectorSpace`, `Point`, `CoordSys`

pub mod matrix;
mod precision;
mod space;

#[cfg(test)]
mod test;

use std::cmp::Ordering;
use std::ops::{Add, MulAssign, Sub};

pub use {
    matrix::{set_biform, set_biform_identity, set_biform_vec, Matrix},
    precision::{aeq, round, set_exact_mode, set_precision, set_round_mode},
    space::{Basis, CoordSys, Point, Vector},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

#[inline(always)]
pub(crate) fn pow_minus(x: usize) -> f64 {
    match x % 2 {
        0 => 1.0,
        1 => -1.0,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Float(pub f64);

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Into<f64> for Float {
    fn into(self) -> f64 {
        self.0
    }
}

impl Into<f64> for &Float {
    fn into(self) -> f64 {
        self.0
    }
}

impl Add<f64> for Float {
    type Output = Float;

    fn add(self, rhs: f64) -> Self::Output {
        Float(self.0 + rhs)
    }
}

impl Sub<f64> for Float {
    type Output = Float;

    fn sub(self, rhs: f64) -> Self::Output {
        Float(self.0 - rhs)
    }
}

impl MulAssign<f64> for Float {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs
    }
}
