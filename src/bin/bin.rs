#![allow(warnings)]

mod labyrinth;

use {
    anyhow::Result,
    rustyengine::prelude::*,
    crate::labyrinth::{
        action::{
            Action, DedupActions,
        },
        scene::Scene,
    }
};


fn main() -> Result<()> {
    let conf = Conf::read(vec!["src/bin/conf.toml"])?;
    let scene = Scene::new();
    let mut game = Game::<Action, DedupActions, Scene>::new(conf, scene)?;
    game.run()?;
    Ok(())
}
