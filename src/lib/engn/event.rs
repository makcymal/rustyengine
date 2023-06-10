use {
    crossterm::event::{
        self,
        Event as ConsoleEvent,
        KeyCode, KeyEvent, KeyModifiers,
    },
    strum::EnumCount,
    strum_macros::{
        EnumCount,
        EnumIter,
        EnumDiscriminants,
    },
    crate::{
        engn::*,
        errs::ReRes,
        math::*,
    },
    std::{
        cmp::Ordering,
        marker::PhantomData,
    },
};


#[derive(Debug, Clone, PartialEq, PartialOrd, EnumCount, EnumIter, EnumDiscriminants)]
#[strum_discriminants(name(MovementEventDiscr))]
pub enum MovementEvent<Scn: AsScene> {
    RotateUp,
    RotateDown,
    RotateLeft,
    RotateRight,
    MoveForward,
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
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => std::process::exit(0),
            _ => Self::None(PhantomData),
        }
    }
}

impl<Scn: AsScene> AsEvent<Scn> for MovementEvent<Scn> {}


pub struct MovementEventSys {
    movement: [usize; 5],
}

impl<Scn: AsScene> AsEventSys<MovementEvent<Scn>, Scn> for MovementEventSys {
    fn new() -> Self {
        Self {
            movement: [0; 5]
        }
    }

    fn push(&mut self, event: MovementEvent<Scn>) {
        match event {
            MovementEvent::RotateUp => self.movement[0] += 1,
            MovementEvent::RotateDown => self.movement[1] += 1,
            MovementEvent::RotateLeft => self.movement[2] += 1,
            MovementEvent::RotateRight => self.movement[3] += 1,
            MovementEvent::MoveForward => self.movement[4] += 1,
            MovementEvent::None(_) => (),
        }
    }

    fn handle_all(&mut self, camera: &mut Camera, _entities: &mut Scn) -> ReRes<()> {
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

        if self.movement[4] != 0 {
            let dir = camera.dir();
            camera.mv(&Vector::new([dir.0, dir.1, 0.0]))
        }

        self.movement = [0; 5];
        Ok(())
    }
}
