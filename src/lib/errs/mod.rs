//! All that relates to error handling

mod game_errs;
mod grid_errs;
mod math_errs;

use strum_macros::Display;
pub use {game_errs::GameErr, grid_errs::GridErr, math_errs::MathErr};

/// `Result` with `ReErr` as `Err` variant
pub type ReRes<T> = Result<T, ReErr>;

/// Unified errors enum, holds different errors in variants with related names.
/// `ReErr` stands for RustyEngineError
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum ReErr {
    GameErr(GameErr),
    GridErr(GridErr),
    MathErr(MathErr),
}
