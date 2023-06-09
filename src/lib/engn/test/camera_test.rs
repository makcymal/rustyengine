use {
    super::super::{camera::rays, camera::rays_df, *},
    crate::{conf::*, grid::*, math::*},
    std::f64::consts::{FRAC_PI_4, FRAC_PI_8, PI},
};

#[test]
fn xy_rays_df_discr_3() {
    let df = vec![1.0];
    assert_eq!(rays_df(1, PI / 2.0, 3), df)
}

#[test]
fn xy_rays_df_discr_5() {
    let df = vec![(PI / 6.0).tan(), (PI / 12.0).tan()];
    assert!(aeq(&rays_df(1, PI / 3.0, 5)[1], &df[1]));
}

#[test]
fn xz_rays_df_discr_4() {
    let df = vec![(PI / 8.0).tan(), (PI / 24.0).tan()];
    assert!(aeq(&rays_df(2, PI / 4.0, 4)[0], &df[0]));
}

#[test]
fn xz_rays_df_discr_4_closest() {
    let df = vec![(PI / 8.0).tan(), (PI / 24.0).tan()];
    assert!(aeq(&rays_df(2, PI / 4.0, 4)[1], &df[1]));
}

#[test]
fn rays_discr_3_3_att_0_2() {
    let rays = rays(PI / 2.0, PI / 2.0, 3, 3);
    assert_eq!(rays.att(0, 2), &Vector::new(vec![1.0, -1.0, 1.0]));
}

#[test]
fn rays_discr_4_3_att_1_1() {
    let rays = rays(PI / 2.0, PI / 2.0, 4, 3);
    assert!(rays
        .att(1, 1)
        .coord
        .aeq(&Vector::new(vec![1.0, (PI / 12.0).tan(), 0.0]).coord));
}

#[test]
fn rays_discr_6_4_att_3_4() {
    let rays = rays(2.0 * PI / 3.0, PI / 4.0, 6, 4);
    let res = Vector::new(vec![1.0, -(PI / 5.0).tan(), -(PI / 8.0).tan()]).coord;
    assert!(rays.att(3, 4).coord.aeq(&res));
}

#[test]
fn rays_discr_6_5_att_center() {
    let rays = rays(2.0 * PI / 3.0, PI / 4.0, 6, 5);
    let res = Vector::new(vec![1.0, -(PI / 15.0).tan(), 0.0]).coord;
    assert!(rays.att(2, 3).coord.aeq(&res));
}

#[test]
fn mv_camera_about_pos() {
    let mut id_pool = IdPool::new();
    let mut camera = Camera::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![1.0, 0.0, 0.0]),
        500.0,
        2.0 * PI / 3.0,
        PI / 4.0,
        6,
        5,
    );
    camera.dir(&Vector::new(vec![1.0, 0.0, 0.0])).unwrap();
    assert_eq!(camera.pos, Point::new(vec![1.0, 0.0, 0.0]))
}

#[test]
fn mv_camera_about_rays() {
    let mut id_pool = IdPool::new();
    let mut camera = Camera::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![1.0, 0.0, 0.0]),
        500.0,
        2.0 * PI / 3.0,
        PI / 4.0,
        6,
        5,
    );
    camera.dir(&Vector::new(vec![1.0, 0.0, 0.0])).unwrap();
    assert!(camera
        .rays
        .att(2, 3)
        .coord
        .aeq(&Vector::new(vec![1.0, -(PI / 15.0).tan(), 0.0]).coord))
}

#[test]
fn rotate_camera_about_dir() {
    let mut id_pool = IdPool::new();
    let mut camera = Camera::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![1.0, 0.0, 0.0]),
        500.0,
        2.0 * PI / 3.0,
        PI / 4.0,
        6,
        5,
    );
    camera.planar_rotate(0, 1, -PI / 4.0).unwrap();
    assert_eq!(
        camera.dir,
        Vector::new(vec![FRAC_PI_4.cos(), -FRAC_PI_4.sin(), 0.0])
    );
}

#[test]
fn rotate_camera_twice_about_dir() {
    let mut id_pool = IdPool::new();
    let mut camera = Camera::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![1.0, 0.0, 0.0]),
        500.0,
        2.0 * PI / 3.0,
        PI / 4.0,
        6,
        5,
    );
    camera.planar_rotate(0, 1, -PI / 4.0).unwrap();
    camera.planar_rotate(0, 1, PI / 8.0).unwrap();
    assert_eq!(
        camera.dir,
        Vector::new(vec![FRAC_PI_8.cos(), -FRAC_PI_8.sin(), 0.0])
    );
}

#[test]
fn rotate_camera_about_rays() {
    let mut id_pool = IdPool::new();
    let mut camera = Camera::new(
        Entity::new(id_pool.generate()),
        Point::default(),
        Vector::new(vec![1.0, 0.0, 0.0]),
        500.0,
        PI / 2.0,
        PI / 2.0,
        4,
        3,
    );
    camera.planar_rotate(0, 2, PI / 4.0).unwrap();
    let res = Matrix::col(vec![0.0, -1.0, 2.0_f64.sqrt()]);
    assert!(camera.rays.att(0, 3).coord.aeq(&res));
}
