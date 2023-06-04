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
        set_biform(Matrix::identity(3));

        set_exact_mode();
        set_precision(conf.precision);

        let mut id_pool = IdPool::new();
        let entities = EntityList::new();

        let cs = CoordSys::new(
            conf.initpt.clone(),
            Basis::new(Matrix::identity(3))?)?;

        let canvas = Canvas::new(
            GameObject::new(
                EntityCore::new(&id_pool.generate()),
                conf.initpt.clone(),
                Vector::new(vec![1.0, 0.0, 0.0])),
            conf.hscr,
            conf.wscr);

        let yfov = match conf.hfov {
            Some(val) => val,
            None => conf.wfov * (conf.hscr as f64) / (conf.wscr as f64)
        };

        let camera = Camera::new(
            GameObject::new(
                EntityCore::new(&id_pool.generate()),
                conf.initpt,
                Vector::new(vec![1.0, 0.0, 0.0])),
            yfov,
            conf.wfov,
            conf.draw_dist,
            conf.hscr,
            conf.wscr);

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
