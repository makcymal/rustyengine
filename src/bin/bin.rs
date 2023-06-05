#![allow(warnings)]

mod labyrinth;

use {anyhow::Result, labyrinth::console, rustyengine::prelude::*};

fn main() -> Result<()> {
    let mut conf = Conf::read(vec!["src/bin/conf.toml"])?;
    let (cols, rows) = console::init()?;
    (conf.wscr, conf.hscr) = (cols as usize, rows as usize);
    let game = Game::new(conf);

    Ok(())
}
