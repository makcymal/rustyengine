#![allow(warnings)]

mod labyrinth;

use std::time::Duration;
use {
    crate::labyrinth::scene::*,
    anyhow::Result,
    rustyengine::{
        conf::*,
        engn::{Game, MovementEvent, MovementEventSys},
    },
};

fn main() -> Result<()> {
    let mut conf = Conf::read(vec!["C:/Users/makcym/conf.toml"])?;
    conf.initpt = gen_init_pos();
    let mut scene = Scene::new(conf.draw_dist);
    scene.expand();
    let es = MovementEventSys::new(STEP);
    let mut game = Game::<MovementEvent<Scene>, MovementEventSys, Scene>::new(conf, scene, es)?;
    game.run()?;
    Ok(())
}
