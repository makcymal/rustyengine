use {
    crate::{
        engn::*,
        errs::{
            GameErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        grid::*,
        math::*,
    },
    either::Either,
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

/// For material that can be indexed inside the `Game` instance with `Uuid` and can store properties within `HashMap`
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

/// for material that can be collided with `Ray`. Coefficient of `Ray` resizing is returned if collision exists else `-1.0`
pub trait AsCollided: AsEntity {
    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> Option<f64>;

    fn charmap(&self, dist: f64) -> Option<char>;
}

impl std::fmt::Debug for dyn AsCollided {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID {:?}", self.id())
    }
}

pub fn validate_collision(dist: f64) -> Option<f64> {
    if dist < 0.0 {
        None
    } else {
        Some(dist)
    }
}

/// For material that has not-consistent position and direction in the game
pub trait AsGameObject: AsCollided {
    fn pos(&self) -> &Point;

    fn pos_mut(&mut self) -> &mut Point;

    fn dir(&self) -> &Matrix;

    fn dir_mut(&mut self) -> &mut Matrix;

    fn mv(&mut self, vec: &Vector) -> ReRes<()> {
        self.pos_mut().mv_assign(vec)
    }

    fn df(&self, pt: &Point) -> ReRes<Vector> {
        self.pos().df(pt)
    }

    fn rotate_3d(&mut self, x: f64, y: f64, z: f64) -> ReRes<()> {
        *self.dir_mut() = self.dir().mul(&Matrix::teit_bryan_rotation(x, y, z));
        self.dir().ag_failed()?;
        Ok(())
    }

    fn planar_rotate(&mut self, from: usize, to: usize, angle: f64) -> ReRes<()> {
        *self.dir_mut() = Matrix::rotation(from, to, angle, 3)
            .mul(self.dir())
            .to_col();
        self.dir().ag_failed()?;
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

pub trait AsEntityList {
    /// Wrapper around dyn AsCollided, eg Box<dyn AsCollided> or Rc<RefCell<dyn AsCollided>>
    type Item;

    /// Appends given item to the current list
    fn append(&mut self, item: Self::Item);

    /// Removes item with given id from the current list
    fn remove(&mut self, id: &Rc<Uuid>);

    /// Returns ref to `Self::Item` if requested material exists
    fn get(&self, id: &Rc<Uuid>) -> Option<&Self::Item>;

    /// Permorms closure to some subset of all the entities
    fn exec(&self, f: fn(&Self::Item));
}

pub trait AsScene {
    /// Computes minimal distance to entities
    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> Either<f64, char>;

    fn validate_mv(&self, cs: &CoordSys, pos: &Point, mv: &mut Vector);
}
