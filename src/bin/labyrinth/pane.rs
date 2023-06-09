use {
    rustyengine::{
        engn::*,
        math::*,
    },
    std::{any::Any, cmp::Ordering, collections::HashMap, rc::Rc},
    uuid::Uuid,
};


/// Height of all panes
const H: f64 = 10.0;


/// Part of the entire plane that is right rectangle and collinear to Oxz plane.
/// Points on it is defined as `(x, y0, z): x1 <= x < x2, 0 <= z <= H`
#[derive(Debug)]
pub struct XZPanes {
    pub core: Entity,
    pub y0: f64,
    pub x_seg: Vec<Float>,
}

impl XZPanes {
    pub fn new(core: Entity, y0: f64, x_seg: Vec<f64>) -> Self {
        Self {
            core,
            y0,
            x_seg: x_seg.iter().map(|f| Float(*f)).collect(),
        }
    }
}

impl AsEntity for XZPanes {
    fn id(&self) -> &Rc<Uuid> {
        self.core.id()
    }

    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>> {
        self.core.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>> {
        self.core.props_mut()
    }
}

impl AsCollided for XZPanes {
    fn collide(&self, _cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        if aeq(&dir.at(1), &0.0) || self.x_seg.is_empty() {
            return -1.0;
        }

        let t = (self.y0 - inc.at(1)) / dir.at(1);

        let z = inc.at(2) + t * dir.at(2);
        if z < 0.0 || H < z {
            return -1.0;
        }

        let x = Float(inc.at(0) + t * dir.at(0));
        let idx = match self.x_seg.binary_search(&x) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if idx == 0 {
            match aeq(&x.0, &self.x_seg[0].0) {
                true => t,
                false => -1.0
            }
        }
        else if idx == self.x_seg.len() {
            match aeq(&x.0, &self.x_seg.last().unwrap().0) {
                true => t,
                false => -1.0
            }
        }
        else if idx % 2 == 1 || aeq(&x.0, &self.x_seg[idx - 1].0) || aeq(&x.0, &self.x_seg[idx].0) {
            t
        } else {
            -1.0
        }
    }
}

impl AsGameObject for XZPanes {
    fn pos(&self) -> &Point {
        self.props()
            .get("pos")
            .unwrap()
            .downcast_ref::<Point>()
            .unwrap()
    }

    fn pos_mut(&mut self) -> &mut Point {
        self.props_mut()
            .get_mut("pos")
            .unwrap()
            .downcast_mut::<Point>()
            .unwrap()
    }

    fn dir(&self) -> &Matrix {
        self.props()
            .get("dir")
            .unwrap()
            .downcast_ref::<Matrix>()
            .unwrap()
    }

    fn dir_mut(&mut self) -> &mut Matrix {
        self.props_mut()
            .get_mut("dir")
            .unwrap()
            .downcast_mut::<Matrix>()
            .unwrap()
    }
}


/// Part of the entire plane that is right rectangle and collinear to Oyz plane.
/// Points on it is defined as `(x0, y, z): y1 <= y < y2, 0 <= z <= H`
#[derive(Debug)]
pub struct YZPanes {
    pub core: Entity,
    pub x0: f64,
    pub y_seg: Vec<Float>,
}

impl YZPanes {
    pub fn new(core: Entity, x0: f64, y_seg: Vec<f64>) -> Self {
        Self {
            core,
            x0,
            y_seg: y_seg.iter().map(|f| Float(*f)).collect(),
        }
    }
}

impl AsEntity for YZPanes {
    fn id(&self) -> &Rc<Uuid> {
        self.core.id()
    }

    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>> {
        self.core.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>> {
        self.core.props_mut()
    }
}

impl AsCollided for YZPanes {
    fn collide(&self, _cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        if aeq(&dir.at(0), &0.0) || self.y_seg.is_empty() {
            return -1.0;
        }

        let t = (self.x0 - inc.at(0)) / dir.at(0);

        let z = inc.at(2) + t * dir.at(2);
        if z < 0.0 || H < z {
            return -1.0;
        }

        let y = Float(inc.at(1) + t * dir.at(1));
        let idx = match self.y_seg.binary_search(&y) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if idx == 0 {
            match aeq(&y.0, &self.y_seg[0].0) {
                true => t,
                false => -1.0
            }
        }
        else if idx == self.y_seg.len() {
            match aeq(&y.0, &self.y_seg.last().unwrap().0) {
                true => t,
                false => -1.0
            }
        }
        else if idx % 2 == 1 || aeq(&y.0, &self.y_seg[idx - 1].0) || aeq(&y.0, &self.y_seg[idx].0) {
            t
        } else {
            -1.0
        }
    }
}

impl AsGameObject for YZPanes {
    fn pos(&self) -> &Point {
        self.props()
            .get("pos")
            .unwrap()
            .downcast_ref::<Point>()
            .unwrap()
    }

    fn pos_mut(&mut self) -> &mut Point {
        self.props_mut()
            .get_mut("pos")
            .unwrap()
            .downcast_mut::<Point>()
            .unwrap()
    }

    fn dir(&self) -> &Matrix {
        self.props()
            .get("dir")
            .unwrap()
            .downcast_ref::<Matrix>()
            .unwrap()
    }

    fn dir_mut(&mut self) -> &mut Matrix {
        self.props_mut()
            .get_mut("dir")
            .unwrap()
            .downcast_mut::<Matrix>()
            .unwrap()
    }
}
