use {
    super::super::*,
    crate::{
        conf::*,
        math::*,
    },
};


#[test]
fn incepted_rays_from_initpt() {
    let game = Game::default();
    let camera = game.camera();
    let rays = camera.incepted_rays(game.canvas.height, game.canvas.width).unwrap();
    dbg!(rays.directions);
}
