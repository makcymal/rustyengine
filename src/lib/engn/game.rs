use {
    super::*,
    crate::{
        conf::Conf,
        math::*,
    },
    crate::errs::{
        ReRes,
        ReErr::{self, *},
    },
};


/// Struct responsible for storing current CoordSys and EntityList and running related scripts
#[derive(Debug)]
pub struct Game {
    pub(crate) cs: CoordSys,
    pub(crate) id_pool: IdPool,
    pub(crate) entities: EntityList,
    pub(crate) canvas: Canvas,
    pub(crate) camera: Camera,
}

impl Game {
    /// Constructor for `Game` taking `Conf` and `ReRes` if something fails
    pub fn new(conf: Conf) -> ReRes<Self> {
        set_biform(conf.biform);

        set_exact_mode();
        set_precision(conf.precision);

        let mut id_pool = IdPool::new();
        let entities = EntityList::new();

        let cs = CoordSys::new(
            conf.initpt.clone(),
            Basis::new(conf.basis)?)?;

        let canvas = Canvas::new(
            GameObject::new(
                EntityCore::new(&id_pool.generate()),
                conf.initpt.clone(),
                conf.camera_dir.clone()),
            conf.scr_height,
            conf.scr_width);

        let camera = Camera::new(
            GameObject::new(EntityCore::new(&id_pool.generate()), conf.initpt, conf.camera_dir),
            conf.camera_fov,
            conf.camera_vfov,
            conf.draw_dist);

        Ok(Self {
            cs,
            id_pool,
            entities,
            canvas,
            camera,
        })
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

    /// `EntityCore` in current basis with appending it's `Uuid` into `IdPool`
    pub fn entity_core(&mut self) -> EntityCore {
        EntityCore::new(&self.id_pool.generate())
    }

    /// `GameObject` in current game, uses `self.entity_core()`
    pub fn game_object(&mut self, pos: Point, dir: Vector) -> GameObject {
        GameObject::new(self.entity_core(), pos, dir)
    }

    /// `Canvas` in current game
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    /// `Camera` in current game
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

impl Default for Game {
    fn default() -> Self {
        let conf = Conf::default();
        Self::new(conf).unwrap()
    }
}
