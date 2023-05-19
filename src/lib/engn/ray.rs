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
        math::{
            Matrix, Point, CoordSys,
        }
    },
    std::{
        rc::Rc,
    },
};


/// Ray as pinned to `inc` point vector directed as `dir`
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub(in super) cs: Rc<CoordSys>,
    pub(in super) inc: Point,
    pub(in super) dir: Matrix,
}

impl Ray {
    /// Constructs ray validating dir, setting it to `Col`
    pub fn new(cs: &Rc<CoordSys>, inc: Point, mut dir: Matrix) -> ReRes<Self> {
        dir.ag_failed()?.ag_not_row_or_col()?;
        dir = dir.to_col();
        Ok(Self { cs: Rc::clone(cs), inc, dir })
    }

    /// Resize `dir` vector to length 1
    pub fn normalize(&mut self) {
        self.cs.space().normalize(&mut self.dir);
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            cs: Rc::new(CoordSys::default()),
            inc: Point::default(),
            dir: Matrix::col(vec![1.0, 0.0, 0.0]),
        }
    }
}


/// Bunch of rays pinned to single point
#[derive(Debug, Clone, PartialEq)]
pub struct InceptedRays {
    pub(in super) cs: Rc<CoordSys>,
    pub(in super) inc: Point,
    pub(in super) directions: Grid<Matrix>,
    pub(in super) lens: Matrix,
}

impl InceptedRays {
    pub fn dir_att(&self, r: usize, c: usize) -> &Matrix {
        self.directions.att(r, c)
    }

    pub fn len_att(&self, r: usize, c: usize) -> f64 { *self.lens.att(r, c) }
}
