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
    let mut game = Game::<Action, DedupActions, Scene>::new(conf)?;
    let scene = Scene::new(&mut game.id_pool);
    game.set_entities(scene);

    game.run()?;

    Ok(())
}
