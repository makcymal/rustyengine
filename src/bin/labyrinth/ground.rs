use rustyengine::{engn::*, math::*};

#[derive(Debug)]
pub struct Ground {}

impl Ground {
    pub fn new() -> Self {
        Self {}
    }
}

impl AsCollided for Ground {
    fn collide(&self, inc: &Point, dir: &Vector) -> Option<f32> {
        if aeq(dir[2], 0.0) {
            return None;
        }
        validate_collision(-inc[2] / dir[2])
    }

    fn charmap(&self, _dist: f32) -> Option<char> {
        None
    }
}
