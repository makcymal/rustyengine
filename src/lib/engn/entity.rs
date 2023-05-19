use {
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GameErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
        },
        math::{
            Matrix, Point, CoordSys,
        },
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
    pub(in super) fn generate(&mut self) -> Rc<Uuid> {
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


/// Trait that shoul implement custom property in purpose of having possibilty being saved in `EntityCore.props`
pub trait Property {
    /// 'Feed' property as key for `EntityCore.props` that is `HashMap`
    /// Serializes property to `(TypeId, isize)` that is `Hash`
    fn feed(&self) -> (TypeId, isize);
}

/// Properties that are available to be set within `EntityCore.props`
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    /// Position, `Point`
    Pos,
    /// Direction of view, `Matrix`
    Dir,
    /// Roll of entity, `Matrix`
    Roll,
    /// Field-of-view, `f64`
    Fov,
    /// Vertical field-of-view, `f64`
    VFov,
    /// `Point` of view for `GameCamera`
    LookAt,
    /// Drawing distance of `GameCamera`, 'f64'
    DrawDist,
}

impl Property for Prop {
    fn feed(&self) -> (TypeId, isize) {
        (self.type_id(), *self as isize)
    }
}


/// Struct responsible for operations that are typical for entities
#[derive(Debug)]
pub struct EntityCore {
    /// Cloned `Rc` from actual `Game` instance
    pub(in super) cs: Rc<CoordSys>,
    /// Cloned `Rc` from `IdPool` within actual `Game` instance
    pub(in super) id: Rc<Uuid>,
    /// Dictionary with `(TypeId, isize)` as key and `Any` trait object wrapped into `Box` as value
    pub(in super) props: HashMap<(TypeId, isize), Box<dyn Any>>,
}

impl EntityCore {
    /// Basic constructor that intended to be called from `Game` instance
    pub fn new(cs: &Rc<CoordSys>, id: &Rc<Uuid>) -> Self {
        Self {
            cs: Rc::clone(cs),
            id: Rc::clone(id),
            props: HashMap::new(),
        }
    }

    /// Inserts new pair `key`: `val` into `props` field or replaces already existing
    pub fn set_prop(&mut self, key: Prop, val: Box<dyn Any>) {
        match self.props.entry(key.feed()) {
            Entry::Occupied(o) => *o.into_mut() = val,
            Entry::Vacant(v) => {
                let _ = v.insert(val);
            }
        };
    }

    /// Returns `ReRes` with ref to requested `Box<dyn Any>` instance or meaningful error  if key doesn't exist
    pub fn get_prop(&self, key: Prop) -> ReRes<&Box<dyn Any>> {
        if let Some(prop) = self.props.get(&key.feed()) {
            Ok(prop)
        } else {
            Err(GameErr(NotInitializedProp))
        }
    }

    /// Performs deleting value by the given `Prop` key
    pub fn del_prop(&mut self, key: Prop) {
        self.props.remove(&key.feed());
    }
}

impl Index<Prop> for EntityCore {
    type Output = Box<dyn Any>;

    fn index(&self, key: Prop) -> &Self::Output {
        &self.props[&key.feed()]
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
    pub(in super) core: EntityCore,
}

impl GameObject {
    /// Constructor that takes `EntityCore`, position, direction, and then glob
    /// such properties to the given core
    pub(in super) fn new(mut core: EntityCore, pos: Point, dir: Matrix) -> Self {
        core.set_prop(Prop::Pos, Box::new(pos));
        core.set_prop(Prop::Dir, Box::new(dir));
        Self { core }
    }

    /// Moves game object on the given vector
    pub fn mv(&mut self, vec: &Matrix) -> ReRes<()> {
        match self.core.props.get_mut(&Prop::Pos.feed()) {
            Some(val) => val.downcast_mut::<Point>().unwrap().mv_assign(vec),
            None => unreachable!(),
        }
    }

    /// Set property `Pos` of game object
    pub fn set_pos(&mut self, pos: Point) {
        self.core.set_prop(Prop::Pos, Box::new(pos));
    }

    /// Set property `Dir` of game object
    pub fn set_dir(&mut self, dir: Matrix) {
        self.core.set_prop(Prop::Dir, Box::new(dir));
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
