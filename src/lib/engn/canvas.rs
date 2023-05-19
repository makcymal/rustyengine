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
    scr_height: usize,
    scr_width: usize,
    dist: Matrix,
}

impl Canvas {
    pub fn new(go: GameObject, scr_height: usize, scr_width: usize) -> Self {
        Self {
            go,
            scr_height,
            scr_width,
            dist: Matrix::zero(scr_height, scr_width),
        }
    }

    pub fn update(&mut self, camera: &Camera, entities: &EntityList) -> ReRes<()> {
        let rays = camera.incepted_rays(self.scr_height, self.scr_width)?;
        for r in 0..self.scr_height {
            for c in 0..self.scr_width {
                for ent in entities.entities.iter() {
                    let dist = ent.borrow().intersect(&rays.inc, rays.dir_att(r, c), rays.len_att(r, c))?;
                    *self.dist.att_mut(r, c) = if dist > camera.get_draw_dist()? {
                        -1.0
                    } else {
                        dist
                    };
                }
            }
        }
        Ok(())
    }
}