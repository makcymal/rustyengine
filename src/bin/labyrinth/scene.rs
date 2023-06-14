use std::ops::{Div, Mul};
use {
    crate::labyrinth::{
        ground::*,
        walls::*,
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
    either::Either,
};


pub const STEP: f32 = 0.2;
pub const XZWALLS: usize = 11;
pub const YZWALLS: usize = 11;
pub const PASSAGE: f32 = 5.0;
pub const BACKWALL: f32 = 55.0;
pub const GAP: f32 = 0.5;


pub struct Scene {
    xz_walls: [XzWalls; XZWALLS],
    xz_walls_charcoal: Charcoal,
    xz_limiters: [XzWalls; 2 * XZWALLS],
    yz_walls: [YzWalls; YZWALLS],
    yz_walls_charcoal: Charcoal,
    yz_limiters: [YzWalls; 2 * YZWALLS],
    ground: Ground,
    ground_charcoal: Charcoal,
    sun: HypeEllipse,
}

impl Scene {
    pub fn new(draw_dist: f32) -> Self {
        let xz_walls = [
            XzWalls::new(0.0, vec![0.0, 10.0]),
            XzWalls::new(1.0, vec![3.0, 7.0, 8.0, 9.0]),
            XzWalls::new(2.0, vec![0.0, 1.0, 4.0, 7.0, 8.0, 9.0]),
            XzWalls::new(3.0, vec![1.0, 2.0, 4.0, 5.0, 6.0, 7.0]),
            XzWalls::new(4.0, vec![0.0, 1.0, 3.0, 6.0, 7.0, 10.0]),
            XzWalls::new(5.0, vec![1.0, 6.0, 8.0, 9.0]),
            XzWalls::new(6.0, vec![2.0, 3.0, 4.0, 7.0, 8.0, 9.0]),
            XzWalls::new(7.0, vec![8.0, 9.0]),
            XzWalls::new(8.0, vec![4.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
            XzWalls::new(9.0, vec![4.0, 5.0, 8.0, 9.0]),
            XzWalls::new(10.0, vec![0.0, 10.0]),
        ];
        let xz_limiters = [
            XzWalls::new(0.0 - GAP, vec![0.0 - GAP, 10.0 + GAP]),
            XzWalls::new(0.0 + GAP, vec![0.0 - GAP, 10.0 + GAP]),
            XzWalls::new(1.0 - GAP, vec![3.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(1.0 + GAP, vec![3.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(2.0 - GAP, vec![0.0 - GAP, 1.0 + GAP, 4.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(2.0 + GAP, vec![0.0 - GAP, 1.0 + GAP, 4.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(3.0 - GAP, vec![1.0 - GAP, 2.0 + GAP, 4.0 - GAP, 5.0 + GAP, 6.0 - GAP, 7.0 + GAP]),
            XzWalls::new(3.0 + GAP, vec![1.0 - GAP, 2.0 + GAP, 4.0 - GAP, 5.0 + GAP, 6.0 - GAP, 7.0 + GAP]),
            XzWalls::new(4.0 - GAP, vec![0.0 - GAP, 1.0 + GAP, 3.0 - GAP, 6.0 + GAP, 7.0 - GAP, 10.0 + GAP]),
            XzWalls::new(4.0 + GAP, vec![0.0 - GAP, 1.0 + GAP, 3.0 - GAP, 6.0 + GAP, 7.0 - GAP, 10.0 + GAP]),
            XzWalls::new(5.0 - GAP, vec![1.0 - GAP, 6.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(5.0 + GAP, vec![1.0 - GAP, 6.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(6.0 - GAP, vec![2.0 - GAP, 3.0 + GAP, 4.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(6.0 + GAP, vec![2.0 - GAP, 3.0 + GAP, 4.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(7.0 - GAP, vec![8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(7.0 + GAP, vec![8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(8.0 - GAP, vec![4.0 - GAP, 5.0 + GAP, 6.0 - GAP, 7.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            XzWalls::new(8.0 + GAP, vec![4.0 - GAP, 5.0 + GAP, 6.0 - GAP, 7.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            XzWalls::new(9.0 - GAP, vec![4.0 - GAP, 5.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(9.0 + GAP, vec![4.0 - GAP, 5.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            XzWalls::new(10.0 - GAP, vec![0.0 - GAP, 10.0 + GAP]),
            XzWalls::new(10.0 + GAP, vec![0.0 - GAP, 10.0 + GAP]),
        ];

        let yz_walls = [
            YzWalls::new(0.0, vec![0.0, 10.0]),
            YzWalls::new(1.0, vec![1.0, 2.0, 5.0, 9.0]),
            YzWalls::new(2.0, vec![0.0, 8.0, 9.0, 10.0]),
            YzWalls::new(3.0, vec![1.0, 4.0, 7.0, 9.0]),
            YzWalls::new(4.0, vec![2.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
            YzWalls::new(5.0, vec![7.0, 8.0, 9.0, 10.0]),
            YzWalls::new(6.0, vec![3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            YzWalls::new(7.0, vec![2.0, 3.0, 5.0, 9.0]),
            YzWalls::new(8.0, vec![0.0, 2.0, 3.0, 4.0, 5.0, 9.0]),
            YzWalls::new(9.0, vec![2.0, 3.0, 9.0, 10.0]),
            YzWalls::new(10.0, vec![0.0, 10.0]),
        ];

        let yz_limiters = [
            YzWalls::new(0.0 - GAP, vec![0.0 - GAP, 10.0 + GAP]),
            YzWalls::new(0.0 + GAP, vec![0.0 - GAP, 10.0 + GAP]),
            YzWalls::new(1.0 - GAP, vec![1.0 - GAP, 2.0 + GAP, 5.0 - GAP, 9.0 + GAP]),
            YzWalls::new(1.0 + GAP, vec![1.0 - GAP, 2.0 + GAP, 5.0 - GAP, 9.0 + GAP]),
            YzWalls::new(2.0 - GAP, vec![0.0 - GAP, 8.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(2.0 + GAP, vec![0.0 - GAP, 8.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(3.0 - GAP, vec![1.0 - GAP, 4.0 + GAP, 7.0 - GAP, 9.0 + GAP]),
            YzWalls::new(3.0 + GAP, vec![1.0 - GAP, 4.0 + GAP, 7.0 - GAP, 9.0 + GAP]),
            YzWalls::new(4.0 - GAP, vec![2.0 - GAP, 3.0 + GAP, 6.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            YzWalls::new(4.0 + GAP, vec![2.0 - GAP, 3.0 + GAP, 6.0 - GAP, 7.0 + GAP, 8.0 - GAP, 9.0 + GAP]),
            YzWalls::new(5.0 - GAP, vec![7.0 - GAP, 8.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(5.0 + GAP, vec![7.0 - GAP, 8.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(6.0 - GAP, vec![3.0 - GAP, 5.0 + GAP, 7.0 - GAP, 8.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(6.0 + GAP, vec![3.0 - GAP, 5.0 + GAP, 7.0 - GAP, 8.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(7.0 - GAP, vec![2.0 - GAP, 3.0 + GAP, 5.0 - GAP, 9.0 + GAP]),
            YzWalls::new(7.0 + GAP, vec![2.0 - GAP, 3.0 + GAP, 5.0 - GAP, 9.0 + GAP]),
            YzWalls::new(8.0 - GAP, vec![0.0 - GAP, 2.0 + GAP, 3.0 - GAP, 4.0 + GAP, 5.0 - GAP, 9.0 + GAP]),
            YzWalls::new(8.0 + GAP, vec![0.0 - GAP, 2.0 + GAP, 3.0 - GAP, 4.0 + GAP, 5.0 - GAP, 9.0 + GAP]),
            YzWalls::new(9.0 - GAP, vec![2.0 - GAP, 3.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(9.0 + GAP, vec![2.0 - GAP, 3.0 + GAP, 9.0 - GAP, 10.0 + GAP]),
            YzWalls::new(10.0 - GAP, vec![0.0 - GAP, 10.0 + GAP]),
            YzWalls::new(10.0 + GAP, vec![0.0 - GAP, 10.0 + GAP]),
        ];

        Self {
            xz_walls,
            xz_walls_charcoal: Charcoal::new("=".to_string(), draw_dist),
            xz_limiters,

            yz_walls,
            yz_walls_charcoal: Charcoal::new("^".to_string(), draw_dist),
            yz_limiters,

            ground: Ground::new(),
            ground_charcoal: Charcoal::new("#".to_string(), draw_dist),

            sun: HypeEllipse::new(
                Point::new([0.0, 0.0, 100.0]),
                Basis::new(),
                [50.0, 50.0, 50.0],
                false,
                Some(Charcoal::new("O".to_string(), draw_dist)),
            ),
        }
    }

    pub fn expand(&mut self) {
        let coef = PASSAGE;
        for wall in &mut self.xz_walls {
            wall.y0 *= coef;
            for coord in &mut wall.x_seg {
                *coord *= coef;
            }
        }
        for wall in &mut self.yz_walls {
            wall.x0 *= coef;
            for coord in &mut wall.y_seg {
                *coord *= coef;
            }
        }
    }
}


impl AsScene for Scene {
    fn collide(&self, inc: &Point, dir: &Vector) -> Either<f32, char> {
        let mut collision: Option<(f32, char)> = None;

        match dir[0].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                let rng = match inc[0] < 0.0 {
                    true => 0..YZWALLS,
                    false => (inc[0] / PASSAGE + 1.0).floor() as usize ..YZWALLS,
                };
                for i in rng {
                    if let Some(dist) = self.yz_walls[i].collide(inc, dir) {
                        update_collision(&mut collision, dist, &self.yz_walls_charcoal);
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                let rng = match BACKWALL < inc[0] {
                    true => (0..YZWALLS).rev(),
                    false => (0..(inc[0] / PASSAGE).ceil() as usize).rev()
                };
                for i in rng {
                    if let Some(dist) = self.yz_walls[i].collide(inc, dir) {
                        update_collision(&mut collision, dist, &self.yz_walls_charcoal);
                        break;
                    }
                }
            }
            _ => ()
        };


        match dir[1].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                let rng = match inc[0] < 0.0 {
                    true => 0..XZWALLS,
                    false => (inc[0] / PASSAGE + 1.0).floor() as usize ..XZWALLS,
                };
                for i in rng {
                    if let Some(dist) = self.xz_walls[i].collide(inc, dir) {
                        update_collision(&mut collision, dist, &self.xz_walls_charcoal);
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                let rng = match BACKWALL < inc[0] {
                    true => (0..XZWALLS).rev(),
                    false => (0..(inc[0] / PASSAGE).ceil() as usize).rev()
                };
                for i in rng {
                    if let Some(dist) = self.xz_walls[i].collide(inc, dir) {
                        update_collision(&mut collision, dist, &self.xz_walls_charcoal);
                        break;
                    }
                }
            }
            _ => (),
        };

        if let Some(dist) = self.ground.collide(inc, dir) {
            update_collision(&mut collision, dist, &self.ground_charcoal);
        }

        if let Some(dist) = self.sun.collide(inc, dir) {
            update_collision(&mut collision, dist, &self.sun.charcoal.as_ref().unwrap());
        }

        if let Some((_, c)) = collision {
            Either::Right(c)
        } else {
            Either::Left(-1.0)
        }
    }

    fn validate_mv(&self, pos: &Point, mv: &mut Vector) {
        match mv[0].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for i in 0..YZWALLS.mul(2) {
                    if let Some(dist) = self.yz_limiters[i].collide(pos, mv) {
                        if dist < 1.0 {
                            mv.resize(dist * 0.1);
                        }
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                for i in (0..YZWALLS.mul(2)).rev() {
                    if let Some(dist) = self.yz_limiters[i].collide(pos, mv) {
                        if dist < 1.0 {
                            mv.resize(dist * 0.1);
                        }
                        break;
                    }
                }
            }
            _ => ()
        }

        match mv[1].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                for i in 0..XZWALLS.mul(2) {
                    if let Some(dist) = self.xz_limiters[i].collide(pos, mv) {
                        if dist < 1.0 {
                            mv.resize(dist * 0.1);
                        }
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                for i in (0..XZWALLS.mul(2)).rev() {
                    if let Some(dist) = self.xz_limiters[i].collide(pos, mv) {
                        if dist < 1.0 {
                            mv.resize(dist * 0.1);
                        }
                        break;
                    }
                }
            }
            _ => ()
        }
    }
}

fn update_collision(collision: &mut Option<(f32, char)>, dist: f32, charcoal: &Charcoal) {
        match collision {
            Some(col) =>  {
                if dist < col.0 {
                    collision.as_mut().unwrap().0 = dist;
                    collision.as_mut().unwrap().1 = charcoal.ignite(dist);
                }
            },
            None => *collision = Some((dist, charcoal.ignite(dist))),
        }
}
