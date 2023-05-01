use std::ops::IndexMut;
/// Vecspace, Point and CoordSys structs.
/// Depends on global 3, BIFORM, COORDSYS and Matrix, Vector from matrixify module.
/// Provides singleton with GRAMM matrix in current basis defined in COORDSYS.

use {
    crate::{
        utils::Size,
        linal::{
            BIFORM,
            matrixify::{
                Matrixify, Matrix, Vector,
                scalar_prod
            }
        },
    },
    std::ops::{
        Add, Sub, Index,
    },
};


// /// Actual coordinate system. Must be initialized in main() function.
// static COORDSYS: OnceCell<CoordSys> = OnceCell::new();
//
// /// Signleton with GRAMM matrix in current basis defined in COORDSYS. Accessible via COORDSYS::gramm().
// static GRAM: OnceCell<Matrix> = OnceCell::new();


// <<< Vecspace

/// Vector space defined by static array of basis vectors of 3 length.
#[derive(Debug)]
pub struct VecSpace {
    /// Basis vectors themselves. Must not be linear-dependent.
    basis: [Vector; 3],
    /// Gram matrix in this basis
    gram: Matrix,
    /// Inversed
    /// False if it's unknown that basis vectors are orthogonal pairwise.
    ortho: bool,
}

impl VecSpace {
    /// The most common orthonormal basis.
    pub fn identity() -> Self {
        let mut basis = Default::default();
        for i in 0..3 {
            basis[i] = Vector::from(vec![0.0; 3]);
            basis[i][i] = 1.0;
        }

        let mut gram = Matrix::zeros(Size::Rect((3, 3)));
        for row in 0..3 {
            for col in 0..3 {
                gram[(row, col)] = &basis[row] % &basis[col];
            }
        }

        Self { basis, gram, ortho: true }
    }
}

/// Creates basis with the given vectors without any checks for linear-independency or orthonormality.
impl From<[Vector; 3]> for VecSpace {
    fn from(basis: [Vector; 3]) -> Self {
        let mut gram = Matrix::zeros(Size::Rect((3, 3)));
        for row in 0..3 {
            for col in 0..3 {
                gram[(row, col)] = &basis[row] % &basis[col];
            }
        }

        let mut ortho = true;
        for i in 0..3 {
            let (lhs, rhs) = (&basis[i], &basis[(i + 1) % 3]);
            if lhs % rhs != 0.0 {
                ortho = false;
                break;
            }
        }

        Self { basis, gram, ortho }
    }
}

/// Returns immutable reference to indexed basis vector. Panics if index is out of bounds.
impl Index<usize> for VecSpace {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.basis[index])
    }
}

/// Returns mutable reference to indexed basis vector. Panics if index is out of bounds.
impl IndexMut<usize> for VecSpace {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.basis[index])
    }
}

impl PartialEq for VecSpace {
    fn eq(&self, other: &Self) -> bool {
        self.basis == other.basis
    }
}

// Vecspace >>>


// <<< Point

/// The end of the radius vector, pinned to origin of coordinates.
#[derive(Debug, PartialEq)]
pub struct Point {
    /// Radius vector itself.
    radvec: Vector,
}

impl Point {
    /// Returns Point in origin of coordinates.
    pub fn zeros() -> Self {
        Self::from(Vector::zeros(Size::Row(3)))
    }

    // Takes point and returns it's radius vector in actual basis. Works only in orthogonal basis, else panics.
    pub fn as_vector(&self) -> Vector {
        if !CoordSys::global().is_ortho() {
            panic!("Basis may be not orthogonal");
        }

        let vs: &VecSpace = CoordSys::vecspace();
        let mut inner = vec![];
        for i in 0..3 {
            inner.push((&self.radvec ^ &vs[i]) / vs[i].length())
        }
        Vector::from(inner)
    }
}

/// Creates Point from the Vector under assumption it's pinned in the origin of the coordinates.
impl From<Vector> for Point {
    fn from(radvec: Vector) -> Self {
        Self { radvec }
    }
}

/// Provides shifting for Point in direction and length of Vector;
impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.radvec + rhs)
    }
}

/// Provides shifting for Point in counter direction and length of Vector;
impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.radvec - rhs)
    }
}

// Point >>>


// <<< CoordSys

/// Vector space + initial point.
#[derive(Debug, PartialEq)]
pub struct CoordSys {
    initpt: Point,
    vecspace: VecSpace,
}

impl CoordSys {
    /// Basic constructor.
    pub fn from(init_pt: Point, vecspace: VecSpace) -> Self {
        Self { initpt: init_pt, vecspace }
    }

    /// Provides access to the private field property - vecspace.surely_is_ortho
    pub fn is_ortho(&self) -> bool {
        self.vecspace.ortho
    }

    /// Returns global COORDSYS from singleton. Panics if it's not initialized yet.
    pub fn global() -> &'static CoordSys {
        COORDSYS.get().expect("COORDSYS is not initialized")
    }

    /// Returns actual vector space of global COORDSYS. Panics if it's not initialized yet.
    pub fn vecspace() -> &'static VecSpace {
        &CoordSys::global().vecspace
    }

    /// Returns actual GRAM matrix in actual basis. Panics if it's not initialized yet.
    pub fn gram() -> &'static Matrix {
        CoordSys::global().vecspace.gram()
    }
}

// CoordSys >>>
