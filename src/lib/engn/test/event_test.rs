use {
    super::super::*,
    crate::{conf::*, math::*},
};

struct EmtpyEvent(i8);

impl From<console::Event> for EmtpyEvent {
    fn from(value: console::Event) -> Self {
        EmtpyEvent(0)
    }
}

impl AsEvent<EntityList> for EmtpyEvent {}

#[test]
fn push_to_event_queue() {
    let mut q = EventQueue::new();
    q.push(EmtpyEvent(0));
    assert_eq!(q.events.len(), 1);
}

#[test]
fn push_push_to_event_queue() {
    let mut q = EventQueue::new();
    q.push(EmtpyEvent(0));
    q.push(EmtpyEvent(1));
    assert_eq!(q.events.len(), 2);
}

#[test]
fn handle_all_to_event_queue() {
    let mut game = Game::<EmtpyEvent, EventQueue<EmtpyEvent, EntityList>, EntityList>::default();
    game.set_entities(EntityList::new());
    let mut q = EventQueue::new();
    q.push(EmtpyEvent(0));
    q.push(EmtpyEvent(1));
    assert!(q
        .handle_all(&mut game.camera, game.scene.as_mut().unwrap())
        .is_ok());
}

#[test]
fn handle_all_count_to_event_queue() {
    let mut game = Game::<EmtpyEvent, EventQueue<EmtpyEvent, EntityList>, EntityList>::default();
    game.set_entities(EntityList::new());
    let mut q = EventQueue::new();
    q.push(EmtpyEvent(0));
    q.push(EmtpyEvent(1));
    q.handle_all(&mut game.camera, game.scene.as_mut().unwrap())
        .unwrap();
    assert_eq!(q.events.len(), 0);
}
