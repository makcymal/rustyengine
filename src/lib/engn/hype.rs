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
    pub(crate) core: Entity,
    pub(crate) initpt: Point,
    pub(crate) normal: Vector,
    pub(crate) is_visible: bool,
}

impl HypePlane {
    /// HypePlane constructor takes actual `GameObject`, `Point` on plane and normal vector
    pub fn new(core: Entity, initpt: Point, mut normal: Vector) -> ReRes<Self> {
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
            is_visible: true,
        })
    }
}

impl AsEntity for HypePlane {
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

impl AsGameObject for HypePlane {
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

    fn change_visibility(&mut self) {
        self.is_visible = !self.is_visible
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn intersect(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        let denom = cs.scalar_prod(&dir.coord, &self.normal.coord).unwrap();
        if aeq(&denom, &0.0) {
            return -1.0;
        }
        let numer = cs
            .scalar_prod(&self.initpt.df(inc).unwrap().coord, &self.normal.coord)
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
    pub(crate) core: Entity,
    pub(crate) cen: Point,
    pub(crate) dir: Vector,
    pub(crate) is_visible: bool,
}

impl AsEntity for HypeEllipse {
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

impl AsGameObject for HypeEllipse {
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

    fn change_visibility(&mut self) {
        self.is_visible = !self.is_visible
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn intersect(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        todo!()
    }
}
