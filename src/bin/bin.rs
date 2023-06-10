#![allow(warnings)]

mod labyrinth;

use {
    anyhow::Result,
    rustyengine::{
        conf::*,
        engn::{Game, MovementEvent, MovementEventSys},
    },
    crate::labyrinth::{
        scene::Scene,
    }
};


fn main() -> Result<()> {
    let conf = Conf::read(vec!["src/bin/conf.toml"])?;
    let scene = Scene::new();
    let mut game = Game::<MovementEvent<Scene>, MovementEventSys, Scene>::new(conf, scene)?;
    game.run()?;
    Ok(())
}
