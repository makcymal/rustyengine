use {
    super::*,
    crate::{
        conf::Conf,
        math::*,
        math::matrix::set_biform_vec,
    },
    std::{
        rc::Rc,
    },
    crate::errs::{
        ReRes,
        ReErr::{self, *},
    },
};


/// Struct responsible for storing current CoordSys and EntityList and running related scripts
#[derive(Debug)]
pub struct Game {
    pub(in super) cs: Rc<CoordSys>,
    pub(in super) id_pool: IdPool,
    pub(in super) entities: EntityList,
    pub(in super) canvas: Canvas,
    pub(in super) camera: Camera,
}

impl Game {
    /// Constructor that takes CoordSys
    pub fn new(conf: Conf) -> Self {
        Self::from(conf)
    }

    pub fn run() {
        todo!()
    }

    pub fn update() {
        todo!()
    }

    pub fn exit() {
        todo!()
    }

    /// `Ray` in current basis, takes inception `Point` and direction `Vector`
    pub fn game_ray(&self, inc: Point, dir: Vector) -> ReRes<Ray> {
        Ray::new(inc, dir)
    }

    /// `EntityCore` in current basis with appending it's `Uuid` into `IdPool`
    pub fn entity_core(&mut self) -> EntityCore {
        EntityCore::new(&self.cs, &self.id_pool.generate())
    }

    /// `GameObject` in current game, uses `self.entity_core()`
    pub fn game_object(&mut self, pos: Point, dir: Vector) -> GameObject {
        GameObject::new(self.entity_core(), pos, dir)
    }

    /// `Camera` in current game, uses `self.game_object()`
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

impl Default for Game {
    fn default() -> Self {
        let conf = Conf::default();
        Self::from(conf)
    }
}

impl From<Conf> for Game {
    fn from(conf: Conf) -> Self {
        set_biform(conf.biform);
        let cs = Rc::new(
            CoordSys::new(conf.initpt.clone(),
                          VectorSpace::new(conf.basis).unwrap())
                .unwrap());
        let mut id_pool = IdPool::new();
        let entities = EntityList::new();
        let canvas = Canvas::new(
            GameObject::new(
                EntityCore::new(&cs, &id_pool.generate()), conf.initpt.clone(), conf.camera_dir.clone()),
            conf.scr_height, conf.scr_width);
        let camera = Camera::new(
            GameObject::new(
                EntityCore::new(&cs, &id_pool.generate()), conf.initpt, conf.camera_dir))
            .set_fov(conf.camera_fov).unwrap()
            .set_draw_dist(conf.draw_dist).unwrap();

        Self {
            cs,
            id_pool,
            entities,
            canvas,
            camera,
        }
    }
}
