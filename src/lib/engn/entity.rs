use {
    uuid::Uuid,
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
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GridErr::{self, *},
            MatrixErr::{self, *},
            CoordSysErr::{self, *},
        },
        math::{
            Matrix, Point, CoordSys,
        },
    },
};


/// Matrix of `Uuid` (standard v4) allocated in heap
#[derive(Debug, Clone, PartialEq)]
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
}

impl Index<usize> for IdPool {
    type Output = Uuid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ids[index]
    }
}


pub trait Property {
    fn feed(&self) -> (TypeId, isize);
}

/// Properties that are available to be set within `EntityCore.props`
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    /// Position in current `CoordSys`, intended to be `Point`
    Pos,
    /// Direction of view, intended to be `Matrix`
    Dir,
    /// Field-of-view, intended to be `Float`
    Fov,
    /// Vertical field-of-view, intended to be `Float`
    VFov,
    /// `Point` of view for `GameCamera`
    LookAt,
    /// Drawing distance of `GameCamera`
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
    cs: Rc<CoordSys>,
    /// Cloned `Rc` from `IdPool` within actual `Game` instance
    id: Rc<Uuid>,
    /// Dictionary with `(TypeId, isize)` as key and `Any` trait object wrapped into `Box` as value
    props: HashMap<(TypeId, isize), Box<dyn Any>>,
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

    /// Inserts new pair `prop_name`: `prop_val` into `props` field or replaces already existing
    pub fn set_prop(&mut self, key: Prop, val: Box<dyn Any>) {
        match self.props.entry(key.feed()) {
            Entry::Occupied(o) => *o.into_mut() = val,
            Entry::Vacant(v) => {
                let _ = v.insert(val);
            }
        };
    }

    /// Returns `Some` with ref to requested `AnyVal` instance or `None` if key doesn't exist
    pub fn get_prop(&self, key: Prop) -> Option<&Box<dyn Any>> {
        self.props.get(&key.feed())
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


#[derive(Debug)]
pub struct EntityList {
    entities: Vec<Rc<RefCell<dyn Entity>>>,
}

impl EntityList {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn append(&mut self, entity: impl Entity + 'static) {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    pub fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities.retain(|entity| Rc::ptr_eq(&entity.borrow().core().id, id));
    }

    pub fn get(&self, id: &Rc<Uuid>) -> Option<Rc<RefCell<dyn Entity>>> {
        if let Some(rc) = self.entities.iter().find(|entity| Rc::ptr_eq(&entity.borrow().core().id, id)) {
            Some(Rc::clone(rc))
        } else {
            None
        }
    }

    pub fn exec(&self, f: fn(&RefCell<dyn Entity>)) {
        for entity in self.entities.iter() {
            f(entity);
        }
    }
}


/// Basic game object
#[derive(Debug)]
pub struct GameObject {
    core: EntityCore,
}

impl GameObject {
    /// Constructor that takes `EntityCore`, position, direction, and then sets
    /// such properties to the given core
    pub(in super) fn new(mut core: EntityCore, pos: Point, dir: Matrix) -> Self {
        core.set_prop(Prop::Pos, Box::new(pos));
        core.set_prop(Prop::Dir, Box::new(dir));
        Self { core }
    }

    pub fn shift(&mut self, dir: &Matrix) -> ReRes<()> {
        match self.core.props.get_mut(&Prop::Pos.feed()) {
            Some(val) => val.downcast_mut::<Point>().unwrap().mv(dir),
            None => unreachable!(),
        }
    }

    pub fn set_pos(&mut self, pos: Point) {
        self.core.set_prop(Prop::Pos, Box::new(pos));
    }

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



#[derive(Debug)]
pub struct GameCamera {
    game_object: GameObject,
}

impl GameCamera {
    pub fn new(mut game_object: GameObject) -> Self {
        Self { game_object }
    }

    pub fn fov(mut self, fov: f64) -> Self {
        self.game_object.core.set_prop(Prop::Fov, Box::new(fov));
        self
    }

    pub fn vfov(mut self, vfov: f64) -> Self {
        self.game_object.core.set_prop(Prop::VFov, Box::new(vfov));
        self
    }

    pub fn draw_dist(mut self, draw_dist: f64) -> Self {
        self.game_object.core.set_prop(Prop::DrawDist, Box::new(draw_dist));
        self
    }

    pub fn lookat(mut self, lookat: f64) -> Self {
        self.game_object.core.set_prop(Prop::LookAt, Box::new(lookat));
        self
    }
}

impl Entity for GameCamera {
    fn core(&self) -> &EntityCore {
        &self.game_object.core
    }

    fn core_mut(&mut self) -> &mut EntityCore {
        &mut self.game_object.core
    }
}
