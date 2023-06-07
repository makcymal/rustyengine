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


pub trait AsEvent<EntLst, ColLst>: From<console::Event>
where EntLst: AsEntityList, ColLst: AsCollidedList
{
    fn handle(&mut self, camera: &mut Camera,
              entities: &mut EntLst, collided: &mut ColLst) -> ReRes<()> {
        Ok(())
    }
}


pub trait AsEventSys<Evt, EntLst, ColLst>
where Evt: AsEvent<EntLst, ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList
{
    fn new() -> Self;
    fn push(&mut self, event: Evt);
    fn handle_all(&mut self, camera: &mut Camera,
                  entities: &mut EntLst, collided: &mut ColLst) -> ReRes<()>;
}


pub struct EventQueue<Evt, EntLst, ColLst>
where Evt: AsEvent<EntLst, ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList
{
    phantom: PhantomData<(EntLst, ColLst)>,
    events: VecDeque<Evt>,
}

impl<Evt, EntLst, ColLst> AsEventSys<Evt, EntLst, ColLst> for EventQueue<Evt, EntLst, ColLst>
where Evt: AsEvent<EntLst, ColLst>, EntLst: AsEntityList, ColLst: AsCollidedList
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

    fn handle_all(&mut self, camera: &mut Camera,
                  entities: &mut EntLst, collided: &mut ColLst) -> ReRes<()> {
        while let Some(mut event) = self.events.pop_front() {
            event.handle(camera, entities, collided)?;
        }
        Ok(())
    }
}
