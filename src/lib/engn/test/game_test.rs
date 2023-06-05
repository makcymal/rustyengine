use {
    super::super::*,
    crate::{conf::*, math::*},
};

#[test]
fn given_vert_fov() {
    let mut conf = Conf::default();
    conf.hfov = Some(1.0);
    let game = Game::new(conf);
    assert_eq!(game.unwrap().camera.hfov, 1.0);
}

#[test]
fn computed_vert_fov() {
    let mut conf = Conf::default();
    conf.wfov = 2.0;
    conf.wscr = 200;
    conf.hscr = 100;
    let game = Game::new(conf);
    assert_eq!(game.unwrap().camera.hfov, 1.0);
}

#[test]
fn plane_straight_intersect() {
    let mut game = Game::default();
    let plane = HypePlane::new(
        game.core(),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 0.0, 0.0]),
    )
    .unwrap();
    let dist = plane.intersect(
        &game.cs,
        &Point::default(),
        &Vector::new(vec![1.0, 0.0, 0.0]),
    );
    assert_eq!(dist, 3.0);
}

#[test]
fn curve_plane_straight_intersect() {
    let mut game = Game::default();
    let plane = HypePlane::new(
        game.core(),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 1.0, 0.0]),
    )
    .unwrap();
    let dist = plane.intersect(
        &game.cs,
        &Point::default(),
        &Vector::new(vec![1.0, 0.0, 0.0]),
    );
    assert_eq!(dist, 3.0);
}

#[test]
fn straight_plane_curve_intersect() {
    let mut game = Game::default();
    let plane = HypePlane::new(
        game.core(),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 0.0, 0.0]),
    )
    .unwrap();
    let dist = plane.intersect(
        &game.cs,
        &Point::default(),
        &Vector::new(vec![1.0, 1.0, 0.0]),
    );
    assert_eq!(dist, 3.0);
}

#[test]
fn horizontal_plane_curve_intersect() {
    let mut game = Game::default();
    let plane = HypePlane::new(
        game.core(),
        Point::default(),
        Vector::new(vec![0.0, 0.0, 1.0]),
    )
    .unwrap();
    let dist = plane.intersect(
        &game.cs,
        &Point::new(vec![0.0, 0.0, 1.0]),
        &Vector::new(vec![3.0, -1.0, -2.0]),
    );
    assert_eq!(dist, 0.5);
}

#[test]
fn horizontal_plane_no_intersect() {
    let mut game = Game::default();
    let plane = HypePlane::new(
        game.core(),
        Point::default(),
        Vector::new(vec![0.0, 0.0, 1.0]),
    )
    .unwrap();
    let dist = plane.intersect(
        &game.cs,
        &Point::new(vec![0.0, 0.0, 1.0]),
        &Vector::new(vec![3.0, -1.0, 2.0]),
    );
    assert_eq!(dist, -1.0);
}
