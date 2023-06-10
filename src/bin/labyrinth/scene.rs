use {
    crate::labyrinth::{
        ground::*,
        pane::*,
    },
    rustyengine::{
        engn::*,
        math::*,
    },
    std::{
        rc::Rc,
        cmp::Ordering,
    },
    uuid::Uuid,
};


pub const STEP: f32 = 0.5;
pub const XZPANES: usize = 11;
pub const YZPANES: usize = 11;


pub struct Scene {
    xz_panes: [XZPanes; XZPANES],
    yz_panes: [YZPanes; YZPANES],
    ground: Ground,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            xz_panes: [
                XZPanes::new(0.0, vec![0.0, 10.0]),
                XZPanes::new(1.0, vec![3.0, 7.0, 8.0, 9.0]),
                XZPanes::new(2.0, vec![0.0, 1.0, 4.0, 7.0, 8.0, 9.0]),
                XZPanes::new(3.0, vec![1.0, 2.0, 4.0, 5.0, 6.0, 7.0]),
                XZPanes::new(4.0, vec![0.0, 1.0, 3.0, 6.0, 7.0, 10.0]),
                XZPanes::new(5.0, vec![1.0, 6.0, 8.0, 9.0]),
                XZPanes::new(6.0, vec![2.0, 3.0, 4.0, 7.0, 8.0, 9.0]),
                XZPanes::new(7.0, vec![8.0, 9.0]),
                XZPanes::new(8.0, vec![4.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
                XZPanes::new(9.0, vec![4.0, 5.0, 8.0, 9.0]),
                XZPanes::new(10.0, vec![0.0, 10.0]),
            ],
            yz_panes: [
                YZPanes::new(0.0, vec![0.0, 10.0]),
                YZPanes::new(1.0, vec![1.0, 2.0, 5.0, 9.0]),
                YZPanes::new(2.0, vec![0.0, 8.0, 9.0, 10.0]),
                YZPanes::new(3.0, vec![1.0, 4.0, 7.0, 9.0]),
                YZPanes::new(4.0, vec![2.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
                YZPanes::new(5.0, vec![7.0, 8.0, 9.0, 10.0]),
                YZPanes::new(6.0, vec![3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
                YZPanes::new(7.0, vec![2.0, 3.0, 5.0, 9.0]),
                YZPanes::new(8.0, vec![0.0, 2.0, 3.0, 4.0, 5.0, 9.0]),
                YZPanes::new(9.0, vec![2.0, 3.0, 9.0, 10.0]),
                YZPanes::new(10.0, vec![0.0, 10.0]),
            ],
            ground: Ground::new(),
        }
    }
}

impl AsScene for Scene {
    fn collide(&self, inc: &Point, dir: &Vector) -> f32 {
        let mut dist_opt: Option<f32> = None;

        match dir[0].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for i in 0..YZPANES {
                    let d = self.yz_panes[i].collide(inc, dir);
                    if d >= 0.0 {
                        dist_opt = Some(d);
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                for i in (0..YZPANES).rev() {
                    let d = self.yz_panes[i].collide(inc, dir);
                    if d >= 0.0 {
                        dist_opt = Some(d);
                        break;
                    }
                }
            }
            _ => ()
        };


        match dir[1].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for i in 0..XZPANES {
                    let d = self.xz_panes[i].collide(inc, dir);
                    if d >= 0.0 {
                        if let Some(ref mut dist) = dist_opt {
                            if d <= *dist {
                                *dist = d
                            }
                        } else {
                            dist_opt = Some(d);
                        }
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                for i in (0..XZPANES).rev() {
                    let d = self.xz_panes[i].collide(inc, dir);
                    if d >= 0.0 {
                        if let Some(ref mut dist) = dist_opt {
                            if d <= *dist {
                                *dist = d
                            }
                        } else {
                            dist_opt = Some(d);
                        }
                        break;
                    }
                }
            }
            _ => (),
        };

        let d = self.ground.collide(inc, dir);
        if let Some(ref mut dist) = dist_opt {
            if d <= *dist {
                *dist = d
            }
        } else {
            dist_opt = Some(d)
        }

        // dbg!(inc, dir, &dist_opt);
        dist_opt.unwrap_or(-1.0)
    }
}

