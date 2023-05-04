/// Matrix and Vector structs of arbitrary size.
/// Doesn't depend on any global state variable related to linear algebra.
pub mod matrixify;
/// Vecspace, Point and CoordSys structs. Depends on global DIM.
pub mod coord_sys;
#[cfg(test)]
mod tests;

pub use {
    matrixify::{
        Matrix, Vector,
    },
    coord_sys::{
        VecSpace, Point, CoordSys,
    },
};

use {
    crate::{
        linal::matrixify::scalar_prod,
        globals::{
            DIM,
        },
        errs::RotationErr::{self, *},
        utils::{
            Size, pow_minus,
        },
    },
    std::ops::{
        BitOr, BitXor, Rem,
    },
    once_cell::sync::OnceCell,
};


/// Actual bilinear form. Must be iniitialized in main() function.
static BIFORM: OnceCell<Matrix> = OnceCell::new();


impl Matrix {
    /// Returns global BIFORM matrix from singleton.
    /// Panics if it's not initialized yet.
    pub fn biform() -> &'static Matrix {
        BIFORM.get().expect("BIFORM does not initialized")
    }


    /// Returns rotational matrix on plane from one axis to another by given angle.
    pub fn rot(mut from_axis: usize, mut to_axis: usize, mut angle: f64) -> Result<Self, RotationErr> {
        if DIM <= from_axis || DIM <= to_axis {
            return Err(InexistentAxis);
        } else if from_axis == to_axis {
            return Err(RepeatedAxis);
        }

        if from_axis > to_axis {
            (from_axis, to_axis) = (to_axis, from_axis);
            angle = -angle;
        }
        let (cos, sin) = (angle.cos(), angle.sin());

        let mut output = Matrix::identity(Size::Rect((DIM, DIM))).unwrap();
        output[(from_axis, from_axis)] = cos;
        output[(to_axis, to_axis)] = cos;
        output[(from_axis, to_axis)] = pow_minus(from_axis + to_axis) * sin;
        output[(to_axis, from_axis)] = pow_minus(from_axis + to_axis + 1) * sin;

        Ok(output)
    }
}

impl Vector {
    /// Decomposition of the given vector by basis
    pub fn by_basis(&mut self) {

    }

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

/// Provides scalar product in basis: Vector ^ Vector = f64
/// Panics if LHS isn't a Row or RHS isn't a Col or sizes don't match.
impl BitXor for &Vector {
    type Output = f64;

    fn bitxor(self, rhs: Self) -> Self::Output {
        scalar_prod(self, CoordSys::gram(), rhs)
    }
}

/// Provides vector product in basis: Vector | Vector = Vector
/// Panics if actual DIM doesn't equal to 3.
impl BitOr for &Vector {
    type Output = Vector;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self.inner_len() != 3 || rhs.inner_len() != 3 {
            panic!("Trying to compute vector product of vectors with length of not 3");
        }
        Vector::from(vec![
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0]])
    }
}


/// Initializes global singleton BIFORM with identity matrix.
pub fn init_biform() {
    BIFORM.set(Matrix::identity(Size::Rect((3, 3))).unwrap())
        .expect("BIFORM initialization failed");
}

