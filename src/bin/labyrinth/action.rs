use {
    crossterm::event::{
        self,
        Event as TerminalEvent,
        KeyCode, KeyEvent, KeyModifiers,
    },
    strum::EnumCount,
    strum_macros::{
        EnumCount,
        EnumIter,
    },
    rustyengine::{
        engn::*,
        errs::ReRes,
    },
    std::cmp::Ordering,
};


#[derive(Debug, Clone, PartialEq, PartialOrd, EnumCount, EnumIter)]
pub enum Action {
    LeaveClue,
    TakeClue,
    RotateUp,
    RotateDown,
    RotateLeft,
    RotateRight,
    MoveForward,
    None,
}

impl From<TerminalEvent> for Action {
    fn from(te: TerminalEvent) -> Self {
        let mut key_code = KeyCode::Backspace;
        let mut key_modif = KeyModifiers::NONE;

        match te {
            TerminalEvent::Key(KeyEvent {
                                   code, modifiers, ..
                               }) => (key_code, key_modif) = (code, modifiers),
            _ => return Self::None,
        };

        match (key_code, key_modif) {
            (KeyCode::Up, KeyModifiers::NONE) => Self::RotateUp,
            (KeyCode::Down, KeyModifiers::NONE) => Self::RotateDown,
            (KeyCode::Left, KeyModifiers::NONE) => Self::RotateLeft,
            (KeyCode::Right, KeyModifiers::NONE) => Self::RotateRight,
            (KeyCode::Char('w'), KeyModifiers::NONE) => Self::MoveForward,
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => Self::TakeClue,
            (KeyCode::Char('s'), KeyModifiers::NONE) => Self::LeaveClue,
            _ => Self::None,
        }
    }
}

impl AsEvent for Action {
    fn handle(self, camera: &mut Camera, entities: &mut EntityList) -> ReRes<()> {
        match self {
            //     Self::Forward(diff) => {}
            //     Self::CameraUp(diff) => {}
            //     Self::CameraDown(diff) => {}
            //     Self::CameraLeft(diff) => {}
            //     Self::CameraRight(diff) => {}
            _ => ()
        }
        Ok(())
    }
}


pub struct DedupActions {
    actions: [(Action, i8); Action::COUNT],
}

impl AsEventSys for DedupActions {
    type Event = Action;

    fn new() -> Self {
        Self {
            actions: [
                (Action::LeaveClue, 0),
                (Action::TakeClue, 0),
                (Action::RotateUp, 0),
                (Action::RotateDown, 0),
                (Action::RotateLeft, 0),
                (Action::RotateRight, 0),
                (Action::MoveForward, 0),
                (Action::None, 0)
            ]
        }
    }

    fn push(&mut self, event: Action) {
        if let Action::None = event {
            return;
        }
        self.actions[event as usize].1 += 1;
    }

    fn handle_all(&mut self, camera: &mut Camera, entities: &mut EntityList) -> ReRes<()> {
        (self.actions[0].1, self.actions[1].1) = match self.actions[0].1.cmp(&self.actions[1].1) {
            Ordering::Greater => (1, 0),
            Ordering::Equal => (0, 0),
            Ordering::Less => (0, 1),
        };

        let vert = self.actions[2].1 - self.actions[3].1;
        (self.actions[2].1, self.actions[3].1) = match vert.cmp(&0) {
            Ordering::Greater => (vert, 0),
            Ordering::Equal => (0, 0),
            Ordering::Less => (0, -vert),
        };

        let hor = self.actions[4].1 - self.actions[5].1;
        (self.actions[4].1, self.actions[5].1) = match hor.cmp(&0) {
            Ordering::Greater => (hor, 0),
            Ordering::Equal => (0, 0),
            Ordering::Less => (0, -hor),
        };



        Ok(())
    }
}
