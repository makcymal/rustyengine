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
        f32::consts::PI,
    },
    uuid::Uuid,
};

/// Struct responsible for storing current CoordSys and EntityList and running related scripts
#[derive(Debug)]
pub struct Game<Evt, EvtSys, Scn>
    where Evt: AsEvent<Scn>, Scn: AsScene, EvtSys: AsEventSys<Evt, Scn>
{
    phantom: PhantomData<Evt>,
    pub(crate) es: EvtSys,
    pub(crate) scene: Scn,
    pub(crate) canvas: Canvas<Scn>,
    pub(crate) camera: Camera,
}

impl<Evt, EvtSys, Scn> Game<Evt, EvtSys, Scn>
    where Evt: AsEvent<Scn>, Scn: AsScene, EvtSys: AsEventSys<Evt, Scn>
{
    /// Constructor for `Game` taking `Conf` and returning `ReRes` if something fails
    pub fn new(mut conf: Conf, scene: Scn) -> ReRes<Self> {
        set_precision(conf.precision);

        let es = EvtSys::new();

        let size = console::init()?;
        let mut size = ((size.0 - 3) as usize, size.1 as usize);
        if size.0 % 2 == 0 { size.0 -= 1 }
        if size.1 % 2 == 0 { size.1 -= 1 }

        let hfov = match conf.hfov {
            Some(val) => val,
            None => (size.0 as f32) * conf.wfov / (size.1 as f32),
        };

        let camera = Camera::new(
            conf.initpt,
            conf.angle_discr,
            conf.wfov * PI,
            hfov * PI,
            size.clone(),
            conf.draw_dist,
        );

        let canvas = Canvas::new(size, conf.charmap, conf.draw_dist);

        Ok(Self {
            phantom: PhantomData,
            es,
            scene,
            canvas,
            camera,
        })
    }

    /// Running game: listening to events, handling them with respect to given implementation.
    /// Never exits if such event isn't provided
    pub fn run(&mut self) -> ReRes<()> {
        loop {
            self.es.push(Evt::from(console::listen()?));
            self.es.handle_all(&mut self.camera, &mut self.scene)?;
            self.update()?;
        }
    }

    /// Updates image on canvas and drawing it in console
    fn update(&mut self) -> ReRes<()> {
        self.canvas.update(&self.camera, &self.scene)?;
        self.canvas.draw()?;
        Ok(())
    }

    /// Exits game process with printing useful message
    pub fn ban(self) {
        self.canvas.banner("BAN", Duration::from_secs(1)).ok();
        std::process::exit(0)
    }

    /// `Canvas` in current game
    pub fn canvas(&self) -> &Canvas<Scn> {
        &self.canvas
    }

    /// `Camera` in current game
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
