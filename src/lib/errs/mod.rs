mod grid_errs;
mod math_errs;

pub use {
    grid_errs::GridErr::{self, *},
    math_errs::{
        MatrixErr::{self, *},
        CoordSysErr::{self, *},
    },
};
use strum_macros::Display;

/// `Result` with `ReErr` as `Err` variant
pub type ReRes<T> = Result<T, ReErr>;

/// Unified errors enum, holds different errors in variants with related names.
/// `ReErr` stands for RustyEngineError
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum ReErr {
    GridErr(GridErr),
    MatrixErr(MatrixErr),
    CoordSysErr(CoordSysErr),
}
