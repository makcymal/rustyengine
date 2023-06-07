use {
    crate::labyrinth::{
        action::*,
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


pub const STEP: f64 = 0.5;
pub const XZPANES: usize = 11;
pub const YZPANES: usize = 11;
pub const CLUES: usize = 4;
pub const HANDLEN: f64 = 0.5;


pub struct Scene {
    xz_panes: [XZPanes; XZPANES],
    yz_panes: [YZPanes; YZPANES],
    ground: HypePlane,
    clues: [HypeEllipse; CLUES],
    clues_visibility: [bool; CLUES],
    clues_available: i8,
}

impl Scene {
    pub fn new(game: &mut Game<Action, DedupActions, Self>) -> Self {
        Self {
            xz_panes: [
                XZPanes::new(game.entity(), 0.0,
                             vec![0.0, 10.0]),
                XZPanes::new(game.entity(), 1.0,
                             vec![3.0, 7.0, 8.0, 9.0]),
                XZPanes::new(game.entity(), 2.0,
                             vec![0.0, 1.0, 4.0, 7.0, 8.0, 9.0]),
                XZPanes::new(game.entity(), 3.0,
                             vec![1.0, 2.0, 4.0, 5.0, 6.0, 7.0]),
                XZPanes::new(game.entity(), 4.0,
                             vec![0.0, 1.0, 3.0, 6.0, 7.0, 10.0]),
                XZPanes::new(game.entity(), 5.0,
                             vec![1.0, 6.0, 8.0, 9.0]),
                XZPanes::new(game.entity(), 6.0,
                             vec![2.0, 3.0, 4.0, 7.0, 8.0, 9.0]),
                XZPanes::new(game.entity(), 7.0,
                             vec![8.0, 9.0]),
                XZPanes::new(game.entity(), 8.0,
                             vec![4.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
                XZPanes::new(game.entity(), 9.0,
                             vec![4.0, 5.0, 8.0, 9.0]),
                XZPanes::new(game.entity(), 10.0,
                             vec![0.0, 10.0]),
            ],
            yz_panes: [
                YZPanes::new(game.entity(), 0.0,
                             vec![0.0, 10.0]),
                YZPanes::new(game.entity(), 1.0,
                             vec![1.0, 2.0, 5.0, 9.0]),
                YZPanes::new(game.entity(), 2.0,
                             vec![0.0, 8.0, 9.0, 10.0]),
                YZPanes::new(game.entity(), 3.0,
                             vec![1.0, 4.0, 7.0, 9.0]),
                YZPanes::new(game.entity(), 4.0,
                             vec![2.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
                YZPanes::new(game.entity(), 5.0,
                             vec![7.0, 8.0, 9.0, 10.0]),
                YZPanes::new(game.entity(), 6.0,
                             vec![3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
                YZPanes::new(game.entity(), 7.0,
                             vec![2.0, 3.0, 5.0, 9.0]),
                YZPanes::new(game.entity(), 8.0,
                             vec![0.0, 2.0, 3.0, 4.0, 5.0, 9.0]),
                YZPanes::new(game.entity(), 9.0,
                             vec![2.0, 3.0, 9.0, 10.0]),
                YZPanes::new(game.entity(), 10.0,
                             vec![0.0, 10.0]),
            ],
            ground: HypePlane::default(game.entity()),
            clues: [
                HypeEllipse::default(game.entity()),
                HypeEllipse::default(game.entity()),
                HypeEllipse::default(game.entity()),
                HypeEllipse::default(game.entity()),
            ],
            clues_visibility: [false; CLUES],
            clues_available: CLUES as i8,
        }
    }

    pub fn leave_clue(&mut self, pos: &Point) {
        if self.clues_available == 0 {
            return;
        }
        let i = (0..CLUES).find(|i| !self.clues_visibility[*i]).unwrap();
        let clue_pos = self.clues[i].pos_mut();
        *clue_pos.at_mut(0) = pos.at(0);
        *clue_pos.at_mut(1) = pos.at(1);
        self.clues_visibility[i] = true;
        self.clues_available -= 1;
    }

    pub fn take_clue(&mut self, pos: &Point) {
        for i in 0..CLUES {
            if !self.clues_visibility[i] { continue; }
            let clue_pos = self.clues[i].pos();
            if (clue_pos.at(0) - pos.at(0)).powi(2) +
                (clue_pos.at(1) - pos.at(1)).powi(2) < HANDLEN
            {
                self.clues_visibility[i] = false;
                self.clues_available += 1;
                break;
            }
        }
    }
}

impl AsMaterialList for Scene {
    type Item = ();

    fn append(&mut self, item: Self::Item) {}

    fn remove(&mut self, id: &Rc<Uuid>) {}

    fn get(&self, id: &Rc<Uuid>) -> Option<&Self::Item> { None }

    fn exec(&self, f: fn(&Self::Item)) {}

    fn collide(&self, cs: &CoordSys, inc: &Point, dir: &Vector) -> f64 {
        let mut dist_opt: Option<f64> = None;

        match dir.at(0).partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for i in 0..YZPANES {
                    let d = self.yz_panes[i].collide(cs, inc, dir);
                    if d >= 0.0 {
                        dist_opt = Some(d);
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                for i in (0..YZPANES).rev() {
                    let d = self.yz_panes[i].collide(cs, inc, dir);
                    if d >= 0.0 {
                        dist_opt = Some(d);
                        break;
                    }
                }
            }
            _ => ()
        };


        match dir.at(1).partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for i in 0..XZPANES {
                    let d = self.xz_panes[i].collide(cs, inc, dir);
                    if d >= 0.0 {
                        if let Some(mut dist) = dist_opt {
                            if d <= dist {
                                dist = d
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
                    let d = self.xz_panes[i].collide(cs, inc, dir);
                    if d >= 0.0 {
                        if let Some(mut dist) = dist_opt {
                            if d <= dist {
                                dist = d
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


        dist_opt.unwrap_or(-1.0)
    }
}
