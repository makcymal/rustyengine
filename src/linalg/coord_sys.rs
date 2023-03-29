use {
    crate::{
        globals::{
            DIM, GRAMM, Flt,
        },
        linalg::matrixified::Vector,
    },
    std::ops::{
        Add, Sub,
    },
};
use crate::globals::VECSPACE;

pub struct Vecspace {
    pub basis: [Vector; DIM],
}

impl Vecspace {
    pub const fn empty() -> Self {
        Self { basis: [Vector::empty(), Vector::empty(), Vector::empty()] }
    }

    pub fn identity() -> Self {
        Self {
            basis: [Vector::from(vec![1.0, 0.0, 0.0]),
                Vector::from(vec![0.0, 1.0, 0.0]),
                Vector::from(vec![0.0, 0.0, 1.0])],
        }
    }
}

impl From<[Vector; DIM]> for Vecspace {
    fn from(basis: [Vector; DIM]) -> Self {
        Self { basis }
    }
}


pub struct Point {
    radvec: Vector,
}

impl Point {
    // only in orthogonal basis
    pub fn as_vector(&self) -> Vector {
        unsafe {
            Vector::from(vec![
                (&self.radvec ^ &VECSPACE.basis[0]) / VECSPACE.basis[0].length(),
                (&self.radvec ^ &VECSPACE.basis[1]) / VECSPACE.basis[1].length(),
                (&self.radvec ^ &VECSPACE.basis[2]) / VECSPACE.basis[2].length()])
        }
    }
}

impl From<Vector> for Point {
    fn from(radvec: Vector) -> Self {
        Self { radvec }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.radvec + rhs)
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point::from(&self.radvec - rhs)
    }
}


pub struct CoordSys {
    init_pt: Point,
    vecspace: Vecspace,
}
