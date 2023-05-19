use std::ffi::c_short;
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
        math::{
            Matrix, Point, CoordSys,
        },
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
    go: GameObject,
}

impl Camera {
    /// Constructs new camera from the given game object
    pub fn new(mut game_object: GameObject) -> Self {
        Self { go: game_object }
    }

    /// Setting property `Fov` and validating it
    pub fn set_fov(mut self, fov: f64) -> ReRes<Self> {
        if fov < 0.0 || PI <= fov {
            return Err(GameErr(InvalidPropF64 { key: Prop::Fov, val: fov }));
        }
        self.go.core_mut().set_prop(Prop::Fov, Box::new(fov));
        Ok(self)
    }

    /// Getting value of property `Fov` if it set or meaningful error
    pub fn get_fov(&self) -> ReRes<f64> {
        Ok(*self.go.core().get_prop(Prop::Fov)?.downcast_ref::<f64>().unwrap())
    }

    /// Setting property `VFov` and validating it
    pub fn set_vfov(mut self, vfov: f64) -> ReRes<Self> {
        if vfov < 0.0 || PI <= vfov {
            return Err(GameErr(InvalidPropF64 { key: Prop::VFov, val: vfov }));
        }
        self.go.core_mut().set_prop(Prop::VFov, Box::new(vfov));
        Ok(self)
    }

    /// Getting value of property `VFov` if it set or meaningful error
    pub fn get_vfov(&self) -> ReRes<f64> {
        Ok(*self.go.core().get_prop(Prop::VFov)?.downcast_ref::<f64>().unwrap())
    }

    /// Setting property `VFov` and validating it
    pub fn set_draw_dist(mut self, draw_dist: f64) -> ReRes<Self> {
        if draw_dist < 0.0 {
            return Err(GameErr(InvalidPropF64 { key: Prop::DrawDist, val: draw_dist }));
        }
        self.go.core_mut().set_prop(Prop::DrawDist, Box::new(draw_dist));
        Ok(self)
    }

    /// Getting value of property `DrawDist` if it set or meaningful error
    pub fn get_draw_dist(&self) -> ReRes<f64> {
        Ok(*self.go.core().get_prop(Prop::DrawDist)?.downcast_ref::<f64>().unwrap())
    }

    /// Setting property `LookAt`
    pub fn set_lookat(mut self, lookat: Point) -> Self {
        self.go.core_mut().set_prop(Prop::LookAt, Box::new(lookat));
        self
    }

    /// Getting value of property `LookAt` if it set or meaningful error
    pub fn get_lookat(&self) -> ReRes<&Point> {
        Ok(self.go.core().get_prop(Prop::LookAt)?.downcast_ref::<Point>().unwrap())
    }

    /// Setting property `Roll`
    pub fn set_roll(mut self, roll: f64) -> ReRes<Self> {
        self.go.core_mut().set_prop(Prop::Roll, Box::new(roll));
        Ok(self)
    }

    /// Getting value of property `Roll` if it set or meaningful error
    pub fn get_roll(&self) -> ReRes<f64> {
        Ok(*self.go.core().get_prop(Prop::Roll)?.downcast_ref::<f64>().unwrap())
    }

    /// Constructor for bunch of rays having one inception, does not depend on `Prop::Roll` angle
    pub fn incepted_rays(&self, mut h: usize, mut w: usize) -> ReRes<InceptedRays> {
        if h % 2 == 1 {
            h += 1
        }
        if w % 2 == 1 {
            w += 1
        }
        let mut directions = Grid::new(h + 1, w + 1, Matrix::col(vec![0.0; 3]));
        let mut lens = Matrix::zero(h + 1, w + 1);

        let cs = Rc::clone(&self.go.core.cs);
        let fov = self.get_fov()?;
        let vfov = match self.get_vfov() {
            Ok(val) => val,
            Err(_) => fov / (w as f64) * (h as f64),
        };

        let pos = self.go.core.get_prop(Prop::Pos).unwrap().downcast_ref::<Point>().unwrap().clone();
        let dir = if let Ok(lookat) = self.get_lookat() {
            lookat.sub(&pos)?
        } else {
            self.go.core.get_prop(Prop::Dir).unwrap().downcast_ref::<Matrix>().unwrap().clone()
        };
        let dir_sqlen = cs.space().scalar_prod(&dir, &dir)?;

        let (alpha, beta) = (fov / w as f64, vfov / h as f64);
        let (mut yaw, pitch_top) = (-alpha * (w / 2) as f64, beta * (h / 2) as f64);

        for y in 0..(w + 1) {
            let yaw_ray = &Matrix::rotation(0, 1, yaw, 3).mul(&dir).to_col();
            let yaw_len = dir_sqlen / cs.space().scalar_prod(&dir, &yaw_ray)?;
            yaw_ray.num_mul(yaw_len);

            let mut pitch = pitch_top;
            for z in 0..(h + 1) {
                let mut ray: Matrix = Matrix::rotation(0, 2, pitch, 3).mul(&yaw_ray).to_col();
                let len = yaw_len * dir_sqlen / cs.space().scalar_prod(&yaw_ray, &ray)?;
                ray = ray.num_mul_assign(len);
                *directions.att_mut(z, y) = ray;
                *lens.att_mut(z, y) = len;

                pitch -= beta;
            }
            yaw += alpha;
        }


        Ok(InceptedRays {
            cs,
            inc: pos,
            directions,
            lens
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
