use {
    super::super:: {
        *,
        camera::rays_df,
        camera::rays,
    },
    crate::{
        conf::*,
        math::*,
        grid::*,
    },
    std::f64::consts::PI,
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
    dbg!(&rays_df(2, PI / 4.0, 4)[0], &df[0]);
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
    assert!(rays.att(1, 1).coord.aeq(&Vector::new(vec![1.0, (PI / 12.0).tan(), 0.0]).coord));
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
