//! This module defines type `Grid`, that is parent for `Matrix` type.
//! Like `Matrix` `Grid` holds collection of elements of type `E`, structured in rectangular table.
//! The reason `Grid` is named as it's named is that it would be more convinient to call widely used
//! `Grid` of floats as `Matrix`
//!
//! `Grid` inner incapsulated in struct `RawGrid` that holds `VecWrapper` and transposition flag.
//! `VecWrapper` is a workaround to deal with `Vec<E>` as well as with `Vec<Vec<E>>`.
//! `VecWrapper::Single` holds `Vec<E>` and `VecWrapper::Double` holds `Vec<Vec<E>>`.
//! `RawGrid` is easy-transposable `VecWrapper`, furthermore it can take additional transpose flag,
//! that provides possibility of treating `Grid` as once more transposed without mutating it.

mod raw_grid;
mod grid;

#[cfg(test)]
mod test;

pub use {
    raw_grid::RawGrid,
    grid::{
        Repr,
        Grid,
        Line,
        Elem
    },
};