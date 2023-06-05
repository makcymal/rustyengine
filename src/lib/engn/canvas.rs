use {
    super::*,
    crate::{
        errs::{
            ReErr::{self, *},
            ReRes,
        },
        grid::*,
        math::*,
    },
};

#[derive(Debug)]
pub struct Canvas {
    pub wscr: usize,
    pub hscr: usize,
    dist: Matrix,
}

impl Canvas {
    pub fn new(wscr: usize, hscr: usize) -> Self {
        Self {
            wscr,
            hscr,
            dist: Matrix::zero(wscr, hscr),
        }
    }

    pub fn update(&mut self, camera: &Camera, entities: &EntityList) -> ReRes<()> {
        Ok(())
    }
}
