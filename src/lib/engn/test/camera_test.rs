use {
    super::super::*,
    crate::{
        conf::*,
        math::*,
        grid::*,
    },
    std::f64::consts::PI,
};


#[test]
fn incepted_rays_from_initpt() {
    let mut conf = Conf::default();
    conf.scr_height = 3;
    conf.scr_width = 3;
    conf.camera_fov = PI / 2.0;
    let game = Game::new(conf).unwrap();
    let camera = game.camera();
    let rays = camera.incepted_rays(game.canvas.height, game.canvas.width).unwrap();
    let directions = Grid::from_double(vec![
        vec![Vector::new(vec![1.0, -1.0, 1.0]),
             Vector::new(vec![1.0, 0.0, 1.0]),
             Vector::new(vec![1.0, 1.0, 1.0])],
        vec![Vector::new(vec![1.0, -1.0, 0.0]),
             Vector::new(vec![1.0, 0.0, 0.0]),
             Vector::new(vec![1.0, 1.0, 0.0])],
        vec![Vector::new(vec![1.0, -1.0, -1.0]),
             Vector::new(vec![1.0, 0.0, -1.0]),
             Vector::new(vec![1.0, 1.0, -1.0])],
    ]);
    assert_eq!(rays.directions, directions)
}
