use crate::errs::ReRes;
use {
    super::*,
    crate::{grid::*, math::*},
    std::{any::Any, collections::HashMap, f64::consts::PI, rc::Rc},
    uuid::Uuid,
};


#[derive(Debug)]
pub struct Vision {
    // first - zenith rotation
    // second - azimut rotation
    // third - rows of rays
    // fourth - columns of rays

    rays: Vec<Vec<Vec<Vec<Vector>>>>,
}

impl Vision {
    pub(crate) fn new(discr: usize, wfov: f64, hfov: f64, size: (usize, usize)) -> Self {
        let mut vision: Vec<Vec<Vec<Vec<Vector>>>> = vec![vec![vec![vec![]; size.0]; 4 * discr]; 2 * discr - 1];
        let wfov_step = wfov / (size.1 as f64);
        let hfov_step = hfov / (size.0 as f64);

        vision[discr - 1][0] = rays(wfov, hfov, size.1, size.0);

        let rot = &Matrix::rotation(0, 2, hfov_step, 3);
        for i in (0..(discr - 1)).rev() {
            for r in 0..size.0 {
                for c in 0..size.1 {
                    let coord = rot.mul(vision[i + 1][0][r][c].coord());
                    vision[i][0][r].push(Vector { coord })
                }
            }
        }

        let rot = &Matrix::rotation(0, 2, -hfov_step, 3);
        for i in discr..(2 * discr - 1) {
            for r in 0..size.0 {
                for c in 0..size.1 {
                    let coord = rot.mul(vision[i - 1][0][r][c].coord());
                    vision[i][0][r].push(Vector { coord })
                }
            }
        }

        let rot = &Matrix::rotation(0, 1, wfov_step, 3);
        for i in 0..(2 * discr - 1) {
            for j in 1..(4 * discr) {
                for r in 0..size.0 {
                    for c in 0..size.1 {
                        let coord = rot.mul(vision[i][j - 1][r][c].coord());
                        vision[i][j][r].push(Vector { coord })
                    }
                }
            }
        }

        Self { rays: vision }
    }
}


/// Camera object that can be moved and rotated
#[derive(Debug)]
pub struct Camera {
    pub(crate) pos: Point,
    pub(crate) vision: Vision,
    pub(crate) dir: Vec<f64>,
    pub(crate) discr: usize,
    pub(crate) zen_idx: usize,
    pub(crate) azi_idx: usize,
    pub(crate) zen_max: usize,
    pub(crate) azi_max: usize,
    pub(crate) size: (usize, usize),
    pub(crate) wfov: f64,
    pub(crate) hfov: f64,
    pub(crate) draw_dist: f64
}

impl Camera {
    pub fn new(pos: Point, discr: usize, yfov: f64, zfov: f64, mut size: (usize, usize), draw_dist: f64) -> Self {
        if size.0 % 2 == 0 { size.0 -= 1 }
        if size.1 % 2 == 0 { size.1 -= 1 }
        Self {
            pos,
            vision: Vision::new(discr, yfov, zfov, size),
            dir: (0..(discr + 1)).map(|i| ((i / discr) as f64).cos()).collect(),
            discr,
            zen_idx: discr - 1,
            azi_idx: 0,
            zen_max: 2 * discr - 1,
            azi_max: 4 * discr,
            size,
            wfov: yfov,
            hfov: zfov,
            draw_dist,
        }
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn dir(&self) -> (f64, f64) {
        let ang = ((2 * self.azi_idx) as f64 / self.azi_max as f64) * PI;
        (ang.cos(), ang.sin())
    }

    pub fn mv(&mut self, vec: &Vector) -> ReRes<()> {
        self.pos.mv_assign(vec)
    }

    pub fn ray(&self, r: usize, c: usize) -> &Vector {
        &self.vision.rays[self.zen_idx][self.azi_idx][r][c]
    }

    pub fn rotate_up(&mut self, step: usize) {
        self.zen_idx = self.zen_idx.saturating_sub(step)
    }

    pub fn rotate_down(&mut self, step: usize) {
        let idx = self.zen_idx + step;
        self.zen_idx = if idx >= self.zen_max {
            self.zen_max
        } else {
            idx
        }
    }

    pub fn rotate_left(&mut self, step: usize) {
        self.azi_idx = (self.azi_idx + step) % self.azi_max
    }

    pub fn rotate_right(&mut self, step: usize) {
        if self.azi_idx >= step {
            self.azi_idx = self.azi_idx - step
        } else {
            self.azi_idx = self.azi_max + self.azi_idx - (step % self.azi_max)
        }
    }
}

/// Computes bunch of directions of rays when camera stands at `INITIAL_POINT` and looks in the
/// direction [1, 0, 0]. `yfov` and `zfov` are the horizontal and vertical fields of view respectively.
/// `y` and `z` are the screen width and height respectively.
/// All the vectors will be rotated with the camera rotation as well
pub(crate) fn rays(yfov: f64, zfov: f64, y: usize, z: usize) -> Vec<Vec<Vector>> {
    let mut rays = vec![vec![Vector::new(vec![1.0, 0.0, 0.0]); y]; z];

    let y_rays_df = rays_df(1, yfov, y);
    for c in 0..(y / 2) {
        let df = y_rays_df[c];
        for r in 0..z {
            *rays[r][c].at_mut(1) += df;
        }
    }
    for c in ((y + 1) / 2)..y {
        let df = y_rays_df[y - 1 - c];
        for r in 0..z {
            *rays[r][c].at_mut(1) -= df;
        }
    }

    let z_rays_df = rays_df(2, zfov, z);
    for r in 0..(z / 2) {
        let df = z_rays_df[r];
        for c in 0..y {
            *rays[r][c].at_mut(2) += df;
        }
    }
    for r in ((z + 1) / 2)..z {
        let df = z_rays_df[z - 1 - r];
        for c in 0..y {
            *rays[r][c].at_mut(2) -= df;
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
