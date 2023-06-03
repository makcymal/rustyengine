use {
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GameErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
        },
        math::*,
    },
    std::{
        collections::hash_map::{
            HashMap, Entry,
        },
        rc::Rc,
        cell::RefCell,
        ops::{
            Index, IndexMut,
        },
        any::{
            Any,
            TypeId
        },
    },
    uuid::Uuid,
};


/// Matrix of `Uuid` (standard v4) allocated in heap
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IdPool {
    ids: Vec<Rc<Uuid>>,
}

impl IdPool {
    /// Empty constructor
    pub fn new() -> Self {
        Self { ids: vec![] }
    }

    /// Method scoped in `engine` namespace, generates `Uuid` of v4
    pub fn generate(&mut self) -> Rc<Uuid> {
        self.ids.push(Rc::new(Uuid::new_v4()));
        Rc::clone(self.ids.last().unwrap())
    }

    pub fn len(&self) -> usize { self.ids.len() }
}

impl Index<usize> for IdPool {
    type Output = Uuid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ids[index]
    }
}


/// Struct responsible for operations that are typical for entities
#[derive(Debug)]
pub struct EntityCore {
    /// Cloned `Rc` from `IdPool` within actual `Game` instance
    pub(in super) id: Rc<Uuid>,
    /// Dictionary with `(TypeId, isize)` as key and `Any` trait object wrapped into `Box` as value
    pub(in super) props: HashMap<&'static str, Box<dyn Any>>,
}

impl EntityCore {
    /// Basic constructor that intended to be called from `Game` instance
    pub fn new(id: &Rc<Uuid>) -> Self {
        Self {
            id: Rc::clone(id),
            props: HashMap::new(),
        }
    }

    /// Inserts new pair `key`: `val` into `props` field or replaces already existing
    pub fn set_prop(&mut self, key: &'static str, val: Box<dyn Any>) {
        match self.props.entry(key) {
            Entry::Occupied(o) => *o.into_mut() = val,
            Entry::Vacant(v) => {
                let _ = v.insert(val);
            }
        };
    }

    /// Returns `ReRes` with ref to requested `Box<dyn Any>` instance or meaningful error  if key doesn't exist
    pub fn get_prop(&self, key: &'static str) -> ReRes<&Box<dyn Any>> {
        if let Some(prop) = self.props.get(key) {
            Ok(prop)
        } else {
            Err(GameErr(NotInitializedProp))
        }
    }

    /// Performs deleting value by the given `Prop` key
    pub fn del_prop(&mut self, key: &'static str) {
        self.props.remove(key);
    }
}

impl Index<&'static str> for EntityCore {
    type Output = Box<dyn Any>;

    fn index(&self, key: &'static str) -> &Self::Output {
        &self.props[key]
    }
}


/// Generic trait for any entity instance requires returning
/// ref to `EntityCore` that should exist by principle Composition Over Inheritance
pub trait Entity {
    /// Ref to the `EntityCore`
    fn core(&self) -> &EntityCore;

    /// Mut ref to the `EntityCore`
    fn core_mut(&mut self) -> &mut EntityCore;
}

impl std::fmt::Debug for dyn Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Core {:?}", self.core())
    }
}


/// Basic game object
#[derive(Debug)]
pub struct GameObject {
    pub core: EntityCore,
    pub pos: Point,
    pub dir: Vector,
}

impl GameObject {
    /// Constructor that takes `EntityCore`, position, direction, and then glob
    /// such properties to the given core
    pub(in super) fn new(mut core: EntityCore, pos: Point, dir: Vector) -> Self {
        Self { core, pos, dir }
    }

    /// Moves game object on the given vector
    pub fn mv(&mut self, vec: &Vector) -> ReRes<()> {
        Ok(self.pos.mv_assign(vec)?)
    }
}

impl Entity for GameObject {
    fn core(&self) -> &EntityCore {
        &self.core
    }

    fn core_mut(&mut self) -> &mut EntityCore {
        &mut self.core
    }
}
