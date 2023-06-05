//! This modules defines math features like
//! 1. `Matrix` that is just `Grid<f64>`
//! 2. Precision features like roundation and approximate equality
//! 3. Types related to analytical geometry like `VectorSpace`, `Point`, `CoordSys`

pub mod matrix;
mod precision;
// mod coord_sys;
mod space;

#[cfg(test)]
mod test;

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
