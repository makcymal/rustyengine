use std::collections::VecDeque;
use {
    crate::{engn::*, errs::ReRes, math::*},
    crossterm::event::{self, Event as ConsoleEvent, KeyCode, KeyEvent, KeyModifiers},
    std::{cmp::Ordering, marker::PhantomData},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MovementEvent<Scn: AsScene> {
    RotateUp,
    RotateDown,
    RotateLeft,
    RotateRight,
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    None(PhantomData<Scn>),
}

impl<Scn: AsScene> From<ConsoleEvent> for MovementEvent<Scn> {
    fn from(ev: ConsoleEvent) -> Self {
        let mut key_code = KeyCode::Backspace;
        let mut key_modif = KeyModifiers::NONE;

        match ev {
            ConsoleEvent::Key(KeyEvent {
                                  code, modifiers, ..
                              }) => (key_code, key_modif) = (code, modifiers),
            _ => return Self::None(PhantomData),
        };

        match (key_code, key_modif) {
            (KeyCode::Up, KeyModifiers::NONE) => Self::RotateUp,
            (KeyCode::Down, KeyModifiers::NONE) => Self::RotateDown,
            (KeyCode::Left, KeyModifiers::NONE) => Self::RotateLeft,
            (KeyCode::Right, KeyModifiers::NONE) => Self::RotateRight,
            (KeyCode::Char('w'), KeyModifiers::NONE) => Self::MoveForward,
            (KeyCode::Char('s'), KeyModifiers::NONE) => Self::MoveBack,
            (KeyCode::Char('a'), KeyModifiers::NONE) => Self::MoveLeft,
            (KeyCode::Char('d'), KeyModifiers::NONE) => Self::MoveRight,
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => std::process::exit(0),
            _ => Self::None(PhantomData),
        }
    }
}

impl<Scn: AsScene> AsEvent<Scn> for MovementEvent<Scn> {}

pub struct MovementEventSys {
    step: f64,
    movement: [usize; 8],
}

impl MovementEventSys {
    pub fn new(step: f64) -> Self {
        Self {
            step,
            movement: [0; 8],
        }
    }
}

impl<Scn: AsScene> AsEventSys<MovementEvent<Scn>, Scn> for MovementEventSys {
    fn push(&mut self, event: MovementEvent<Scn>) {
        match event {
            MovementEvent::RotateUp => self.movement[0] += 1,
            MovementEvent::RotateDown => self.movement[1] += 1,
            MovementEvent::RotateLeft => self.movement[2] += 1,
            MovementEvent::RotateRight => self.movement[3] += 1,
            MovementEvent::MoveForward => self.movement[4] += 1,
            MovementEvent::MoveBack => self.movement[5] += 1,
            MovementEvent::MoveLeft => self.movement[6] += 1,
            MovementEvent::MoveRight => self.movement[7] += 1,
            MovementEvent::None(_) => (),
        }
    }

    fn handle_all(&mut self, cs: &CoordSys, camera: &mut Camera, scene: &mut Scn) -> ReRes<()> {
        for i in 0..5 {
            self.movement[i] /= 2;
        }

        match self.movement[0].cmp(&self.movement[1]) {
            Ordering::Greater => camera.rotate_up(self.movement[0] - self.movement[1]),
            Ordering::Less => camera.rotate_down(self.movement[1] - self.movement[0]),
            _ => (),
        }

        match self.movement[2].cmp(&self.movement[3]) {
            Ordering::Greater => camera.rotate_left(self.movement[2] - self.movement[3]),
            Ordering::Less => camera.rotate_right(self.movement[3] - self.movement[2]),
            _ => (),
        }

        let dir = camera.dir();
        let step = self.step * (self.movement[4] as f64 - self.movement[5] as f64);
        let mut mv = Vector::new(vec![dir.0 * step, dir.1 * step, 0.0]);
        scene.validate_mv(cs, camera.pos(), &mut mv);
        camera.mv(&mv)?;

        let step = self.step * (self.movement[6] as f64 - self.movement[7] as f64);
        let mut mv = Vector::new(vec![-dir.1 * step, dir.0 * step, 0.0]);
        scene.validate_mv(cs, camera.pos(), &mut mv);
        camera.mv(&mv)?;

        self.movement = [0; 8];
        Ok(())
    }
}

/// Simple event system that is just queue of obtaining events and furthermore
/// it implements `AsEventSys` handling events consequently
pub struct EventQueue<Evt, Lst>
    where
        Evt: AsEvent<Lst>,
        Lst: AsScene,
{
    phantom: PhantomData<Lst>,
    pub(crate) events: VecDeque<Evt>,
}

impl<Evt, Scn> AsEventSys<Evt, Scn> for EventQueue<Evt, Scn>
    where
        Evt: AsEvent<Scn>,
        Scn: AsScene,
{
    fn push(&mut self, event: Evt) {
        self.events.push_back(event);
    }

    fn handle_all(&mut self, cs: &CoordSys, camera: &mut Camera, entities: &mut Scn) -> ReRes<()> {
        while let Some(mut event) = self.events.pop_front() {
            event.handle(camera, entities)?;
        }
        Ok(())
    }
}
