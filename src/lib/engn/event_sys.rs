use std::marker::PhantomData;
use {
    std::collections::VecDeque,
    crate::errs::{
        ReRes,
    },
    super::*,
};


pub trait AsEvent: From<console::Event> {
    type EntLst: AsEntityList;
    type ColLst: AsCollidedList;

    fn handle(self, camera: &mut Camera, entities: &mut Self::EntLst, collided: &mut Self::ColLst) -> ReRes<()>;
}


pub trait AsEventSys<Evt: AsEvent> {
    type EntLst: AsEntityList;
    type ColLst: AsCollidedList;

    fn new() -> Self;
    fn push(&mut self, event: Evt);
    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Self::EntLst, collided: &mut Self::ColLst) -> ReRes<()>;
}


pub struct EventQueue<Evt, EntLst, ColLst>
    where Evt: AsEvent, EntLst: AsEntityList, ColLst: AsCollidedList {

    phantom: PhantomData<(EntLst, ColLst)>,
    events: VecDeque<Evt>,
}

impl<Evt, EntLst, ColLst> AsEventSys<Evt> for EventQueue<Evt, EntLst, ColLst>
    where Evt: AsEvent<EntLst=EntLst, ColLst=ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList {

    type EntLst = EntLst;
    type ColLst = ColLst;

    fn new() -> Self {
        Self {
            phantom: PhantomData,
            events: VecDeque::new()
        }
    }

    fn push(&mut self, event: Evt) {
        self.events.push_back(event);
    }

    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Self::EntLst, collided: &mut Self::ColLst) -> ReRes<()> {
        while let Some(event) = self.events.pop_front() {
            event.handle(camera, entities, collided)?
        }
        Ok(())
    }
}
