use {
    super::super::*,
    crate::{conf::*, math::*},
};

#[test]
fn given_vert_fov() {
    let mut conf = Conf::default();
    conf.hfov = Some(1.0);
    let game = Game::<ConsoleEvent, EventQueue<ConsoleEvent, EntityList>, EntityList>::new(conf);
    assert_eq!(game.unwrap().camera.hfov, 1.0);
}

#[test]
fn computed_vert_fov() {
    let mut conf = Conf::default();
    conf.wfov = 2.0;
    conf.wscr = 200;
    conf.hscr = 100;
    assert_eq!(conf.comp_hfov(), 1.0);
}

#[test]
fn plane_straight_collision() {
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let plane = HypePlane::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 0.0, 0.0]),
    ).unwrap();
    let dist = plane.collide(
        &cs,
        &Point::default(),
        &Vector::new(vec![1.0, 0.0, 0.0]),
    );
    assert_eq!(dist, 3.0);
}

#[test]
fn curve_plane_straight_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let plane = HypePlane::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 1.0, 0.0]),
    ).unwrap();
    let dist = plane.collide(
        &cs,
        &Point::default(),
        &Vector::new(vec![1.0, 0.0, 0.0]),
    );
    assert_eq!(dist, 3.0);
}

#[test]
fn straight_plane_curve_collision() {
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let plane = HypePlane::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 0.0, 0.0]),
    ).unwrap();
    let dist = plane.collide(
        &cs,
        &Point::default(),
        &Vector::new(vec![1.0, 1.0, 0.0]),
    );
    assert_eq!(dist, 3.0);
}

#[test]
fn horizontal_plane_curve_collision() {
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let plane = HypePlane::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![0.0, 0.0, 1.0]),
    ).unwrap();
    let dist = plane.collide(
        &cs,
        &Point::new(vec![0.0, 0.0, 1.0]),
        &Vector::new(vec![3.0, -1.0, -2.0]),
    );
    assert_eq!(dist, 0.5);
}

#[test]
fn horizontal_plane_no_collision() {
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let plane = HypePlane::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![0.0, 0.0, 1.0]),
    ).unwrap();
    let dist = plane.collide(
        &cs,
        &Point::new(vec![0.0, 0.0, 1.0]),
        &Vector::new(vec![3.0, -1.0, 2.0]),
    );
    assert_eq!(dist, -1.0);
}

#[test]
fn hype_ellipse_sphere_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![1.0, 2.0, 3.0]
    ).unwrap();
    let dist = ellipse.collide(&cs, &Point::default(), &Vector::new(vec![1.0, 0.0, 0.0]));
    assert_eq!(dist, 2.0);
}

#[test]
fn hype_ellipse_sphere_inception_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![3.0, 3.0, 3.0]
    ).unwrap();
    let dist = ellipse.collide(&cs, &Point::default(), &Vector::new(vec![0.0, 1.0, 0.0]));
    assert_eq!(dist, 0.0);
}

#[test]
fn hype_ellipse_sphere_no_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![2.0, 2.0, 2.0]
    ).unwrap();
    let dist = ellipse.collide(&cs, &Point::default(), &Vector::new(vec![1.0, 1.0, 0.0]));
    assert_eq!(dist, -1.0);
}

#[test]
fn hype_ellipse_hot_dog_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![4.0, 1.0, 1.0]
    ).unwrap();
    let dist = ellipse.collide(&cs, &Point::default(), &Vector::new(vec![1.0, 0.0, 0.0]));
    assert_eq!(dist, 7.0);
}

#[test]
fn hype_ellipse_hot_dog_curve_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![4.0, 1.0, 1.0]
    ).unwrap();
    let dist = ellipse.collide(
        &cs,
        &Point::new(vec![3.0, 2.0, 2.0]),
        &Vector::new(vec![0.0, -1.0, -1.0]));
    assert_eq!(dist, (8.0_f64.sqrt() - 1.0) / 2.0_f64.sqrt());
}

#[test]
fn hype_ellipse_hot_dog_angled_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![4.0, 1.0, 1.0]
    ).unwrap();
    let dist = ellipse.collide(
        &cs,
        &Point::new(vec![3.0, 1.0, 2.0]),
        &Vector::new(vec![0.0, -1.0, -1.0]));
    assert_eq!(dist, 1.0);
}

#[test]
fn hype_ellipse_hot_dog_no_collision() {
    set_biform_identity();
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let ellipse = HypeEllipse::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Basis::default(),
        vec![4.0, 1.0, 1.0]
    ).unwrap();
    let dist = ellipse.collide(
        &cs,
        &Point::new(vec![3.0, 1.0, 2.0]),
        &Vector::new(vec![0.0, -1.0, 1.0]));
    assert_eq!(dist, -1.0);
}
