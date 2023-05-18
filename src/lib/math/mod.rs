//! This modules defines math features like
//! 1. `Matrix` that is just `Grid<f64>`
//! 2. Precision features like roundation and approximate equality
//! 3. Types related to analytical geometry like `VectorSpace`, `Point`, `CoordSys`


mod matrix;
mod precision;
mod coord_sys;

#[cfg(test)]
mod test;


pub use {
    matrix::{Matrix, Vector},
    coord_sys::{VectorSpace, Point, CoordSys},
    precision::{
        round, aeq,
        set_exact_mode,
        set_round_mode,
        set_precision,
    },
};
use {
    once_cell::sync::OnceCell,
};


const DIM: usize = 3;

static mut BIFORM: OnceCell<Matrix> = OnceCell::new();


pub fn set_biform_identity() {
    let biform = Matrix::identity(DIM);
    unsafe {
        if let Some(bf) = BIFORM.get_mut() {
            *bf = biform;
        } else {
            BIFORM.set(biform).expect("BIFORM initialization failed");
        }
    }
}

pub fn set_biform(double: Vec<Vec<f64>>) {
    let biform = Matrix::from_double(double);
    if biform.is_square() {
        unsafe {
            if let Some(bf) = BIFORM.get_mut() {
                *bf = biform;
            } else {
                BIFORM.set(biform).expect("BIFORM initialization failed");
            }
        }
    }
}

pub fn get_biform() -> &'static Matrix {
    unsafe {
        BIFORM.get().expect("BIFORM is not initialized")
    }
}

