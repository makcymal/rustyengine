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
fn entity_core_first_prop() {
    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut core = Entity::new(&id);
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
fn entity_core_second_prop() {
    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut core = Entity::new(&id);
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
fn hype_plane_pos() {
    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut core = Entity::new(&id);

    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    let dir = Vector::new(vec![1.0, 1.0, 1.0]);
    let mut hype = HypePlane::new(core, pos, dir).unwrap();
    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    assert_eq!(hype.initpt, pos);
}

#[test]
fn hype_plane_mv_pos() {
    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut core = Entity::new(&id);

    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    let dir = Vector::new(vec![1.0, 1.0, 1.0]);
    let mut hype = HypePlane::new(core, pos, dir).unwrap();
    let mv = Vector::new(vec![2.0, 2.0, 2.0]);
    hype.mv(&mv).unwrap();

    let pos = Point::new(vec![3.0, 3.0, 3.0]);
    assert_eq!(hype.initpt, pos);
}
