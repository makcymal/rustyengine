use {
    crate::{
        engn::*,
        errs::{
            GameErr::{self, *},
            ReErr::{self, *},
            ReRes,
        },
        grid::*,
        math::*,
    },
    either::Either,
    std::{cmp::min, marker::PhantomData, str::Chars, thread, time::Duration},
};

/// Stores picture as `Vec<String>` respectively to `charmap` given in the `Conf`
#[derive(Debug)]
pub struct Canvas<Scn: AsScene> {
    phantom: PhantomData<Scn>,
    size: (usize, usize),
    charcoal: Charcoal,
    picture: Vec<String>,
}

impl<Scn: AsScene> Canvas<Scn> {
    /// Constructs new canvas
    pub fn new(size: (usize, usize), chars: String, draw_dist: f64) -> Self {
        Self {
            phantom: PhantomData,
            size,
            charcoal: Charcoal::new(chars, draw_dist),
            picture: vec![(0..size.1).map(|_| ' ').collect::<String>(); size.0],
        }
    }

    /// Updates picture via colliding entities against all camera rays
    pub fn update(&mut self, camera: &Camera, cs: &CoordSys, scene: &Scn) -> ReRes<()> {
        for r in 0..self.size.0 {
            self.picture[r] = (0..self.size.1)
                .map(|c| {
                    let ray = camera.ray(r, c);
                    match scene.collide(cs, &camera.pos, ray) {
                        Either::Left(d) => self.charcoal.ignite(d),
                        Either::Right(c) => c,
                    }
                })
                .collect::<String>();
        }
        Ok(())
    }

    /// Prints all string to console
    pub fn draw(&self) -> ReRes<()> {
        console::move_cursor(3, 0)?;
        for line in &self.picture {
            println!("{}", line);
        }
        Ok(())
    }

    /// Clears all console and shows one message
    pub fn banner(&self, msg: &str, timeout: Duration) -> ReRes<()> {
        let col = self.size.1.saturating_sub(msg.len()) / 2;
        let row = self.size.0 / 2;
        console::move_cursor(row as u16, col as u16)?;

        console::clear();
        println!("{}", &msg[..min(msg.len(), self.size.1)]);
        thread::sleep(timeout);
        console::clear();
        Ok(())
    }
}
