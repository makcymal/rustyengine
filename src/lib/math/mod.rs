mod matrix;
mod prec;
mod coord_sys;

#[cfg(test)]
mod test;


use std::iter::Once;
pub use {
    matrix::Matrix,
    prec::round,
};

use {
    once_cell::sync::OnceCell,
};


const DIM: usize = 3;

static BIFORM: OnceCell<Matrix> = OnceCell::new();


pub fn set_biform_identity() {
    let biform = Matrix::identity(DIM);
    BIFORM.set(biform).expect("BIFORM initialization failed");
}

pub fn set_biform(rec: Vec<Vec<f64>>) {
    let biform = Matrix::from_double(rec);
    if biform.is_square() {
        BIFORM.set(biform).expect("BIFORM initialization failed");
    }
}

pub fn get_biform() -> &'static Matrix {
    BIFORM.get().expect("BIFORM is not initialized")
}

