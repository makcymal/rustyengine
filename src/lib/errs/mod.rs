//! All that relates to error handling

mod engn_errs;
mod grid_errs;
mod math_errs;

pub use {engn_errs::GameErr, grid_errs::GridErr, math_errs::MathErr};
use {strum_macros::Display, thiserror::Error, std::io};

/// `Result` with `ReErr` as `Err` variant
pub type ReRes<T> = Result<T, ReErr>;

/// Unified errors enum, holds different errors in variants with related names.
/// `ReErr` stands for RustyEngineError
#[derive(Debug, Display, Clone, Copy, PartialEq, Error)]
pub enum ReErr {
    GameErr(GameErr),
    GridErr(GridErr),
    MathErr(MathErr),
    IoError,
}

impl From<io::Error> for ReErr {
    fn from(_error: io::Error) -> Self {
        Self::IoError
    }
}
