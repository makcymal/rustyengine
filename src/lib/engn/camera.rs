use {
    super::*,
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
            GameErr::{self, *},
            GridErr::{self, *},
            MathErr::{self, *},
        },
        grid::*,
        math::*,
    },
    std::{
        rc::Rc,
        cell::RefCell,
        ops::{
            Index, IndexMut,
        },
        any::{
            Any,
            TypeId,
        },
        f64::consts::PI,
    },
};


/// Game camera that contains `GameObject`
#[derive(Debug)]
pub struct Camera {
    pub go: GameObject,
    pub fov: f64,
    pub vfov: f64,
    pub draw_dist: f64,
    pub lookat: Option<Point>,
}

impl Camera {
    /// Constructs new camera from the given game object
    pub fn new(mut game_object: GameObject, fov: f64, vfov: f64, draw_dist: f64) -> Self {
        Self {
            go: game_object,
            fov,
            vfov,
            draw_dist,
            lookat: None,
        }
    }

    /// Setting property `LookAt`
    pub fn set_lookat(mut self, lookat: Point) -> Self {
        self.lookat = Some(lookat);
        self
    }

    /// Constructor for bunch of rays having one inception
    pub fn incepted_rays(&self, h: usize, w: usize) -> ReRes<InceptedRays> {
        let mut directions = Grid::new(h, w, Vector::new(vec![0.0; 3]));

        let pos = self.go.pos.clone();
        let dir = if let Some(lookat) = &self.lookat {
            lookat.df(&pos)?
        } else {
            self.go.dir.clone()
        };

        let (alpha, beta) = (self.fov / (w - 1) as f64, self.vfov / (h - 1) as f64);
        let (mut yaw, pitch_top) = (-alpha * (w / 2) as f64, beta * (h / 2) as f64);

        for y in 0..w {
            let yaw_ray = if yaw != 0.0 {
                Vector {
                    coord: Matrix::triag_rotation(0, 1, yaw, 3)
                        .mul(dir.coord())
                        .to_col()
                }
            } else {
                dir.clone()
            };
            let mut pitch = pitch_top;

            for z in 0..h {
                *directions.att_mut(z, y) = if pitch != 0.0 {
                    Vector {
                        coord: Matrix::triag_rotation(0, 2, pitch, 3)
                            .mul(yaw_ray.coord())
                            .to_col()
                    }
                } else {
                    yaw_ray.clone()
                };
                pitch -= beta;
            }
            yaw += alpha;
        }


        Ok(InceptedRays {
            inc: pos,
            directions,
        })
    }
}

impl Entity for Camera {
    fn core(&self) -> &EntityCore {
        self.go.core()
    }

    fn core_mut(&mut self) -> &mut EntityCore {
        self.go.core_mut()
    }
}
