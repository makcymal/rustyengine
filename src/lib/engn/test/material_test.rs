use std::cell::RefCell;
use {
    super::super::*,
    crate::{conf::*, engn::*, math::*},
    std::{any::Any, rc::Rc},
};

#[test]
fn id_generate() {
    let mut id_pool = IdPool::new();
    id_pool.generate();
    assert_eq!(id_pool.len(), 1);
}

#[test]
fn entity_first_prop() {
    let mut id_pool = IdPool::new();
    let mut core = Entity::new(id_pool.generate());
    core.set_prop("drawdist", Box::new(10.0));
    assert_eq!(
        core.get_prop("drawdist")
            .unwrap()
            .downcast_ref::<f64>()
            .unwrap(),
        &10.0
    );
}

#[test]
fn entity_second_prop() {
    let mut id_pool = IdPool::new();
    let mut core = Entity::new(id_pool.generate());
    core.set_prop("drawdist", Box::new(10.0));
    core.set_prop("drawdist", Box::new(20.0));
    assert_eq!(
        core.get_prop("drawdist")
            .unwrap()
            .downcast_ref::<f64>()
            .unwrap(),
        &20.0
    );
}

#[test]
fn entity_list_get() {
    set_biform_identity();
    let mut list = EntityList::new();
    let id = IdPool::new().generate();
    list.append(Rc::new(RefCell::new(
        HypePlane::new(
            Entity::new(id.clone()),
            Point::new(vec![1.0, 1.0, 1.0]),
            Vector::new(vec![1.0, 1.0, 1.0]),
        )
        .unwrap(),
    )));
    assert_eq!(
        list.get(&id).unwrap().borrow().collide(
            &CoordSys::default(),
            &Point::new(vec![2.0, 2.0, 2.0]),
            &Vector::new(vec![-1.0, -1.0, -1.0])
        ),
        1.0
    );
}

#[test]
fn hype_plane_pos() {
    let mut id_pool = IdPool::new();
    let mut core = Entity::new(id_pool.generate());

    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    let dir = Vector::new(vec![1.0, 1.0, 1.0]);
    let mut hype = HypePlane::new(core, pos, dir).unwrap();
    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    assert_eq!(hype.initpt, pos);
}

#[test]
fn hype_plane_mv_pos() {
    let mut id_pool = IdPool::new();
    let mut core = Entity::new(id_pool.generate());

    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    let dir = Vector::new(vec![1.0, 1.0, 1.0]);
    let mut hype = HypePlane::new(core, pos, dir).unwrap();
    let mv = Vector::new(vec![2.0, 2.0, 2.0]);
    hype.mv(&mv).unwrap();

    let pos = Point::new(vec![3.0, 3.0, 3.0]);
    assert_eq!(hype.initpt, pos);
}

#[test]
fn plane_straight_collision() {
    let cs = CoordSys::default();
    let mut id_pool = IdPool::new();
    let plane = HypePlane::new(
        Entity::new(id_pool.generate()),
        Point::new(vec![3.0, 0.0, 0.0]),
        Vector::new(vec![1.0, 0.0, 0.0]),
    )
    .unwrap();
    let dist = plane.collide(&cs, &Point::default(), &Vector::new(vec![1.0, 0.0, 0.0]));
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
    )
    .unwrap();
    let dist = plane.collide(&cs, &Point::default(), &Vector::new(vec![1.0, 0.0, 0.0]));
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
    )
    .unwrap();
    let dist = plane.collide(&cs, &Point::default(), &Vector::new(vec![1.0, 1.0, 0.0]));
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
    )
    .unwrap();
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
    )
    .unwrap();
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
        vec![1.0, 2.0, 3.0],
    )
    .unwrap();
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
        vec![3.0, 3.0, 3.0],
    )
    .unwrap();
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
        vec![2.0, 2.0, 2.0],
    )
    .unwrap();
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
        vec![4.0, 1.0, 1.0],
    )
    .unwrap();
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
        vec![4.0, 1.0, 1.0],
    )
    .unwrap();
    let dist = ellipse.collide(
        &cs,
        &Point::new(vec![3.0, 2.0, 2.0]),
        &Vector::new(vec![0.0, -1.0, -1.0]),
    );
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
        vec![4.0, 1.0, 1.0],
    )
    .unwrap();
    let dist = ellipse.collide(
        &cs,
        &Point::new(vec![3.0, 1.0, 2.0]),
        &Vector::new(vec![0.0, -1.0, -1.0]),
    );
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
        vec![4.0, 1.0, 1.0],
    )
    .unwrap();
    let dist = ellipse.collide(
        &cs,
        &Point::new(vec![3.0, 1.0, 2.0]),
        &Vector::new(vec![0.0, -1.0, 1.0]),
    );
    assert_eq!(dist, -1.0);
}
