use {
    crate::labyrinth::{ground::*, walls::*},
    either::Either,
    rustyengine::{engn::*, math::*},
    std::{
        cmp::Ordering,
        rc::Rc,
        ops::{Div, Mul},
    },
    uuid::Uuid,
    rand::Rng,
};

pub const STEP: f32 = 1.0;
pub const XZWALLS: usize = 11;
pub const YZWALLS: usize = 11;
pub const PASSAGE: f32 = 5.0;
pub const BACKWALL: f32 = 55.0;

pub struct Scene {
    xz_walls: [XzWalls; XZWALLS],
    xz_charcoal: Charcoal,
    yz_walls: [YzWalls; YZWALLS],
    yz_charcoal: Charcoal,
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
            XzWalls::new(7.0, vec![3.0, 4.0, 8.0, 9.0]),
            XzWalls::new(8.0, vec![4.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
            XzWalls::new(9.0, vec![4.0, 5.0, 8.0, 9.0]),
            XzWalls::new(10.0, vec![0.0, 4.0, 5.0, 10.0]),
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

        Self {
            xz_walls,
            xz_charcoal: Charcoal::new("><".to_string(), 6.0),

            yz_walls,
            yz_charcoal: Charcoal::new("^\"".to_string(), 6.0),

            ground: Ground::new(),
            ground_charcoal: Charcoal::new("#$?".to_string(), draw_dist),

            sun: HypeEllipse::new(
                Point::new([-10.0, -10.0, 80.0]),
                Basis::new(),
                [20.0, 20.0, 20.0],
                false,
                Some(Charcoal::new("0Oo".to_string(), draw_dist)),
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

    pub fn collision_ag_xz_walls(&self, inc: &Point, dir: &Vector) -> Option<f32> {
        let mut collision = None;
        match dir[1].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                let rng = match inc[1] < 0.0 {
                    true => 0..XZWALLS,
                    false => (inc[1] / PASSAGE + 1.0).floor() as usize..XZWALLS,
                };
                for i in rng {
                    if let Some(dist) = self.xz_walls[i].collide(inc, dir) {
                        if collision.is_none() || dist < collision.unwrap() {
                            collision = Some(dist);
                        }
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                let rng = match BACKWALL < inc[1] {
                    true => (0..XZWALLS).rev(),
                    false => (0..(inc[1] / PASSAGE).ceil() as usize).rev(),
                };
                for i in rng {
                    if let Some(dist) = self.xz_walls[i].collide(inc, dir) {
                        if collision.is_none() || dist < collision.unwrap() {
                            collision = Some(dist);
                        }
                        break;
                    }
                }
            }
            _ => return None,
        };
        collision
    }

    pub fn collision_ag_yz_walls(&self, inc: &Point, dir: &Vector) -> Option<f32> {
        let mut collision = None;

        match dir[0].partial_cmp(&0.0) {
            Some(Ordering::Greater) => {
                let rng = match inc[0] < 0.0 {
                    true => 0..YZWALLS,
                    false => (inc[0] / PASSAGE + 1.0).floor() as usize..YZWALLS,
                };
                for i in rng {
                    if let Some(dist) = self.yz_walls[i].collide(inc, dir) {
                        if collision.is_none() || dist < collision.unwrap() {
                            collision = Some(dist);
                        }
                        break;
                    }
                }
            }
            Some(Ordering::Less) => {
                let rng = match BACKWALL < inc[0] {
                    true => (0..YZWALLS).rev(),
                    false => (0..(inc[0] / PASSAGE).ceil() as usize).rev(),
                };
                for i in rng {
                    if let Some(dist) = self.yz_walls[i].collide(inc, dir) {
                        if collision.is_none() || dist < collision.unwrap() {
                            collision = Some(dist);
                        }
                        break;
                    }
                }
            }
            _ => return None,
        };
        collision
    }
}

impl AsScene for Scene {
    fn collide(&self, inc: &Point, dir: &Vector) -> Either<f32, char> {
        let mut collision: Option<(f32, char)> = None;

        if let Some(dist) = self.collision_ag_xz_walls(inc, dir) {
            if collision.is_none() || dist < collision.unwrap().0 {
                collision = Some((dist, self.xz_charcoal.ignite(dist)));
            }
        }

        if let Some(dist) = self.collision_ag_yz_walls(inc, dir) {
            if collision.is_none() || dist < collision.unwrap().0 {
                collision = Some((dist, self.yz_charcoal.ignite(dist)));
            }
        }

        if let Some(dist) = self.ground.collide(inc, dir) {
            if collision.is_none() || dist < collision.unwrap().0 {
                collision = Some((dist, self.ground_charcoal.ignite(dist)));
            }
        }

        if let Some(dist) = self.sun.collide(inc, dir) {
            if collision.is_none() || dist < collision.unwrap().0 {
                collision = Some((dist, self.sun.charcoal.as_ref().unwrap().ignite(dist)));
            }
        }

        if let Some((_, c)) = collision {
            Either::Right(c)
        } else {
            Either::Left(-1.0)
        }
    }

    fn validate_mv(&self, pos: &Point, mv: &mut Vector) {
        let mut collision = None;
        if let Some(dist) = self.collision_ag_xz_walls(pos, mv) {
            if collision.is_none() || dist < collision.unwrap() {
                collision = Some(dist);
            }
        }
        if let Some(dist) = self.collision_ag_yz_walls(pos, mv) {
            if collision.is_none() || dist < collision.unwrap() {
                collision = Some(dist);
            }
        }

        if let Some(dist) = collision {
            if dist < 1.0 {
                *mv = Vector::new([0.0; 3])
            }
        }
    }
}

pub fn gen_init_pos() -> Point {
    let x = (rand::thread_rng().gen_range(0..(XZWALLS - 1)) as f32 + 0.5) * PASSAGE;
    let y = (rand::thread_rng().gen_range(0..(YZWALLS - 1)) as f32 + 0.5) * PASSAGE;
    Point::new([x, y, 2.0])
}
