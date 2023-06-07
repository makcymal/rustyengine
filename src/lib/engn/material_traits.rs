use {
    super::*,
    crate::{
        errs::{
            GameErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        math::*,
    },
    std::{
        any::{Any, TypeId},
        cell::RefCell,
        collections::hash_map::{Entry, HashMap},
        ops::Index,
        rc::Rc,
    },
    uuid::Uuid,
};


pub type PropKey = &'static str;
pub type PropVal = Box<dyn Any>;


/// Main trait for any entity instance requires returning UUID and map of properties
pub trait AsEntity {
    /// UUID of entity
    fn id(&self) -> &Rc<Uuid>;

    /// Ref to map of properties
    fn props(&self) -> &HashMap<PropKey, PropVal>;

    /// Mutable ref to map of properties
    fn props_mut(&mut self) -> &mut HashMap<PropKey, PropVal>;

    /// Inserts new pair `key`: `val` into `props` field or replaces already existing
    fn set_prop(&mut self, key: PropKey, val: PropVal) {
        match self.props_mut().entry(key) {
            Entry::Occupied(o) => *o.into_mut() = val,
            Entry::Vacant(v) => {
                let _ = v.insert(val);
            }
        };
    }

    /// Returns `ReRes` with ref to requested `Box<dyn Any>` instance or meaningful error if key doesn't exist
    fn get_prop(&self, key: PropKey) -> ReRes<&PropVal> {
        if let Some(prop) = self.props().get(key) {
            Ok(prop)
        } else {
            Err(GameErr(NotInitializedProp))
        }
    }

    /// Performs deleting value by the given `Prop` key
    fn del_prop(&mut self, key: PropKey) {
        self.props_mut().remove(key);
    }
}

impl std::fmt::Debug for dyn AsEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID {:?}", self.id())
    }
}

impl Index<PropKey> for dyn AsEntity {
    type Output = PropVal;

    fn index(&self, key: PropKey) -> &Self::Output {
        &self.props()[key]
    }
}


///
pub trait AsEntityList {
    /// Wrapper around dyn AsEntity, eg Box<dyn AsEntity> or Rc<RefCell<dyn AsEntity>>
    type Item;

    fn new() -> Self;

    fn append(&mut self, item: Self::Item);

    fn remove(&mut self, id: &Rc<Uuid>);

    fn iter(&self) -> Box<dyn Iterator<Item=&Self::Item> + '_>;

    /// Returns shared interior mutable ref to entity if exists
    fn get(&self, id: &Rc<Uuid>) -> Option<&Self::Item>;

    /// Permorms closure that may be immutable due to interior mutability
    fn exec(&self, f: fn(&Self::Item)) {
        for rc in self.iter() {
            f(&rc)
        }
    }
}


///
pub trait AsCollided: AsEntity {
    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64;
}

impl std::fmt::Debug for dyn AsCollided {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID {:?}", self.id())
    }
}


///
pub trait AsCollidedList: AsEntityList {
    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64;
}


///
pub trait AsGameObject: AsEntity {
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

