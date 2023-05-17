use {
    super::{
        Ray, IdPool, EntityCore, EntityList, GameObject, Camera,
    },
    crate::math::{
        Point, Matrix, CoordSys
    },
    std::{
        rc::Rc,
    },
    crate::errs::{
        ReRes,
        ReErr::{self, *},
    },
};


/// Struct responsible for storing current CoordSys and EntityList and running related scripts
#[derive(Debug)]
pub struct Game {
    cs: Rc<CoordSys>,
    id_pool: IdPool,
    entities: EntityList,
}

impl Game {
    /// Constructor that takes CoordSys
    pub fn new(cs: CoordSys) -> Self {
        Self {
            cs: Rc::new(cs),
            id_pool: IdPool::new(),
            entities: EntityList::new(),
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

    /// `Ray` in current basis, takes inception `Point` and direction `Vector`
    pub fn game_ray(&self, inc: Point, dir: Matrix) -> ReRes<Ray> {
        Ray::new(&self.cs, inc, dir)
    }

    /// `EntityCore` in current basis with appending it's `Uuid` into `IdPool`
    pub fn entity_core(&mut self) -> EntityCore {
        EntityCore::new(&self.cs, &self.id_pool.generate())
    }

    /// `GameObject` in current game, calls `self.entity_core()`
    pub fn game_object(&mut self, pos: Point, dir: Matrix) -> GameObject {
        GameObject::new(self.entity_core(), pos, dir)
    }
}
