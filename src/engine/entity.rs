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

#[derive(Debug, Clone, PartialEq)]
pub struct IdSet {
    id_set: Vec<Rc<Uuid>>,
}

impl IdSet {
    pub fn empty() -> Self {
        Self { id_set: vec![] }
    }

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

pub trait Entitify {
    fn core(&self) -> &EntityCore;
}

pub enum Entity {
    Empty(EntityCore),
    GameObject(GameObject),
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

#[derive(Debug, Clone, PartialEq)]
pub struct EntityCore {
    cs: Rc<CoordSys>,
    id: Rc<Uuid>,
    props: HashMap<Prop, AnyVal>,
}

impl EntityCore {
    pub fn new(cs: &Rc<CoordSys>, id: &Rc<Uuid>) -> Self {
        Self {
            cs: Rc::clone(cs),
            id: Rc::clone(id),
            props: HashMap::new(),
        }
    }

    pub fn set_prop(&mut self, prop_name: Prop, prop_val: AnyVal) {
        match self.props.entry(prop_name) {
            Entry::Occupied(o) => *o.into_mut() = prop_val,
            Entry::Vacant(v) => {
                let _ = v.insert(prop_val);
            }
        };
    }

    pub fn get_prop(&self, prop_name: Prop) -> Option<&AnyVal> {
        self.props.get(&prop_name)
    }

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    Pos,
    Dir,
    Fov,
    VFov,
    LooksAt,
    DrawDist,
}

// EntityCore >>>


// <<< GameObject

pub struct GameObject {
    core: EntityCore,
}

impl GameObject {
    pub fn new(mut core: EntityCore, pos: Point, dir: Vector) -> Self {
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
