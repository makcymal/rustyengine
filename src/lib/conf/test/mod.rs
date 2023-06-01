use {
    super::Conf,
    crate::math::*,
};


#[test]
fn drawdist_from_conf() {
    let conf = Conf::read(vec!["src/lib/conf/test/conf1.toml"]).unwrap();
    assert_eq!(conf.draw_dist, 1.0);
}

#[test]
fn biform_from_conf() {
    let conf = Conf::read(vec!["src/lib/conf/test/conf1.toml"]).unwrap();
    let biform = Matrix::identity(3).num_mul_assign(2.0);
    assert_eq!(conf.biform, biform);
}

#[test]
fn conf_biform_double_assignment() {
    let conf =
        Conf::read(vec!["src/lib/conf/test/conf1.toml", "src/lib/conf/test/conf2.toml"])
        .unwrap();
    let biform = Matrix::identity(3).num_mul_assign(2.0);
    assert_eq!(conf.biform, biform);
}

#[test]
fn conf_drawdist_double_assignment() {
    let conf =
        Conf::read(vec!["src/lib/conf/test/conf1.toml", "src/lib/conf/test/conf2.toml"])
        .unwrap();
    assert_eq!(conf.draw_dist, 2.0);
}

#[test]
fn conf_fov_double_assignment() {
    let conf =
        Conf::read(vec!["src/lib/conf/test/conf1.toml", "src/lib/conf/test/conf2.toml"])
        .unwrap();
    assert_eq!(conf.camera_fov, 1.57);
}

#[test]
fn conf_default_double_assignment() {
    let conf =
        Conf::read(vec!["src/lib/conf/test/conf1.toml", "src/lib/conf/test/conf2.toml"])
        .unwrap();
    assert_eq!(conf.initpt, Point::new(vec![0.0; 3]));
}
