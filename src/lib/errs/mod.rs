//! All that relates to error handling

mod grid_errs;
mod math_errs;
mod game_errs;
mod none_opts;

pub use {
    grid_errs::GridErr,
    math_errs::MathErr,
    game_errs::GameErr,
    none_opts::NoneOpt,
};
use strum_macros::Display;


/// `Result` with `ReErr` as `Err` variant
pub type ReRes<T> = Result<T, ReErr>;

/// Unified errors enum, holds different errors in variants with related names.
/// `ReErr` stands for RustyEngineError
#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub enum ReErr {
    GameErr(GameErr),
    GridErr(GridErr),
    MathErr(MathErr),
    NoneOpt(NoneOpt),
}
