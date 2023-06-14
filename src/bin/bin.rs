#![allow(warnings)]

mod labyrinth;

use {
    crate::labyrinth::{
        scene::Scene,
    },
    anyhow::Result,
    rustyengine::prelude::*,
};
use rustyengine::engn::{MovementEvent, MovementEventSys};
use crate::labyrinth::scene::{gen_init_pos, STEP};

fn main() -> Result<()> {
    let mut conf = Conf::read(vec!["src/bin/conf.toml"])?;
    conf.initpt = gen_init_pos();
    let mut scene = Scene::new(conf.draw_dist)?;
    scene.expand();
    let es = MovementEventSys::new(STEP);
    let mut game = Game::<MovementEvent<Scene>, MovementEventSys, Scene>::new(conf, scene, es)?;
    game.run()?;
    Ok(())
}
