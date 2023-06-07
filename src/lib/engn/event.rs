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


pub trait AsEvent<Lst>: From<console::Event>
where Lst: AsMaterialList
{
    fn handle(&mut self, camera: &mut Camera, entities: &mut Lst) -> ReRes<()> {
        Ok(())
    }
}


pub trait AsEventSys<Evt, Lst>
where Evt: AsEvent<Lst>, Lst: AsMaterialList
{
    fn new() -> Self;
    fn push(&mut self, event: Evt);
    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Lst) -> ReRes<()>;
}


pub struct EventQueue<Evt, Lst>
where Evt: AsEvent<Lst>, Lst: AsMaterialList
{
    phantom: PhantomData<Lst>,
    events: VecDeque<Evt>,
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
