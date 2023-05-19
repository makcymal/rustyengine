use std::f64::consts::PI;
use {
    crate::{
        engn::*,
        math::*,
    },
};


#[derive(Debug, Clone, PartialEq)]
pub struct Conf {
    pub biform: Matrix,
    pub basis: Matrix,
    pub initpt: Point,
    pub camera_dir: Matrix,
    pub camera_roll: f64,
    pub camera_fov: f64,
    pub draw_dist: f64,
    pub scr_height: usize,
    pub scr_width: usize,
}

impl Default for Conf {
    fn default() -> Self {
        let mut camera_dir = Matrix::col(vec![0.0; 3]);
        *camera_dir.at_mut(0) = 1.0;
        Self {
            biform: Matrix::identity(3),
            basis: Matrix::identity(3).to_multicol(),
            initpt: Point::new(vec![0.0; 3]),
            camera_dir,
            camera_roll: 0.0,
            camera_fov: PI / 2.0,
            draw_dist: 100.0,
            scr_height: 200,
            scr_width: 200,
        }
    }
}
