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
