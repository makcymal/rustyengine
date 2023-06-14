use {
    rustyengine::{conf::*, engn::*, math::*},
    std::{collections::HashMap, rc::Rc},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Ground {
    entity: Entity,
}

impl Ground {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl AsEntity for Ground {
    fn id(&self) -> &Rc<Uuid> {
        self.entity.id()
    }

    fn props(&self) -> &HashMap<PropKey, PropVal> {
        self.entity.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<PropKey, PropVal> {
        self.entity.props_mut()
    }
}

impl AsCollided for Ground {
    fn collide(&self, _cs: &CoordSys, inc: &Point, dir: &Vector) -> Option<f64> {
        if aeq(&dir[2], &0.0) {
            None
        } else {
            validate_collision(-inc[2] / dir[2])
        }
    }

    fn charmap(&self, _dist: f64) -> Option<char> {
        None
    }
}
