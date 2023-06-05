use {
    super::*,
    crate::{
        errs::{
            GameErr::{self, *},
            MathErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        grid::Repr,
        math::*,
    },
    std::{
        any::Any,
        cell::RefCell,
        collections::HashMap,
        fmt::{Formatter, Pointer},
        rc::Rc,
    },
    uuid::Uuid,
};

/// Hype plane defined with some point on it and normal vector
#[derive(Debug)]
pub struct HypePlane {
    pub(crate) core: Core,
    pub(crate) initpt: Point,
    pub(crate) normal: Vector,
}

impl HypePlane {
    /// HypePlane constructor takes actual `GameObject`, `Point` on plane and normal vector
    pub fn new(core: Core, initpt: Point, mut normal: Vector) -> ReRes<Self> {
        normal.coord.ag_failed()?.ag_not_row_or_col()?;
        if initpt.dim() != normal.dim() {
            return Err(MathErr(PtVecDimMismatch {
                pt_dim: initpt.dim(),
                vec_dim: normal.dim(),
            }));
        }
        if normal.coord.repr() == Repr::Row {
            normal.coord = normal.coord.transpose();
        }
        Ok(Self {
            core,
            initpt,
            normal,
        })
    }
}

impl Entity for HypePlane {
    fn id(&self) -> &Rc<Uuid> {
        self.core.id()
    }

    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>> {
        self.core.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>> {
        self.core.props_mut()
    }
}

impl GameObject for HypePlane {
    fn pos(&self) -> &Point {
        &self.initpt
    }

    fn pos_mut(&mut self) -> &mut Point {
        &mut self.initpt
    }

    fn dir(&self) -> &Vector {
        &self.normal
    }

    fn dir_mut(&mut self) -> &mut Vector {
        &mut self.normal
    }

    fn intersect(&self, cs: &CoordSys, ray: &Ray) -> f64 {
        let denom = cs.scalar_prod(&ray.dir.coord, &self.normal.coord).unwrap();
        if aeq(&denom, &0.0) {
            return -1.0;
        }
        let numer = cs
            .scalar_prod(&self.initpt.df(&ray.inc).unwrap().coord, &self.normal.coord)
            .unwrap();
        let dist = numer / denom;
        if dist < 0.0 {
            return -1.0;
        }
        dist
    }
}

#[derive(Debug)]
pub struct HypeEllipse {
    pub(crate) core: Core,
    pub(crate) cen: Point,
    pub(crate) dir: Vector,
}

impl Entity for HypeEllipse {
    fn id(&self) -> &Rc<Uuid> {
        self.core.id()
    }

    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>> {
        self.core.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>> {
        self.core.props_mut()
    }
}

impl GameObject for HypeEllipse {
    fn pos(&self) -> &Point {
        &self.cen
    }

    fn pos_mut(&mut self) -> &mut Point {
        &mut self.cen
    }

    fn dir(&self) -> &Vector {
        &self.dir
    }

    fn dir_mut(&mut self) -> &mut Vector {
        &mut self.dir
    }

    fn intersect(&self, cs: &CoordSys, ray: &Ray) -> f64 {
        todo!()
    }
}
