use {
    crate::errs::{
        ReRes,
    },
    super::*,
};


/// Trait for events, requires `From<crossterm::event::Event>`
pub trait AsEvent<Scn>: From<console::Event>
    where Scn: AsScene
{
    fn handle(&mut self, camera: &mut Camera, entities: &mut Scn) -> ReRes<()> {
        Ok(())
    }
}


/// Trait for event systems, it's single instance is stored in `Game`
pub trait AsEventSys<Evt, Scn>
    where Evt: AsEvent<Scn>, Scn: AsScene
{
    fn new() -> Self;
    fn push(&mut self, event: Evt);
    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Scn) -> ReRes<()>;
}
