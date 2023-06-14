#![allow(warnings)]

mod labyrinth;

use {
    crate::labyrinth::scene::*,
    anyhow::Result,
    rustyengine::{conf::*, engn::*},
};

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
