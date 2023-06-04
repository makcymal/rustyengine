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
fn initpt_from_conf() {
    let conf = Conf::read(vec!["src/lib/conf/test/conf1.toml"]).unwrap();
    let initpt = Point::new(vec![1.0, 1.0, 1.0]);
    assert_eq!(conf.initpt, initpt);
}

#[test]
fn conf_initpt_double_assignment() {
    let conf =
        Conf::read(vec!["src/lib/conf/test/conf1.toml", "src/lib/conf/test/conf2.toml"])
        .unwrap();
    let initpt = Point::new(vec![2.0, 2.0, 2.0]);
    assert_eq!(conf.initpt, initpt);
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
    assert_eq!(conf.wfov, std::f64::consts::FRAC_PI_2);
}

#[test]
fn conf_default_double_assignment() {
    let conf =
        Conf::read(vec!["src/lib/conf/test/conf1.toml", "src/lib/conf/test/conf2.toml"])
        .unwrap();
    assert_eq!(conf.wscr, 100);
}
