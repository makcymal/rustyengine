use std::process::id;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;
use {
    super::*,
    crate::{
        conf::*,
        errs::{
            ReErr::{self, *},
            ReRes,
        },
        math::*,
    },
    std::marker::PhantomData,
};

/// Struct responsible for storing current CoordSys and EntityList and running related scripts
#[derive(Debug)]
pub struct Game<Evt, EvtSys, EntLst, ColLst>
where Evt: AsEvent<EntLst, ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList,
      EvtSys: AsEventSys<Evt, EntLst, ColLst>
{
    phantom: PhantomData<Evt>,
    pub(crate) cs: CoordSys,
    pub(crate) es: EvtSys,
    pub(crate) id_pool: IdPool,
    pub(crate) entities: EntLst,
    pub(crate) collided: ColLst,
    pub(crate) canvas: Canvas<ColLst>,
    pub(crate) camera: Camera,
}

impl<Evt, EvtSys, EntLst, ColLst> Game<Evt, EvtSys, EntLst, ColLst>
where Evt: AsEvent<EntLst, ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList,
      EvtSys: AsEventSys<Evt, EntLst, ColLst>
{
    /// Constructor for `Game` taking `Conf` and returning `ReRes` if something fails
    pub fn new(mut conf: Conf) -> ReRes<Self> {
        set_biform(Matrix::identity(3));

        set_exact_mode();
        set_precision(conf.precision);

        let cs = CoordSys::new(
            conf.initpt.clone(),
            Basis::new(Matrix::identity(3).to_multicol())?,
        )?;

        let es = EvtSys::new();

        let mut id_pool = IdPool::new();
        let entities = EntLst::new();
        let collided = ColLst::new();

        if let Ok(size) = console::init() {
            (conf.wscr, conf.hscr) = (size.0 as usize, size.1 as usize)
        }
        let canvas = Canvas::new(conf.wscr, conf.hscr - 3, conf.charmap);

        let hfov = match conf.hfov {
            Some(val) => val,
            None => conf.wfov * (conf.hscr as f64) / (conf.wscr as f64),
        };

        let camera = Camera::new(
            Entity::new(&id_pool.generate()),
            conf.initpt,
            Vector::new(vec![1.0, 0.0, 0.0]),
            conf.draw_dist,
            conf.wfov,
            hfov,
            conf.hscr - 3,
            conf.wscr,
        );

        Ok(Self {
            phantom: PhantomData,
            cs,
            es,
            id_pool,
            entities,
            collided,
            canvas,
            camera,
        })
    }

    pub fn run(&mut self) -> ReRes<()> {
        loop {
            self.es.push(Evt::from(console::listen()?));
            self.es.handle_all(&mut self.camera, &mut self.entities, &mut self.collided)?;
            self.update()?;
        }
    }

    pub fn update(&mut self) -> ReRes<()> {
        self.canvas.update(&self.camera, &self.cs, &self.collided)?;
        self.canvas.draw()?;
        Ok(())
    }

    pub fn ban(self) {
        self.canvas.banner("BAN", Duration::from_secs(1)).ok();
        std::process::exit(0)
    }

    /// `Entity` in current game with appending it's `Uuid` into `IdPool`
    pub fn entity(&mut self) -> Entity {
        Entity::new(&self.id_pool.generate())
    }

    /// `Canvas` in current game
    pub fn canvas(&self) -> &Canvas<ColLst> {
        &self.canvas
    }

    /// `Camera` in current game
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

impl<Evt, EvtSys, EntLst, ColLst> Default for Game<Evt, EvtSys, EntLst, ColLst>
where Evt: AsEvent<EntLst, ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList,
      EvtSys: AsEventSys<Evt, EntLst, ColLst>
{
    fn default() -> Self {
        let conf = Conf::default();
        Self::new(conf).unwrap()
    }
}
