//! All that relates to error handling

mod grid_errs;
mod math_errs;
mod engn_errs;

pub use {
    grid_errs::GridErr,
    math_errs::MathErr,
    engn_errs::EngnErr,
};
use strum_macros::Display;


/// `Result` with `ReErr` as `Err` variant
pub type ReRes<T> = Result<T, ReErr>;

/// Unified errors enum, holds different errors in variants with related names.
/// `ReErr` stands for RustyEngineError
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum ReErr {
    EngnErr(EngnErr),
    GridErr(GridErr),
    MathErr(MathErr),
}
