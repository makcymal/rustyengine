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
    std::{
        marker::PhantomData,
        str::Chars, thread, time::Duration,
    },
};


/// Stores picture as `Vec<String>` respectively to `charmap` given in the `Conf`
#[derive(Debug)]
pub struct Canvas<Lst: AsMaterialList> {
    phantom: PhantomData<Lst>,
    wscr: usize,
    hscr: usize,
    charmap: Vec<char>,
    charmap_len: f64,
    picture: Vec<String>,
}

impl<Lst: AsMaterialList> Canvas<Lst> {
    /// Constructs new canvas
    pub fn new(wscr: usize, hscr: usize, charmap: String) -> Self {
        Self {
            phantom: PhantomData,
            wscr,
            hscr,
            charmap: charmap.chars().collect(),
            charmap_len: charmap.len() as f64,
            picture: vec![(0..wscr).map(|_| ' ').collect::<String>(); hscr],
        }
    }

    /// Updates picture via colliding entities against all camera rays
    pub fn update(&mut self, camera: &Camera, cs: &CoordSys, entities: &Lst) -> ReRes<()> {
        for r in 0..self.hscr {
            self.picture[r] = (0..self.wscr)
                .map(|c| {
                    let mut dist = entities.collide(cs, camera.pos(), camera.rays.att(r, c));
                    if dist < 0.0 || camera.draw_dist < dist {
                        dist = camera.draw_dist;
                    }
                    self.charmap[(dist / camera.draw_dist * self.charmap_len).round() as usize]
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
        let col = self.wscr.saturating_sub(msg.len()) / 2;
        let row = self.hscr / 2;
        console::move_cursor(row as u16, col as u16)?;

        console::clear();
        println!("{}", &msg[..self.wscr]);
        thread::sleep(timeout);
        console::clear();
        Ok(())
    }

    /// Shows message on the second line of console
    pub fn notification(&self, msg: &str) -> ReRes<()> {
        let col = self.wscr.saturating_sub(msg.len()) / 2;
        console::move_cursor(1, col as u16)?;

        println!("{}", &msg[..self.wscr]);
        Ok(())
    }
}
