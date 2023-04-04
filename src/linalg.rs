// Matrix and Vector structs of arbitrary size.
// Depends only on global BIFORM matrix used in definition of scalar product without basis.
// Does not depends on global VECSPACE, GRAMM matrix or COORDSYS
// so related vector, scalar product defined in coord_sys module.
pub mod matrixify;
// Vecspace, Point and CoordSys structs.
// Depends on global DIM.
pub mod coord_sys;
#[cfg(test)]
mod tests;

use {
    crate::{
        globals::{
            DIM, BIFORM
        },
        enums::MatrixType,
        utils::Size,
    },
    matrixify::{
        Matrixify, Matrix, Vector, scalar_prod,
    },
    coord_sys::{
        Vecspace, Point, CoordSys,
    },
    std::ops::{
        BitOr, BitXor, Rem,
    },
};
use crate::globals::COORDSYS;


impl Matrix {
    pub fn biform() -> &'static Matrix {
        BIFORM.get().expect("BIFORM does not initialized")
    }
}


// scalar product without basis
// &Vector % &Vector = Float
impl Rem for &Vector {
    type Output = f64;

    fn rem(self, rhs: Self) -> Self::Output {
        scalar_prod(self, Matrix::biform(), rhs)
    }
}

// scalar product in basis
// &Vector ^ &Vector = Float
impl BitXor for &Vector {
    type Output = f64;

    fn bitxor(self, rhs: Self) -> Self::Output {
        scalar_prod(self, CoordSys::gramm(), rhs)
    }
}

// vector product in basis
// &Vector | &Vector = Vector
impl BitOr for &Vector {
    type Output = Vector;

    fn bitor(self, rhs: Self) -> Self::Output {
        if DIM != 3 || self.length != 3 || rhs.length != 3 {
            panic!("Trying to compute vector product in non 3D space");
        }

        Vector::from(vec![self[1] * rhs[2] - self[2] * rhs[1],
                          self[2] * rhs[0] - self[0] * rhs[2],
                          self[0] * rhs[1] - self[1] * rhs[0]])
    }
}


pub fn common_matrix(m_type: MatrixType) -> Matrix {
    if DIM != 3 {
        panic!("Call for common 3D Matrix in non-3D space");
    }

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


pub fn init_biform() {
    BIFORM.set(common_matrix(MatrixType::Identity)).expect("BIFORM initialization failed");
}


pub fn init_coordsys() {
    let init_pt = Point::zeros();
    let vecspace = Vecspace::identity();
    COORDSYS.set(CoordSys::from(init_pt, vecspace)).expect("COORDSYS initialization failed");
}
