use {
    crate::labyrinth::{
        scene::Scene,
    },
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
    rustyengine::{
        engn::*,
        errs::ReRes,
    },
    std::cmp::Ordering,
};


#[derive(Debug, Clone, PartialEq, PartialOrd, EnumCount, EnumIter, EnumDiscriminants)]
#[strum_discriminants(name(ActionDiscr))]
pub enum Action {
    LeaveClue(i8),
    TakeClue(i8),
    RotateUp(i8),
    RotateDown(i8),
    RotateLeft(i8),
    RotateRight(i8),
    MoveForward(i8),
    None,
}

impl Action {
    pub fn times(&self) -> Option<&i8> {
        match self {
            Self::LeaveClue(t) => Some(t),
            Self::TakeClue(t) => Some(t),
            Self::RotateUp(t) => Some(t),
            Self::RotateDown(t) => Some(t),
            Self::RotateLeft(t) => Some(t),
            Self::RotateRight(t) => Some(t),
            Self::MoveForward(t) => Some(t),
            Self::None => None,
        }
    }

    pub fn times_mut(&mut self) -> Option<&mut i8> {
        match self {
            Self::LeaveClue(t) => Some(t),
            Self::TakeClue(t) => Some(t),
            Self::RotateUp(t) => Some(t),
            Self::RotateDown(t) => Some(t),
            Self::RotateLeft(t) => Some(t),
            Self::RotateRight(t) => Some(t),
            Self::MoveForward(t) => Some(t),
            Self::None => None,
        }
    }
}

impl From<ConsoleEvent> for Action {
    fn from(ev: ConsoleEvent) -> Self {
        let mut key_code = KeyCode::Backspace;
        let mut key_modif = KeyModifiers::NONE;

        match ev {
            ConsoleEvent::Key(KeyEvent {
                                  code, modifiers, ..
                              }) => (key_code, key_modif) = (code, modifiers),
            _ => return Self::None,
        };

        match (key_code, key_modif) {
            (KeyCode::Up, KeyModifiers::NONE) => Self::RotateUp(1),
            (KeyCode::Down, KeyModifiers::NONE) => Self::RotateDown(1),
            (KeyCode::Left, KeyModifiers::NONE) => Self::RotateLeft(1),
            (KeyCode::Right, KeyModifiers::NONE) => Self::RotateRight(1),
            (KeyCode::Char('w'), KeyModifiers::NONE) => Self::MoveForward(1),
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => Self::TakeClue(1),
            (KeyCode::Char('s'), KeyModifiers::NONE) => Self::LeaveClue(1),
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => std::process::exit(0),
            _ => Self::None,
        }
    }
}

impl AsEvent<Scene> for Action {}


pub struct DedupActions {
    actions: [Action; Action::COUNT - 1],
}

impl AsEventSys<Action, Scene> for DedupActions {
    fn new() -> Self {
        Self {
            actions: [
                Action::LeaveClue(0),
                Action::TakeClue(0),
                Action::RotateUp(0),
                Action::RotateDown(0),
                Action::RotateLeft(0),
                Action::RotateRight(0),
                Action::MoveForward(0),
            ]
        }
    }

    fn push(&mut self, event: Action) {
        if let Action::None = event {
            return;
        }
        *self.actions[ActionDiscr::from(event) as usize].times_mut().unwrap() += 1;
    }

    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Scene) -> ReRes<()>
    {
        match self.actions[0].times().cmp(&self.actions[1].times()) {
            Ordering::Greater => entities.leave_clue(camera.pos()),
            Ordering::Less => entities.take_clue(camera.pos()),
            _ => (),
        };


        // let vert = self.actions[2].1 - self.actions[3].1;
        // (self.actions[2].1, self.actions[3].1) = match vert.cmp(&0) {
        //     Ordering::Greater => (vert, 0),
        //     Ordering::Equal => (0, 0),
        //     Ordering::Less => (0, -vert),
        // };
        //
        // let hor = self.actions[4].1 - self.actions[5].1;
        // (self.actions[4].1, self.actions[5].1) = match hor.cmp(&0) {
        //     Ordering::Greater => (hor, 0),
        //     Ordering::Equal => (0, 0),
        //     Ordering::Less => (0, -hor),
        // };



        //
        // camera

        Ok(())
    }
}
