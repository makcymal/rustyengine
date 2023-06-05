use std::collections::BTreeSet;
use {
    rustyengine::{
        engn::*,
        math::*,
    },
    std::{
        any::Any,
        collections::HashMap,
        rc::Rc,
        cmp::Ordering,
        ops::Bound,
    },
    uuid::Uuid,
};

/// Height of all panes
const H: f64 = 10.0;

/// Part of the entire plane that is right rectangle and collinear to Oxz plane.
/// Points on it is defined as `(x, y0, z): x1 <= x < x2, 0 <= z <= H`
#[derive(Debug, Clone, PartialEq)]
pub struct XZPanes {
    pub core: Core,
    pub y0: f64,
    pub x_seg: Vec<f64>,
}

impl XZPanes {
    pub fn new(core: Core, y0: f64, x_seg: Vec<f64>) -> Self {
        Self { core, y0, x_seg }
    }
}

impl Entity for XZPanes {
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

impl GameObject for XZPanes {
    fn pos(&self) -> &Point {
        self.props().get("pos").unwrap().downcast_ref::<Point>().unwrap()
    }

    fn pos_mut(&mut self) -> &mut Point {
        self.props_mut().get("pos").unwrap().downcast_mut::<Point>().unwrap()
    }

    fn dir(&self) -> &Vector {
        self.props().get("dir").unwrap().downcast_ref::<Point>().unwrap()
    }

    fn dir_mut(&mut self) -> &mut Vector {
        self.props_mut().get("dir").unwrap().downcast_mut::<Point>().unwrap()
    }

    fn intersect(&self, _cs: &CoordSys, ray: &Ray) -> f64 {
        if aeq(&ray.dir.at(1), &0.0) {
            return -1.0;
        }

        let t = (self.y0 - ray.inc.at(1)) / ray.dir.at(1);

        let z = ray.inc.at(2) + t * ray.dir.at(2);
        if z < 0.0 || H < z {
            return -1.0;
        }

        let x = ray.inc.at(0) + t * ray.dir.at(0);
        let idx = match self.x_seg.binary_search(&x) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if idx % 2 == 1 || idx == 0 || x == self.x_seg[idx - 1] || x == self.x_seg[idx] {
            return t;
        }

        -1.0
    }
}

/// Part of the entire plane that is right rectangle and collinear to Oyz plane.
/// Points on it is defined as `(x0, y, z): y1 <= y < y2, 0 <= z <= H`
#[derive(Debug, Clone, PartialEq)]
pub struct YZPanes {
    pub core: Core,
    pub x0: f64,
    pub y_seg: Vec<f64>,
}

impl YZPanes {
    pub fn new(core: Core, x0: f64, y_seg: Vec<f64>) -> Self {
        Self { core, x0, y_seg }
    }
}

impl Entity for YZPanes {
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

impl GameObject for YZPanes {
    fn pos(&self) -> &Point {
        self.props().get("pos").unwrap().downcast_ref::<Point>().unwrap()
    }

    fn pos_mut(&mut self) -> &mut Point {
        self.props_mut().get("pos").unwrap().downcast_mut::<Point>().unwrap()
    }

    fn dir(&self) -> &Vector {
        self.props().get("dir").unwrap().downcast_ref::<Point>().unwrap()
    }

    fn dir_mut(&mut self) -> &mut Vector {
        self.props_mut().get("dir").unwrap().downcast_mut::<Point>().unwrap()
    }

    fn intersect(&self, _cs: &CoordSys, ray: &Ray) -> f64 {
        if aeq(&ray.dir.at(0), &0.0) {
            return -1.0;
        }

        let t = (self.x0 - ray.inc.at(0)) / ray.dir.at(0);

        let z = ray.inc.at(2) + t * ray.dir.at(2);
        if z < 0.0 || H < z {
            return -1.0;
        }

        let y = ray.inc.at(1) + t * ray.dir.at(1);
        let idx = match self.y_seg.binary_search(&y) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        if idx % 2 == 1 || idx == 0 || y == self.y_seg[idx - 1] || y == self.y_seg[idx] {
            return t;
        }

        -1.0
    }
}
