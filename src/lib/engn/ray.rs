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
    cs: Rc<CoordSys>,
    inc: Point,
    dir: Matrix,
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


/// Bunch of rays pinned to signle point
#[derive(Debug, Clone)]
pub struct InceptedRays {
    cs: Rc<CoordSys>,
    inc: Point,
    directions: Grid<Matrix>,
}
