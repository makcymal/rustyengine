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
pub struct Game<Evt, EvtSys, Lst>
where Evt: AsEvent<Lst>, Lst: AsMaterialList,
      EvtSys: AsEventSys<Evt, Lst>
{
    phantom: PhantomData<Evt>,
    pub(crate) cs: CoordSys,
    pub(crate) es: EvtSys,
    pub id_pool: IdPool,
    pub(crate) entities: Option<Lst>,
    pub(crate) canvas: Canvas<Lst>,
    pub(crate) camera: Camera,
}

impl<Evt, EvtSys, Lst> Game<Evt, EvtSys, Lst>
where Evt: AsEvent<Lst>, Lst: AsMaterialList,
      EvtSys: AsEventSys<Evt, Lst>
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
        let entities = None;

        if let Ok(size) = console::init() {
            (conf.wscr, conf.hscr) = (size.0 as usize, size.1 as usize)
        }
        let hfov = match conf.hfov {
            Some(val) => val,
            None => conf.comp_hfov(),
        };
        let camera = Camera::new(
            Entity::new(id_pool.generate()),
            conf.initpt,
            Vector::new(vec![1.0, 0.0, 0.0]),
            conf.draw_dist,
            conf.wfov,
            hfov,
            conf.wscr,
            conf.hscr - 3,
        );

        let canvas = Canvas::new(conf.wscr, conf.hscr - 3, conf.charmap);

        Ok(Self {
            phantom: PhantomData,
            cs,
            es,
            id_pool,
            entities,
            canvas,
            camera,
        })
    }

    pub fn run(&mut self) -> ReRes<()> {
        if self.entities.is_none() {
            return Ok(())
        }
        loop {
            self.es.push(Evt::from(console::listen()?));
            self.es.handle_all(&mut self.camera, self.entities.as_mut().unwrap())?;
            self.update()?;
        }
    }

    fn update(&mut self) -> ReRes<()> {
        self.canvas.update(&self.camera, &self.cs, self.entities.as_ref().unwrap())?;
        self.canvas.draw()?;
        Ok(())
    }

    pub fn ban(self) {
        self.canvas.banner("BAN", Duration::from_secs(1)).ok();
        std::process::exit(0)
    }

    pub fn set_entities(&mut self, entities: Lst) {
        self.entities = Some(entities)
    }

    /// `Entity` in current game with appending it's `Uuid` into `IdPool`
    pub fn entity(&mut self) -> Entity {
        Entity::new(self.id_pool.generate())
    }

    /// `Canvas` in current game
    pub fn canvas(&self) -> &Canvas<Lst> {
        &self.canvas
    }

    /// `Camera` in current game
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

impl<Evt, EvtSys, Lst> Default for Game<Evt, EvtSys, Lst>
where Evt: AsEvent<Lst>, Lst: AsMaterialList,
      EvtSys: AsEventSys<Evt, Lst>
{
    fn default() -> Self {
        let conf = Conf::default();
        Self::new(conf).unwrap()
    }
}
