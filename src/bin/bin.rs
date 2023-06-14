#![allow(warnings)]

mod labyrinth;

use {
    anyhow::Result,
    rustyengine::{
        conf::*,
        engn::{Game, MovementEvent, MovementEventSys},
    },
    crate::labyrinth::{
        scene::{Scene, STEP},
    }
};


fn main() -> Result<()> {
    let conf = Conf::read(vec!["src/bin/conf.toml"])?;
    let mut scene = Scene::new(conf.draw_dist);
    scene.expand();
    let es = MovementEventSys::new(STEP);
    let mut game = Game::<MovementEvent<Scene>, MovementEventSys, Scene>::new(conf, scene, es)?;
    game.run()?;
    Ok(())
}
