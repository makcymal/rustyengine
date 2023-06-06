use {
    super::*,
    crate::{
        errs::{
            GameErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        math::*,
    },
    std::{
        any::{Any, TypeId},
        cell::RefCell,
        collections::hash_map::{Entry, HashMap},
        ops::{Index, IndexMut},
        rc::Rc,
    },
    uuid::Uuid,
};



/// Main trait for any entity instance requires returning UUID and map of properties
pub trait AsEntity {
    /// UUID of entity
    fn id(&self) -> &Rc<Uuid>;

    /// Ref to map of properties
    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>>;

    /// Mutable ref to map of properties
    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>>;

    /// Inserts new pair `key`: `val` into `props` field or replaces already existing
    fn set_prop(&mut self, key: &'static str, val: Box<dyn Any>) {
        match self.props_mut().entry(key) {
            Entry::Occupied(o) => *o.into_mut() = val,
            Entry::Vacant(v) => {
                let _ = v.insert(val);
            }
        };
    }

    /// Returns `ReRes` with ref to requested `Box<dyn Any>` instance or meaningful error if key doesn't exist
    fn get_prop(&self, key: &'static str) -> ReRes<&Box<dyn Any>> {
        if let Some(prop) = self.props().get(key) {
            Ok(prop)
        } else {
            Err(GameErr(NotInitializedProp))
        }
    }

    /// Performs deleting value by the given `Prop` key
    fn del_prop(&mut self, key: &'static str) {
        self.props_mut().remove(key);
    }
}

impl std::fmt::Debug for dyn AsEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID {:?}", self.id())
    }
}

impl Index<&'static str> for dyn AsEntity {
    type Output = Box<dyn Any>;

    fn index(&self, key: &'static str) -> &Self::Output {
        &self.props()[key]
    }
}


pub trait AsEntityList {
    fn new() -> Self;

    fn append(&mut self, entity: impl AsEntity + 'static);

    fn remove(&mut self, id: &Rc<Uuid>);

    fn iter(&self) -> dyn Iterator<Item=Rc<RefCell<dyn AsEntity>>>;

    fn get(&self, id: &Rc<Uuid>) -> Option<Rc<RefCell<dyn AsEntity>>> {
        if let Some(rc) =
            self.iter().find(|entity| Rc::ptr_eq(entity.borrow().id(), id))
        {
            Some(rc.clone())
        } else {
            None
        }
    }

    fn exec(&self, f: fn(Rc<RefCell<dyn AsEntity>>)) {
        for rc in self.iter() {
            f(rc.clone())
        }
    }
}


///
pub trait AsIntersected: AsEntity {
    fn intersect(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64;
}


///
pub trait AsIntersectedList: AsEntityList {
    fn intersect(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64;
}


///
pub trait AsGameObject: AsIntersected {
    fn pos(&self) -> &Point;

    fn pos_mut(&mut self) -> &mut Point;

    fn dir(&self) -> &Vector;

    fn dir_mut(&mut self) -> &mut Vector;

    fn mv(&mut self, vec: &Vector) -> ReRes<()> {
        self.pos_mut().mv_assign(vec)
}

    fn df(&self, pt: &Point) -> ReRes<Vector> {
        self.pos().df(pt)
    }

    fn rotate_3d(&mut self, x: f64, y: f64, z: f64) -> ReRes<()> {
        self.dir_mut().coord = self.dir().coord.mul(&Matrix::teit_bryan_rotation(x, y, z));
        self.dir().coord.ag_failed()?;
        Ok(())
    }

    fn planar_rotate(&mut self, from: usize, to: usize, angle: f64) -> ReRes<()> {
        self.dir_mut().coord = Matrix::rotation(from, to, angle, 3)
            .mul(self.dir().coord())
            .to_col();
        self.dir().coord.ag_failed()?;
        Ok(())
    }

    /// Dimension of space where `GameObject` lays
    fn dim(&self) -> usize {
        self.pos().dim()
    }
}

impl std::fmt::Debug for dyn AsGameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UUID {:?}\n Position {:?}\n Direction {:?}",
            self.id(),
            self.pos(),
            self.dir()
        )
    }
}
