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
    std::{
        marker::PhantomData,
        rc::Rc,
        time::Duration,
        f64::consts::PI,
    },
    uuid::Uuid
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

        let size = console::init()?;
        let size = ((size.0 - 3) as usize, size.1 as usize);

        let hfov = match conf.hfov {
            Some(val) => val,
            None => (size.0 as f64) * conf.wfov / (size.1 as f64),
        };

        let camera = Camera::new(
            conf.initpt,
            conf.angle_discr,
            conf.wfov * PI,
            hfov * PI,
            size.clone(),
            conf.draw_dist,
        );

        let canvas = Canvas::new(size, conf.charmap);

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

    /// Running game: listening to events, handling them with respect to given implementation.
    /// Never exits if such event isn't provided
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

    /// Updates image on canvas and drawing it in console
    fn update(&mut self) -> ReRes<()> {
        self.canvas.update(&self.camera, &self.cs, self.entities.as_ref().unwrap())?;
        self.canvas.draw()?;
        Ok(())
    }

    /// Exits game process with printing useful message
    pub fn ban(self) {
        self.canvas.banner("BAN", Duration::from_secs(1)).ok();
        std::process::exit(0)
    }

    /// Providing entities list
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
