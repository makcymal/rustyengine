use {
    uuid::Uuid,
    std::{
        collections::hash_map::{
            HashMap, Entry,
        },
        rc::Rc,
        ops::{
            Index, IndexMut,
        },
    },
    crate::{
        errs::{
            EntityErr,
        },
        linal::{
            Point, Vector, CoordSys,
        },
        utils::AnyVal,
    },
};


// <<< IdSet

/// Vector of `Uuid` (standard v4) allocated in heap
#[derive(Debug, Clone, PartialEq)]
pub struct IdSet {
    id_set: Vec<Rc<Uuid>>,
}

impl IdSet {
    /// Empty constructor
    pub fn empty() -> Self {
        Self { id_set: vec![] }
    }

    /// Method scoped in `engine` namespace, generates `Uuid` of v4
    pub(in super) fn generate(&mut self) -> Rc<Uuid> {
        self.id_set.push(Rc::new(Uuid::new_v4()));
        Rc::clone(self.id_set.last().unwrap())
    }
}

impl Index<usize> for IdSet {
    type Output = Uuid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.id_set[index]
    }
}

// IdSet >>>


// <<< Entity

/// Generic trait for any entity instance requires returning
/// ref to `EntityCore` that should exist by principle Composition Over Inheritance
pub trait Entitify {
    /// Ref to the `EntityCore`
    fn core(&self) -> &EntityCore;
}

/// Available species of entities
/// Create it with trivially calling `Entity::<required entity>(<entity instance>)`
/// It's main purpose is providing polymorphism
#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    /// Just `EntityCore`
    Empty(EntityCore),
    /// `GameObject` that stands for basic game object
    GameObject(GameObject),
    /// Obviously it's camera
    GameCamera(GameCamera),
}

impl Entitify for Entity {
    fn core(&self) -> &EntityCore {
        match self {
            Entity::Empty(entity_core) => &entity_core,
            Entity::GameObject(game_object) => &game_object.core,
            Entity::GameCamera(game_camera) => &game_camera.game_object.core
        }
    }
}

// Entity >>>


// <<< EntityCore

/// Struct responsible for operations that are typical for entities
#[derive(Debug, Clone, PartialEq)]
pub struct EntityCore {
    /// Cloned `Rc` from actual `Game` instance
    cs: Rc<CoordSys>,
    /// Cloned `Rc` from `IdSet` within actual `Game` instance
    id: Rc<Uuid>,
    /// Dictionary with `Prop` enum as key and `AnyVal` enum as value
    props: HashMap<Prop, AnyVal>,
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
    pub fn set_prop(&mut self, prop_name: Prop, prop_val: AnyVal) {
        match self.props.entry(prop_name) {
            Entry::Occupied(o) => *o.into_mut() = prop_val,
            Entry::Vacant(v) => {
                let _ = v.insert(prop_val);
            }
        };
    }

    /// Returns `Some` with ref to requested `AnyVal` instance or `None` if key doesn't exist
    pub fn get_prop(&self, prop_name: Prop) -> Option<&AnyVal> {
        self.props.get(&prop_name)
    }

    /// Performs deleting value by the given `Prop` key
    pub fn del_prop(&mut self, prop_name: Prop) {
        self.props.remove(&prop_name);
    }
}

impl Index<Prop> for EntityCore {
    type Output = AnyVal;

    fn index(&self, prop_name: Prop) -> &Self::Output {
        &self.props[&prop_name]
    }
}

/// Properties that are available to be set within `EntityCore.props`
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    /// Position in current `CoordSys`, intended to be `Point`
    Pos,
    /// Direction of view, intended to be `Vector`
    Dir,
    /// Field-of-view, intended to be `Float`
    Fov,
    /// Vertical field-of-view, intended to be `Float`
    VFov,
    /// `Point` of view for `GameCamera`
    LooksAt,
    /// Drawing distance of `GameCamera`, intended to be `Float`
    DrawDist,
}

// EntityCore >>>


// <<< GameObject

/// Basic game object
#[derive(Debug, Clone, PartialEq)]
pub struct GameObject {
    core: EntityCore,
}

impl GameObject {
    /// Constructor that takes `EntityCore`, position, direction, and then sets
    /// such properties to the given core
    pub(in super) fn new(mut core: EntityCore, pos: Point, dir: Vector) -> Self {
        core.set_prop(Prop::Pos, AnyVal::Point(pos));
        core.set_prop(Prop::Dir, AnyVal::Vector(dir));
        Self { core }
    }

    pub fn shift(&mut self, dir: &Vector) {
        match self.core.props.get_mut(&Prop::Pos) {
            Some(prop_val) => match prop_val {
                AnyVal::Point(point) => *point = &*point + dir,
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }

    pub fn set_pos(&mut self, pos: Point) {
        self.core.set_prop(Prop::Pos, AnyVal::Point(pos));
    }

    pub fn set_dir(&mut self, dir: Vector) {
        self.core.set_prop(Prop::Dir, AnyVal::Vector(dir));
    }
}

// GameObject >>>


// <<< GameCamera

#[derive(Debug, Clone, PartialEq)]
pub struct GameCamera {
    game_object: GameObject,
}

impl GameCamera {
    pub fn new(mut game_object: GameObject, fov: f64, draw_dist: f64) -> Self {
        game_object.core.set_prop(Prop::Fov, AnyVal::Float(fov));
        game_object.core.set_prop(Prop::DrawDist, AnyVal::Float(draw_dist));
        Self { game_object }
    }

    pub fn new_with_vfov(mut game_object: GameObject, fov: f64, vfov: f64, draw_dist: f64) -> Self {
        game_object.core.set_prop(Prop::Fov, AnyVal::Float(fov));
        game_object.core.set_prop(Prop::VFov, AnyVal::Float(vfov));
        game_object.core.set_prop(Prop::DrawDist, AnyVal::Float(draw_dist));
        Self { game_object }
    }

    pub fn new_with_lookat(mut game_object: GameObject, fov: f64, lookat: Point, draw_dist: f64) -> Self {
        game_object.core.set_prop(Prop::Fov, AnyVal::Float(fov));
        game_object.core.set_prop(Prop::LooksAt, AnyVal::Point(lookat));
        game_object.core.set_prop(Prop::DrawDist, AnyVal::Float(draw_dist));
        Self { game_object }
    }

    pub fn new_with_vfov_lookat(mut game_object: GameObject, fov: f64, vfov: f64, lookat: Point, draw_dist: f64) -> Self {
        game_object.core.set_prop(Prop::Fov, AnyVal::Float(fov));
        game_object.core.set_prop(Prop::VFov, AnyVal::Float(vfov));
        game_object.core.set_prop(Prop::LooksAt, AnyVal::Point(lookat));
        game_object.core.set_prop(Prop::DrawDist, AnyVal::Float(draw_dist));
        Self { game_object }
    }
}

// GameCamera >>>


// <<< EntityList

#[derive(Debug, Clone, PartialEq)]
pub struct EntityList {
    entities: Vec<Entity>,
}

impl EntityList {
    pub fn empty() -> Self {
        Self { entities: vec![] }
    }

    pub fn from(entities: Vec<Entity>) -> Self {
        Self { entities }
    }

    pub fn append(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove(&mut self, id: &Rc<Uuid>) {
        self.entities.retain(|entity| Rc::ptr_eq(&entity.core().id, id));
    }

    pub fn get(&self, id: &Rc<Uuid>) -> Option<&Entity> {
        self.entities.iter().find(|entity| Rc::ptr_eq(&entity.core().id, id))
    }

    pub fn get_mut(&mut self, id: &Rc<Uuid>) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|entity| Rc::ptr_eq(&entity.core().id, id))
    }

    pub fn exec(&mut self, f: fn(&mut Entity)) {
        for entity in self.entities.iter_mut() {
            f(entity);
        }
    }
}

impl Index<&Rc<Uuid>> for EntityList {
    type Output = Entity;

    fn index(&self, id: &Rc<Uuid>) -> &Self::Output {
        self.get(id).expect("Index within EntityList with inexistance index")
    }
}

// EntityList >>>
