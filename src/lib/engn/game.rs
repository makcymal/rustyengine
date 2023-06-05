use {
    super::*,
    crate::errs::{
        ReErr::{self, *},
        ReRes,
    },
    crate::{conf::Conf, math::*},
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
            Basis::new(Matrix::identity(3).to_multicol())?,
        )?;

        let canvas = Canvas::new(conf.hscr, conf.wscr);

        let hfov = match conf.hfov {
            Some(val) => val,
            None => conf.wfov * (conf.hscr as f64) / (conf.wscr as f64),
        };

        let camera = Camera::new(
            Core::new(&id_pool.generate()),
            conf.initpt,
            Vector::new(vec![1.0, 0.0, 0.0]),
            conf.draw_dist,
            conf.wfov,
            hfov,
            conf.hscr,
            conf.wscr,
        );

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

    /// `Core` in current basis with appending it's `Uuid` into `IdPool`
    pub fn core(&mut self) -> Core {
        Core::new(&self.id_pool.generate())
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
