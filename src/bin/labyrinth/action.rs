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
        math::*,
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
    pub fn increment(&mut self) {
        match self {
            Self::LeaveClue(t) => *t = t.saturating_add(1),
            Self::TakeClue(t) => *t = t.saturating_add(1),
            Self::RotateUp(t) => *t = t.saturating_add(1),
            Self::RotateDown(t) => *t = t.saturating_add(1),
            Self::RotateLeft(t) => *t = t.saturating_add(1),
            Self::RotateRight(t) => *t = t.saturating_add(1),
            Self::MoveForward(t) => *t = t.saturating_add(1),
            Self::None => unreachable!(),
        }
    }

    pub fn times(&self) -> i8 {
        match self {
            Self::LeaveClue(t) => *t,
            Self::TakeClue(t) => *t,
            Self::RotateUp(t) => *t,
            Self::RotateDown(t) => *t,
            Self::RotateLeft(t) => *t,
            Self::RotateRight(t) => *t,
            Self::MoveForward(t) => *t,
            Self::None => unreachable!(),
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
        self.actions[ActionDiscr::from(event) as usize].increment();
    }

    fn handle_all(&mut self, camera: &mut Camera, entities: &mut Scene) -> ReRes<()> {
        match self.actions[0].times().cmp(&self.actions[1].times()) {
            Ordering::Greater => entities.leave_clue(camera.pos()),
            Ordering::Less => entities.take_clue(camera.pos()),
            _ => (),
        };


        let vert = self.actions[2].times() - self.actions[3].times();
        match vert.cmp(&0) {
            Ordering::Greater => camera.rotate_up(vert as usize),
            Ordering::Less => camera.rotate_down(-vert as usize),
            _ => (),
        };

        let hor = self.actions[4].times() - self.actions[5].times();
        match hor.cmp(&0) {
            Ordering::Greater => camera.rotate_left(hor as usize),
            Ordering::Less => camera.rotate_right(-hor as usize),
            _ => (),
        };

        let mv = self.actions[6].times();
        if mv != 0 {
            let dir = camera.dir();
            camera.mv(&Vector::new(vec![dir.0, dir.1, 0.0]))?
        }

        self.actions = [
                Action::LeaveClue(0),
                Action::TakeClue(0),
                Action::RotateUp(0),
                Action::RotateDown(0),
                Action::RotateLeft(0),
                Action::RotateRight(0),
                Action::MoveForward(0),
        ];

        Ok(())
    }
}
