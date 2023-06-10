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
pub struct Canvas<Scn: AsScene> {
    phantom: PhantomData<Scn>,
    size: (usize, usize),
    charmap: Vec<char>,
    charmap_len: f32,
    draw_dist: f32,
    picture: Vec<String>,
}

impl<Scn: AsScene> Canvas<Scn> {
    /// Constructs new canvas
    pub fn new(size: (usize, usize), charmap: String, draw_dist: f32) -> Self {
        Self {
            phantom: PhantomData,
            size,
            charmap: charmap.chars().collect(),
            charmap_len: charmap.len() as f32,
            draw_dist: draw_dist + 1.0,
            picture: vec![(0..size.1).map(|_| ' ').collect::<String>(); size.0],
        }
    }

    /// Updates picture via colliding entities against all camera rays
    pub fn update(&mut self, camera: &Camera, scene: &Scn) -> ReRes<()> {
        for r in 0..self.size.0 {
            self.picture[r] = (0..self.size.1)
                .map(|c| {
                    let (ray, len) = camera.ray(r, c);
                    let mut dist = scene.collide(&camera.pos, ray) * len;
                    if dist < 0.0 || camera.draw_dist < dist {
                        dist = camera.draw_dist;
                    }
                    // dbg!(dist, self.draw_dist, self.charmap_len, (dist / self.draw_dist * self.charmap_len));
                    self.charmap[(dist / self.draw_dist * self.charmap_len).floor() as usize]
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
        println!("{}", &msg[..self.size.1]);
        thread::sleep(timeout);
        console::clear();
        Ok(())
    }

    /// Shows message on the second line of console
    pub fn notification(&self, msg: &str) -> ReRes<()> {
        let col = self.size.1.saturating_sub(msg.len()) / 2;
        console::move_cursor(1, col as u16)?;

        println!("{}", &msg[..self.size.1]);
        Ok(())
    }
}
