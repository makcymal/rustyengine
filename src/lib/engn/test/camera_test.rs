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
    let game = Game::from(conf);
    let camera = game.camera();
    let rays = camera.incepted_rays(game.canvas.height, game.canvas.width).unwrap();
    let directions = Grid::from_double(vec![
        vec![Vector::col(vec![1.0, -1.0, 1.0]),
             Vector::col(vec![1.0, 0.0, 1.0]),
             Vector::col(vec![1.0, 1.0, 1.0])],
        vec![Vector::col(vec![1.0, -1.0, 0.0]),
             Vector::col(vec![1.0, 0.0, 0.0]),
             Vector::col(vec![1.0, 1.0, 0.0])],
        vec![Vector::col(vec![1.0, -1.0, -1.0]),
             Vector::col(vec![1.0, 0.0, -1.0]),
             Vector::col(vec![1.0, 1.0, -1.0])],
    ]);
    assert_eq!(rays.directions, directions)
}


// #[test]
// fn right_rect_rays_iter_dir() {
//     let mut conf = Conf::default();
//     conf.scr_height = 4;
//     conf.scr_width = 4;
//     conf.camera_fov = PI / 2.0;
//     let game = Game::from(conf);
//     let camera = game.camera();
//     let rays = camera.rect_rays_iter(game.canvas.height, game.canvas.width).unwrap();
//     let dir = Vector::col(vec![1.0, -1.0, 1.0]);
//     assert!(rays.dir.coord().aeq(dir.coord()));
// }


// #[test]
// fn wide_rect_rays_iter_ldir() {
//     let mut conf = Conf::default();
//     conf.scr_height = 3;
//     conf.scr_width = 5;
//     conf.camera_fov = 3.0 * PI / 4.0;
//     let game = Game::from(conf);
//     let camera = game.camera();
//     let rays = camera.rect_rays_iter(game.canvas.height, game.canvas.width).unwrap();
//     let ldir = Vector::col(vec![1.0, -2.414213562373095, 0.8540806854634666]);
//     assert!(rays.ldir.coord().aeq(ldir.coord()));
// }


// #[test]
// fn tilted_rect_rays_iter_ldir() {
//     let mut conf = Conf::default();
//     conf.scr_height = 3;
//     conf.scr_width = 5;
//     conf.camera_dir = Vector::col(vec![
//         (PI / 6.0).cos() * (PI / 4.0).cos(),
//         -(PI / 6.0).cos() * (PI / 4.0).sin(),
//         (PI / 6.0).sin()]);
//     conf.camera_fov = 2.0 * PI / 3.0;
//     let game = Game::from(conf);
//     let camera = game.camera();
//     let rays = camera.rect_rays_iter(game.canvas.height, game.canvas.width).unwrap();
//     let lmv = Vector::col(vec![
//         -(PI / 4.0).sin() * (PI / 3.0).tan(),
//         -(PI / 4.0).cos() * (PI / 3.0).tan(),
//         0.0
//     ]);
//     dbg!(&lmv);
//     assert!(rays.ldir.coord().aeq(ldir.coord()));
// }
