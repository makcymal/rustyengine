use {
    once_cell::sync::OnceCell,
};

pub const EPSILON: f64 = f64::EPSILON * 10.0;

// <<< linalg

use crate::linalg::{
    coord_sys::{Vecspace, CoordSys},
    matrixify::{Matrix, Vector},
    init_biform, init_coordsys,
    common_matrix,
};

pub const DIM: usize = 3;
pub static BIFORM: OnceCell<Matrix> = OnceCell::new();
pub static COORDSYS: OnceCell<CoordSys> = OnceCell::new();

pub fn init_linalg() {
    init_biform();
    init_coordsys();
}

// linalg >>>
