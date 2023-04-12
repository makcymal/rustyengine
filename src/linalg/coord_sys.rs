/// Vecspace, Point and CoordSys structs.
/// Depends on global DIM, BIFORM, COORDSYS and Matrix, Vector from matrixify module.
/// Provides singleton with GRAMM matrix in current basis defined in COORDSYS.

use {
    crate::{
        globals::{
            DIM, COORDSYS,
        },
        utils::Size,
        linalg::matrixify::{
            Matrixify, Matrix, Vector,
        },
    },
    std::ops::{
        Add, Sub, Index,
    },
    once_cell::sync::OnceCell,
};


/// Signleton with GRAMM matrix in current basis defined in COORDSYS. Accessible via COORDSYS::gramm().
static GRAM: OnceCell<Matrix> = OnceCell::new();


// <<< Vecspace

/// Vector space defined by static array of basis vectors of DIM length.
#[derive(Debug)]
pub struct VecSpace {
    /// Basis vectors themselves. Must not be linear-dependent.
    pub basis: [Vector; DIM],
    /// False if it's unknown that basis vectors are orthogonal pairwise.
    surely_is_ortho: bool,
}

impl VecSpace {
    /// The most common orthonormal basis.
    pub fn identity() -> Self {
        let mut basis: [Vector; DIM] = Default::default();
        for i in 0..DIM {
            basis[i] = Vector::from(vec![0.0; DIM]);
            basis[i][i] = 1.0;
        }
        Self { basis, surely_is_ortho: true }
    }

    /// As long as GRAM depends on basis it's defined here.
    /// But this method is private. See CoordSys impl for public method.
    fn gram(&self) -> &'static Matrix {
        if GRAM.get().is_none() {
            let mut gram = Matrix::zeros(Size::Rect((DIM, DIM)));
            for row in 0..DIM {
                for col in 0..DIM {
                    gram[(row, col)] = &self.basis[row] % &self.basis[col];
                }
            }
            GRAM.set(gram).expect("GRAMM initialization failed");
        }

        GRAM.get().expect("GRAMM is not initialized")
    }
}

/// Creates basis with the given vectors without any checks for linear-independency or orthonormality.
impl From<[Vector; DIM]> for VecSpace {
    fn from(basis: [Vector; DIM]) -> Self {
        Self { basis, surely_is_ortho: false }
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
        Self::from(Vector::zeros(Size::Row(DIM)))
    }

    // Takes point and returns it's radius vector in actual basis. Works only in orthogonal basis, else panics.
    pub fn as_vector(&self) -> Vector {
        if !CoordSys::global().surely_is_ortho() {
            panic!("Basis may be not orthogonal");
        }

        let vs: &VecSpace = CoordSys::vecspace();
        let mut inner = vec![];
        for i in 0..DIM {
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
    init_pt: Point,
    vecspace: VecSpace,
}

impl CoordSys {
    /// Basic constructor.
    pub fn from(init_pt: Point, vecspace: VecSpace) -> Self {
        Self { init_pt, vecspace }
    }

    /// Provides access to the private field property - vecspace.surely_is_ortho
    pub fn surely_is_ortho(&self) -> bool {
        self.vecspace.surely_is_ortho
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
