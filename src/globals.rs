use crate::vecspace::{
    matrixified::Matrix,
};

pub type CNT = f64;

pub static mut BIFORM: Option<Matrix<CNT>> = None;
