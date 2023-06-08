use {
    std::{
        collections::VecDeque,
        marker::PhantomData,
    },
    crate::errs::{
        ReRes,
    },
    super::*,
};


/// Trait for events, requires `From<crossterm::event::Event>`
pub trait AsEvent<Lst>: From<console::Event>
    where Lst: AsMaterialList
{
    fn handle(&mut self, camera: &mut Camera, entities: &mut Lst) -> ReRes<()> {
        Ok(())
    }
}

/// Simple event in console
pub struct ConsoleEvent {
    event: console::Event,
}

impl From<console::Event> for ConsoleEvent {
    fn from(event: console::Event) -> Self {
        Self { event }
    }
}

impl AsEvent<EntityList> for ConsoleEvent {}


/// Trait for event systems, it's single instance is stored in `Game`
pub trait AsEventSys<Evt, Lst>
    where Evt: AsEvent<Lst>, Lst: AsMaterialList
{
    fn new() -> Self;
    fn push(&mut self, event: Evt);
    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Lst) -> ReRes<()>;
}


/// Simple event system that is just queue of obtaining events and furthermore
/// it implements `AsEventSys` handling events consequently
pub struct EventQueue<Evt, Lst>
    where Evt: AsEvent<Lst>, Lst: AsMaterialList
{
    phantom: PhantomData<Lst>,
    pub(crate) events: VecDeque<Evt>,
}

impl<Evt, Lst> AsEventSys<Evt, Lst> for EventQueue<Evt, Lst>
    where Evt: AsEvent<Lst>, Lst: AsMaterialList
{
    fn new() -> Self {
        Self {
            phantom: PhantomData,
            events: VecDeque::new(),
        }
    }

    fn push(&mut self, event: Evt) {
        self.events.push_back(event);
    }

    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Lst) -> ReRes<()> {
        while let Some(mut event) = self.events.pop_front() {
            event.handle(camera, entities)?;
        }
        Ok(())
    }
}
