use {
    rustyengine::{
        engn::*,
        math::*,
    },
};

#[derive(Debug)]
pub struct Ground {}

impl Ground {
    pub fn new() -> Self {
        Self {}
    }
}


impl AsCollided for Ground {
    fn collide(&self, inc: &Point, dir: &Vector) -> f32 {
        if aeq(dir[2], 0.0) {
            return -1.0
        }
        -inc.at(2) / dir.at(2)
    }
}
