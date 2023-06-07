use crate::errs::ReRes;
use {
    super::*,
    crate::{grid::*, math::*},
    std::{any::Any, collections::HashMap, f64::consts::PI, rc::Rc},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Camera {
    pub(crate) entity: Entity,
    pub(crate) pos: Point,
    pub(crate) dir: Vector,
    pub(crate) wfov: f64,
    pub(crate) hfov: f64,
    pub(crate) draw_dist: f64,
    pub(crate) lookat: Option<Point>,
    pub(crate) rays: Grid<Vector>,
}

impl Camera {
    /// Constructs new camera from the given game object
    pub fn new(
        entity: Entity,
        pos: Point,
        dir: Vector,
        draw_dist: f64,
        yfov: f64,
        zfov: f64,
        y: usize,
        z: usize,
    ) -> Self {
        Self {
            entity,
            pos,
            dir,
            wfov: yfov,
            hfov: zfov,
            draw_dist,
            lookat: None,
            rays: rays(yfov, zfov, y, z),
        }
    }

    /// Setting property `LookAt`
    pub fn set_lookat(mut self, lookat: Point) -> Self {
        self.lookat = Some(lookat);
        self
    }
}

/// Computes bunch of directions of rays when camera stands at `INITIAL_POINT` and looks in the
/// direction [1, 0, 0]. `yfov` and `zfov` are the horizontal and vertical fields of view respectively.
/// `y` and `z` are the screen width and height respectively.
/// All the vectors will be rotated with the camera rotation as well
pub(crate) fn rays(yfov: f64, zfov: f64, y: usize, z: usize) -> Grid<Vector> {
    let mut rays = Grid::new(z, y, Vector::new(vec![1.0, 0.0, 0.0]));

    let y_rays_df = rays_df(1, yfov, y);
    for c in 0..(y / 2) {
        let df = y_rays_df[c];
        for r in 0..z {
            *rays.att_mut(r, c).at_mut(1) += df;
        }
    }
    for c in ((y + 1) / 2)..y {
        let df = y_rays_df[y - 1 - c];
        for r in 0..z {
            *rays.att_mut(r, c).at_mut(1) -= df;
        }
    }

    let z_rays_df = rays_df(2, zfov, z);
    for r in 0..(z / 2) {
        let df = z_rays_df[r];
        for c in 0..y {
            *rays.att_mut(r, c).at_mut(2) += df;
        }
    }
    for r in ((z + 1) / 2)..z {
        let df = z_rays_df[z - 1 - r];
        for c in 0..y {
            *rays.att_mut(r, c).at_mut(2) -= df;
        }
    }
    rays
}

/// Computes differences between ray lays on given `axis` and the direction [1, 0, 0].
pub(crate) fn rays_df(axis: usize, fov: f64, discr: usize) -> Vec<f64> {
    let dir = Matrix::col(vec![1.0, 0.0, 0.0]);
    let mut angle = fov / 2.0;
    let angle_step = -fov / ((discr - 1) as f64);
    let mut rays_df = vec![];

    for _ in 0..(discr / 2) {
        rays_df.push(Matrix::rotation(0, axis, angle, 3).mul(&dir).att(axis, 0) / angle.cos());
        angle += angle_step;
    }
    rays_df
}

impl AsEntity for Camera {
    fn id(&self) -> &Rc<Uuid> {
        self.entity.id()
    }

    fn props(&self) -> &HashMap<&'static str, Box<dyn Any>> {
        self.entity.props()
    }

    fn props_mut(&mut self) -> &mut HashMap<&'static str, Box<dyn Any>> {
        self.entity.props_mut()
    }
}

impl AsGameObject for Camera {
    fn pos(&self) -> &Point {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut Point {
        &mut self.pos
    }

    fn dir(&self) -> &Vector {
        &self.dir
    }

    fn dir_mut(&mut self) -> &mut Vector {
        &mut self.dir
    }


    fn planar_rotate(&mut self, from: usize, to: usize, angle: f64) -> ReRes<()> {
        let rot = Matrix::rotation(from, to, angle, 3);
        self.dir.coord = rot.mul(self.dir.coord()).to_col();
        self.dir.coord.ag_failed()?;

        for r in 0..self.rays.rows() {
            for c in 0..self.rays.cols() {
                self.rays.att_mut(r, c).coord = rot.mul(self.rays.att(r, c).coord()).to_col();
                self.rays.att(r, c).coord.ag_failed()?;
            }
        }
        Ok(())
    }
}
