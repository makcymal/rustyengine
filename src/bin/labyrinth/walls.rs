use {
    rustyengine::{engn::*, math::*},
    std::{any::Any, cmp::Ordering, collections::HashMap, rc::Rc},
    uuid::Uuid,
};
use rustyengine::engn::material_traits::validate_collision;

/// Height of all panes
const H: f64 = 5.0;

/// Part of the entire plane that is right rectangle and collinear to Oxz plane.
/// Points on it is defined as `(x, y0, z): x1 <= x < x2, 0 <= z <= H`
#[derive(Debug)]
pub struct XzWalls {
    pub entity: Entity,
    pub y0: f64,
    pub x_seg: Vec<Float>,
}

impl XzWalls {
    pub fn new(entity: Entity, y0: f64, x_seg: Vec<f64>) -> Self {
        Self {
            entity,
            y0,
            x_seg: x_seg.iter().map(|f| Float(*f)).collect(),
        }
    }
}

impl AsEntity for XzWalls {
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

impl AsCollided for XzWalls {
    fn collide(&self, _cs: &CoordSys, inc: &Point, dir: &Vector) -> Option<f64> {
        if aeq(&dir[1], &0.0) || self.x_seg.is_empty() {
            return None;
        }

        let t = (self.y0 - inc[1]) / dir[1];

        let z = inc[2] + t * dir[2];
        if z < 0.0 || H < z {
            return None;
        }

        let x = Float(inc[0] + t * dir[0]);
        let idx = match self.x_seg.binary_search(&x) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if idx == 0 {
            match aeq(&x.0, &self.x_seg[0].0) {
                true => validate_collision(t),
                false => None,
            }
        } else if idx == self.x_seg.len() {
            match aeq(&x.0, &self.x_seg.last().unwrap().0) {
                true => validate_collision(t),
                false => None,
            }
        } else if idx % 2 == 1 || aeq(&x.0, &self.x_seg[idx - 1].0) || aeq(&x.0, &self.x_seg[idx].0) {
            validate_collision(t)
        } else {
            None
        }
    }

    fn charmap(&self, _dist: f64) -> Option<char> {
        None
    }
}

/// Part of the entire plane that is right rectangle and collinear to Oyz plane.
/// Points on it is defined as `(x0, y, z): y1 <= y < y2, 0 <= z <= H`
#[derive(Debug)]
pub struct YzWalls {
    pub entity: Entity,
    pub x0: f64,
    pub y_seg: Vec<Float>,
}

impl YzWalls {
    pub fn new(entity: Entity, x0: f64, y_seg: Vec<f64>) -> Self {
        Self {
            entity,
            x0,
            y_seg: y_seg.iter().map(|f| Float(*f)).collect(),
        }
    }
}

impl AsEntity for YzWalls {
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

impl AsCollided for YzWalls {
    fn collide(&self, _cs: &CoordSys, inc: &Point, dir: &Vector) -> Option<f64> {
        if aeq(&dir[0], &0.0) || self.y_seg.is_empty() {
            return None;
        }

        let t = (self.x0 - inc[0]) / dir[0];

        let z = inc[2] + t * dir[2];
        if z < 0.0 || H < z {
            return None;
        }

        let y = Float(inc[1] + t * dir[1]);
        let idx = match self.y_seg.binary_search(&y) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if idx == 0 {
            match aeq(&y.0, &self.y_seg[0].0) {
                true => validate_collision(t),
                false => None,
            }
        } else if idx == self.y_seg.len() {
            match aeq(&y.0, &self.y_seg.last().unwrap().0) {
                true => validate_collision(t),
                false => None,
            }
        } else if idx % 2 == 1 || aeq(&y.0, &self.y_seg[idx - 1].0) || aeq(&y.0, &self.y_seg[idx].0) {
            validate_collision(t)
        } else {
            None
        }
    }

    fn charmap(&self, _dist: f64) -> Option<char> {
        None
    }
}