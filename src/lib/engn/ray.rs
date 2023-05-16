use {
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MatrixErr::{self, *},
            CoordSysErr::{self, *},
        },
        math::{
            Matrix, Point, CoordSys,
        }
    },
    std::{
        rc::Rc,
    },
};


#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    cs: Rc<CoordSys>,
    inc: Point,
    dir: Matrix,
}

impl Ray {
    pub fn new(cs: &Rc<CoordSys>, inc: Point, mut dir: Matrix) -> ReRes<Self> {
        dir.ag_failed()?.ag_not_row_or_col()?;
        dir = dir.to_col();
        Ok(Self { cs: Rc::clone(cs), inc, dir })
    }
}
