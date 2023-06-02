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
    pub height: usize,
    pub width: usize,
    dist: Matrix,
}

impl Canvas {
    pub fn new(go: GameObject, scr_height: usize, scr_width: usize) -> Self {
        Self {
            go,
            height: scr_height,
            width: scr_width,
            dist: Matrix::zero(scr_height, scr_width),
        }
    }

    pub fn update(&mut self, camera: &Camera, entities: &EntityList) -> ReRes<()> {

        Ok(())
    }
}