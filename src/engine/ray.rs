use {
    crate::{
        linal::{
            Vector, Point, CoordSys,
        }
    },
    std::{
        rc::Rc,
    },
};

pub struct Ray {
    cs: Rc<CoordSys>,
    inc: Point,
    dir: Vector,
}

impl Ray {
    pub fn from(cs: &Rc<CoordSys>, inc: Point, dir: Vector) -> Self {
        Self { cs: Rc::clone(cs), inc, dir }
    }
}
