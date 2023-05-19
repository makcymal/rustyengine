use {
    super::*,
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GameErr::{self, *},
            MathErr::{self, *},
            NoneOpt::{self, *},
        },
        math::*,
        grid::Repr,
    },
    std::{
        fmt::{Formatter, Pointer},
        rc::Rc,
        cell::RefCell,
    },
    uuid::Uuid,
};


/// Trait for `GameObject`'s that can be intersected with `Ray`'s, i.e. that are visible on the screen
pub trait Intersected: Entity {
    /// Compute intersection `Point` and distance to it from `inc`
    fn intersect(&self, inc: &Point, dir: &Matrix, len: f64) -> ReRes<f64>;
}

impl std::fmt::Debug for dyn Intersected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}


/// List of entities implementing shared interior mutability
#[derive(Debug)]
pub struct EntityList {
    pub(in super) entities: Vec<Rc<RefCell<dyn Intersected>>>,
}

impl EntityList {
    /// Instantiates empty list
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    /// Appends new entity that must implement Entity
    pub fn append(&mut self, entity: impl Intersected + 'static) {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    /// Removes entity from the list with the given `Uuid`
    pub fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities.retain(|entity| Rc::ptr_eq(&entity.borrow().core().id, id));
    }

    /// Returns shared interior mutable ref to entity if exists
    pub fn get(&self, id: &Rc<Uuid>) -> Option<Rc<RefCell<dyn Intersected>>> {
        if let Some(rc) = self.entities.iter().find(|entity| Rc::ptr_eq(&entity.borrow().core().id, id)) {
            Some(Rc::clone(rc))
        } else {
            None
        }
    }

    /// Permorms closure that may be immutable due to interior mutability
    pub fn exec(&self, f: fn(&RefCell<dyn Intersected>)) {
        for entity in self.entities.iter() {
            f(entity);
        }
    }
}


/// Hype plane defined with some point on it and normal vector
#[derive(Debug)]
pub struct HypePlane {
    go: GameObject,
    initpt: Point,
    normal: Matrix,
}

impl HypePlane {
    /// HypePlane constructor takes actual `GameObject`, `Point` on plane and normal vector
    pub fn new(go: GameObject, initpt: Point, mut normal: Matrix) -> ReRes<Self> {
        normal.ag_failed()?.ag_not_row_or_col()?;
        if initpt.dim() != normal.dim().unwrap() {
            return Err(MathErr(PtVecDimMismatch { pt_dim: initpt.dim(), vec_dim: normal.dim().unwrap() }));
        }
        if normal.repr() == Repr::Row {
            normal = normal.transpose();
        }
        Ok(Self { go, initpt, normal })
    }

    /// Dimension of space where plane lays
    pub fn dim(&self) -> usize {
        self.initpt.dim()
    }

    /// Rotate normal vector in plane stretched on `from` and `to` basis vectors on given `angle`
    pub fn planar_rotate(&mut self, from: usize, to: usize, angle: f64) -> ReRes<()> {
        self.normal = &self.normal * &Matrix::rotation(from, to, angle, self.dim());
        self.normal.ag_failed()?;
        Ok(())
    }

    /// Rotate normal vector in 3d space
    pub fn rotate_3d(&mut self, x: f64, y: f64, z: f64) -> ReRes<()> {
        self.normal = &self.normal * &Matrix::teit_bryan_rotation(x, y, z);
        self.normal.ag_failed()?;
        Ok(())
    }

    /// Move `initpt` on the given vector
    pub fn mv(&mut self, vec: &Matrix) -> ReRes<()> {
        self.initpt.mv_assign(vec)
    }
}

impl Entity for HypePlane {
    fn core(&self) -> &EntityCore {
        self.go.core()
    }

    fn core_mut(&mut self) -> &mut EntityCore {
        self.go.core_mut()
    }
}

impl Intersected for HypePlane {
    fn intersect(&self, inc: &Point, dir: &Matrix, len: f64) -> ReRes<f64> {
        let cs = Rc::clone(&self.go.core.cs);
        let denom = cs.space().scalar_prod(dir, &self.normal)?;
        if aeq(&denom, &0.0) {
            return Err(NoneOpt(NoIntersection));
        }
        let numer = cs.space().scalar_prod(&self.initpt.sub(inc)?, &self.normal)?;
        let dist = numer / denom;
        if dist < 0.0 {
            return Err(NoneOpt(NoIntersection));
        }
        Ok(dist * len)
    }
}
