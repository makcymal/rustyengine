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
pub trait AsCollided {
    fn collide(&self, inc: &Point, dir: &Vector) -> f32;
}

impl std::fmt::Debug for dyn AsCollided {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Collided trait object")
    }
}


/// For material that has not-consistent position and direction in the game
pub trait AsGameObject: AsCollided {
    fn pos(&self) -> &Point;

    fn pos_mut(&mut self) -> &mut Point;

    fn dir(&self) -> &Vector;

    fn dir_mut(&mut self) -> &mut Vector;

    fn mv(&mut self, vec: &Vector) {
        self.pos_mut().mv(vec)
    }

    fn df(&self, pt: &Point) -> Vector {
        self.pos().df(pt)
    }

    fn planar_rotate(&mut self, from: usize, to: usize, angle: f32) {
        *self.dir_mut() = &Matrix::rotation(from, to, angle) * self.dir()
    }

    fn rotate_3d(&mut self, xy: f32, yz: f32, xz: f32) {
        self.planar_rotate(0, 1, xy);
        self.planar_rotate(1, 2, yz);
        self.planar_rotate(0, 2, xz);
    }
}

impl std::fmt::Debug for dyn AsGameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position {:?}\n Direction {:?}",
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

    fn exec(&self, f: fn(&Self::Item));

    /// Returns ref to `Self::Item` if requested material exists
    fn get(&self, id: &Rc<Uuid>) -> Option<&Self::Item>;
}


pub trait AsScene {
    /// Computes minimal distance to entities
    fn collide(&self, inc: &Point, dir: &Vector) -> f32;
}
