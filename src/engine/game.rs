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


/// Struct responsible for storing current CoordSys and EntityList and running related scripts
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    cs: Rc<CoordSys>,
    id_set: IdSet,
    entities: EntityList,
}

impl Game {
    /// Constructor that takes CoordSys
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

    /// `Ray` in current basis, takes inception `Point` and direction `Vector`
    pub fn game_ray(&self, inc: Point, dir: Vector) -> Ray {
        Ray::from(&self.cs, inc, dir)
    }

    /// `EntityCore` in current basis with appending it's `Uuid` into `IdSet`
    /// Intended to call it from specific entity constructors
    pub(in super) fn entity_core(&mut self) -> EntityCore {
        EntityCore::new(&self.cs, &self.id_set.generate())
    }


}
