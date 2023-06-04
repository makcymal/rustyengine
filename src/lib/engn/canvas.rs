use {
    super::*,
    crate::{
        errs::{
            ReRes,
            ReErr::{self, *},
        },
        math::*,
        grid::*,
    },
};

#[derive(Debug)]
pub struct Canvas {
    go: GameObject,
    pub scr_y: usize,
    pub scr_x: usize,
    dist: Matrix,
}

impl Canvas {
    pub fn new(go: GameObject, scr_y: usize, scr_x: usize) -> Self {
        Self {
            go,
            scr_y,
            scr_x,
            dist: Matrix::zero(scr_y, scr_x),
        }
    }

    pub fn update(&mut self, camera: &Camera, entities: &EntityList) -> ReRes<()> {

        Ok(())
    }
}