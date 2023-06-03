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
    matrix::{
        Matrix,
        set_biform, set_biform_identity, set_biform_vec,
    },
    space::{
        Vector, Point, Basis, CoordSys
    },
    precision::{
        round, aeq,
        set_precision,
        set_exact_mode,
        set_round_mode,
    },
};
