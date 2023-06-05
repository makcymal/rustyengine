#![allow(warnings)]

mod labyrinth;

use {
    rustyengine::prelude::*,
    labyrinth::{
        console,
    },
    anyhow::Result,
};

fn main() -> Result<()> {
    let mut conf = Conf::read(vec!["src/bin/conf.toml"])?;
    let (cols, rows) = console::init()?;
    (conf.wscr, conf.hscr) = (cols as usize, rows as usize);
    let game = Game::new(conf);

    Ok(())
}
