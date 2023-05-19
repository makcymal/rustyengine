use {
    super::super::*,
    crate::{
        conf::*,
        engn::*,
        math::*,
    },
    std::{
        any::Any,
        rc::Rc,
    },
};


#[test]
fn id_generate() {
    let mut id_pool = IdPool::new();
    id_pool.generate();
    assert_eq!(id_pool.len(), 1);
}

#[test]
fn prop_feed() {
    let prop = Prop::VFov;
    assert_eq!(prop.feed(), (Prop::VFov.type_id(), 4));
}

#[test]
fn entity_core_first_prop() {
    set_biform_identity();
    let vs = VectorSpace::new(Matrix::identity(3).to_multicol()).unwrap();
    let ip = Point::new(vec![0.0; 3]);
    let cs = CoordSys::new(ip, vs).unwrap();

    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut ec = EntityCore::new(&Rc::new(cs), &id);
    ec.set_prop(Prop::DrawDist, Box::new(10.0));
    assert_eq!(ec.get_prop(Prop::DrawDist).unwrap().downcast_ref::<f64>().unwrap(), &10.0);
}

#[test]
fn entity_core_second_prop() {
    set_biform_identity();
    let vs = VectorSpace::new(Matrix::identity(3).to_multicol()).unwrap();
    let ip = Point::new(vec![0.0; 3]);
    let cs = CoordSys::new(ip, vs).unwrap();

    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut ec = EntityCore::new(&Rc::new(cs), &id);
    ec.set_prop(Prop::DrawDist, Box::new(10.0));
    ec.set_prop(Prop::DrawDist, Box::new(20.0));
    assert_eq!(ec.get_prop(Prop::DrawDist).unwrap().downcast_ref::<f64>().unwrap(), &20.0);
}

#[test]
fn game_object_pos() {
    set_biform_identity();
    let vs = VectorSpace::new(Matrix::identity(3).to_multicol()).unwrap();
    let ip = Point::new(vec![0.0; 3]);
    let cs = CoordSys::new(ip, vs).unwrap();

    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut ec = EntityCore::new(&Rc::new(cs), &id);

    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    let dir = Matrix::from_single(vec![1.0, 1.0, 1.0]).raw_transpose().to_col();
    let mut go = GameObject::new(ec, pos, dir);
    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    assert_eq!(go.core().get_prop(Prop::Pos).unwrap().downcast_ref::<Point>().unwrap(), &pos);
}

#[test]
fn game_object_mv_pos() {
    set_biform_identity();
    let vs = VectorSpace::new(Matrix::identity(3).to_multicol()).unwrap();
    let ip = Point::new(vec![1.0, 1.0, 1.0]);
    let cs = CoordSys::new(ip, vs).unwrap();

    let mut id_pool = IdPool::new();
    let id = id_pool.generate();
    let mut ec = EntityCore::new(&Rc::new(cs), &id);

    let pos = Point::new(vec![1.0, 1.0, 1.0]);
    let dir = Matrix::from_single(vec![1.0, 1.0, 1.0]).raw_transpose().to_col();
    let mut go = GameObject::new(ec, pos, dir);
    let mv = Matrix::from_single(vec![2.0, 2.0, 2.0]).raw_transpose().to_col();
    go.mv(&mv).unwrap();

    let pos = Point::new(vec![3.0, 3.0, 3.0]);
    assert_eq!(go.core().get_prop(Prop::Pos).unwrap().downcast_ref::<Point>().unwrap(), &pos);
}
