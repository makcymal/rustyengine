use {
    std::collections::VecDeque,
    crate::errs::{
        ReRes,
    },
    super::*,
};


pub trait AsEvent {
    fn handle(self, camera: &mut Camera, entities: &mut EntityList) -> ReRes<()>;
}


pub trait AsEventSys {
    type Event;

    fn new() -> Self;
    fn push(&mut self, event: Self::Event);
    fn handle_all(&mut self, camera: &mut Camera, entities: &mut EntityList) -> ReRes<()>;
}


pub struct EventQueue<E: AsEvent> {
    events: VecDeque<E>,
}

impl<E: AsEvent> AsEventSys for EventQueue<E> {
    type Event = E;

    fn new() -> Self {
        Self { events: VecDeque::new() }
    }

    fn push(&mut self, event: E) {
        self.events.push_back(event);
    }

    fn handle_all(&mut self, camera: &mut Camera, entities: &mut EntityList) -> ReRes<()> {
        while let Some(event) = self.events.pop_front() {
            event.handle(camera, entities)?
        }
        Ok(())
    }
}