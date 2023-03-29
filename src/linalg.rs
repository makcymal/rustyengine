pub mod matrixified;
pub mod enums;
pub mod coord_sys;
#[cfg(test)]
mod tests;

use {
    crate::globals::{BIFORM, Flt},
    matrixified::Matrix,
    enums::MatrixType,
};

pub fn common_matrix(m_type: MatrixType) -> Matrix {
    let inner = match m_type {
        MatrixType::Identity => vec![vec![1.0, 0.0, 0.0],
                                     vec![0.0, 1.0, 0.0],
                                     vec![0.0, 0.0, 1.0]],
        MatrixType::NegIdentity => vec![vec![-1.0, 0.0, 0.0],
                                        vec![0.0, -1.0, 0.0],
                                        vec![0.0, 0.0, -1.0]],
        MatrixType::RevIdentity => vec![vec![0.0, 0.0, 1.0],
                                        vec![0.0, 1.0, 0.0],
                                        vec![1.0, 0.0, 0.0]],
        MatrixType::NegRevIdentity => vec![vec![0.0, 0.0, -1.0],
                                           vec![0.0, -1.0, 0.0],
                                           vec![-1.0, 0.0, 0.0]],
        MatrixType::Cross => vec![vec![1.0, 0.0, 1.0],
                                  vec![0.0, 1.0, 0.0],
                                  vec![1.0, 0.0, 1.0]],
        MatrixType::NegCross => vec![vec![-1.0, 0.0, -1.0],
                                     vec![0.0, -1.0, 0.0],
                                     vec![-1.0, 0.0, -1.0]],
        MatrixType::Rhomb => vec![vec![0.0, 1.0, 0.0],
                                  vec![1.0, 0.0, 1.0],
                                  vec![0.0, 1.0, 0.0]],
        MatrixType::NegRhomb => vec![vec![0.0, -1.0, 0.0],
                                     vec![-1.0, 0.0, -1.0],
                                     vec![0.0, -1.0, 0.0]],
        MatrixType::Ones => vec![vec![1.0, 1.0, 1.0],
                                 vec![1.0, 1.0, 1.0],
                                 vec![1.0, 1.0, 1.0]],
    };
    Matrix::from(inner)
}

pub fn set_common_biform(m_type: MatrixType) {
    unsafe {
        BIFORM = common_matrix(m_type);
    }
}

pub fn set_biform_inner(inner: Vec<Vec<Flt>>) {
    unsafe {
        BIFORM = Matrix::from(inner);
    }
}

