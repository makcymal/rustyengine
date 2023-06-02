use std::os::windows::io::InvalidHandleError;
use {
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
        },
        grid::Grid,
        math::*,
    },
    std::{
        rc::Rc,
    },
};
use crate::math::matrix::VectorAt;


/// Ray as pinned to `inc` point vector directed as `dir`
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub(in super) inc: Point,
    pub(in super) dir: Vector,
}

impl Ray {
    /// Constructs ray validating dir, setting it to `Col`
    pub fn new(inc: Point, mut dir: Vector) -> ReRes<Self> {
        dir = dir.to_col();
        Ok(Self { inc, dir })
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            inc: Point::default(),
            dir: Vector::col(vec![1.0, 0.0, 0.0]),
        }
    }
}


/// Bunch of rays pinned to single point
#[derive(Debug, Clone, PartialEq)]
pub struct InceptedRays {
    pub(in super) cs: Rc<CoordSys>,
    pub(in super) inc: Point,
    pub(in super) directions: Grid<Matrix>,
}

impl InceptedRays {
    pub fn dir_att(&self, r: usize, c: usize) -> &Matrix {
        self.directions.att(r, c)
    }
}


/// Iterator that yields rays with common inception whose ends compose rectangular with uniform step by two sides
#[derive(Debug, Clone, PartialEq)]
pub struct RectRaysIter {
    inc: Point,
    end: Point,
    hstep: Vector,
    vstep: Vector,
}
