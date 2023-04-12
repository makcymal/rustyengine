/// Matrix and Vector structs of arbitrary size.
/// Doesn't depend on any global state variable related to linear algebra.
pub mod matrixify;
/// Vecspace, Point and CoordSys structs. Depends on global DIM.
pub mod coord_sys;
/// Rotation matrices constructors. Depends on global DIM.
pub mod rotations;
#[cfg(test)]
mod tests;

use {
    crate::{
        globals::{
            DIM, BIFORM, COORDSYS,
        },
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


impl Matrix {
    /// Returns global BIFORM matrix from singleton.
    /// Panics if it's not initialized yet.
    pub fn biform() -> &'static Matrix {
        BIFORM.get().expect("BIFORM does not initialized")
    }
}

impl Vector {
    /// Computes length on the basis as the sqrt of scalar self squared.
    pub fn length(&self) -> f64 {
        (self ^ self).sqrt()
    }
}


/// Provides scalar product without basis: Vector % Vector = f64
/// Panics if LHS isn't a Row or RHS isn't a Col or sizes don't match.
impl Rem for &Vector {
    type Output = f64;

    fn rem(self, rhs: Self) -> Self::Output {
        scalar_prod(self, Matrix::biform(), rhs)
    }
}

/// Provides scalar product in basis: Vector % Vector = f64
/// Panics if LHS isn't a Row or RHS isn't a Col or sizes don't match.
impl BitXor for &Vector {
    type Output = f64;

    fn bitxor(self, rhs: Self) -> Self::Output {
        scalar_prod(self, CoordSys::gramm(), rhs)
    }
}

/// Provides vector product in basis: Vector | Vector = Vector
/// Panics if actual DIM isn't equal 3.
impl BitOr for &Vector {
    type Output = Vector;

    fn bitor(self, rhs: Self) -> Self::Output {
        if DIM != 3 || self.inner_len() != 3 || rhs.inner_len() != 3 {
            panic!("Trying to compute vector product in non 3D space");
        }

        Vector::from(vec![self[1] * rhs[2] - self[2] * rhs[1],
                          self[2] * rhs[0] - self[0] * rhs[2],
                          self[0] * rhs[1] - self[1] * rhs[0]])
    }
}


/// Initializes global singleton BIFORM with identity matrix.
pub fn init_biform() {
    BIFORM.set(Matrix::identity(Size::Rect((3, 3))).unwrap())
        .expect("BIFORM initialization failed");
}


/// Initializes global singleton COORDSYS with zero point and identity vecspace.
pub fn init_coordsys() {
    let init_pt = Point::zeros();
    let vecspace = Vecspace::identity();
    COORDSYS.set(CoordSys::from(init_pt, vecspace))
        .expect("COORDSYS initialization failed");
}
