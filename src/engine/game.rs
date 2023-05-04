use {
    super::{
        Ray, IdSet, EntityCore, EntityList, GameObject, GameCamera,
    },
    crate::linal::{
        Point, Vector, CoordSys
    },
    std::{
        rc::Rc,
    }
};


pub struct Game {
    cs: Rc<CoordSys>,
    id_set: IdSet,
    entities: EntityList,
}

impl Game {
    pub fn new(cs: CoordSys) -> Self {
        Self {
            cs: Rc::new(cs),
            id_set: IdSet::empty(),
            entities: EntityList::from(vec![]),
        }
    }

    pub fn run() {
        todo!()
    }

    pub fn update() {
        todo!()
    }

    pub fn exit() {
        todo!()
    }

    pub fn game_ray(&self, inc: Point, dir: Vector) -> Ray {
        Ray::from(&self.cs, inc, dir)
    }

    pub fn entity_core(&mut self) -> EntityCore {
        EntityCore::new(&self.cs, &self.id_set.generate())
    }

    pub fn game_object(&mut self, pos: Point, dir: Vector) -> GameObject {
        GameObject::new(self.entity_core(), pos, dir)
    }

    pub fn game_camera(&mut self) -> GameCamera {
        todo!()
    }
}
