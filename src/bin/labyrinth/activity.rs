use crossterm::event::{
    self,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyModifiers,
};

pub enum Activity {
    Forward,
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    None,
}

impl From<Event> for Activity {
    fn from(click: Event) -> Self {
        let mut key_code: KeyCode = KeyCode::Backspace;
        let mut key_modif: KeyModifiers = KeyModifiers::NONE;

        match click {
            Key(KeyEvent {
                code, modifiers, ..
            }) => (key_code, key_modif) = (code, modifiers),
            _ => return Self::None,
        };

        match (key_code, key_modif) {
            (KeyCode::Char('w'), KeyModifiers::NONE) => Self::Forward,

            (KeyCode::Up, KeyModifiers::NONE) => Self::CameraUp,

            (KeyCode::Left, KeyModifiers::NONE) => Self::CameraDown,

            (KeyCode::Down, KeyModifiers::NONE) => Self::CameraLeft,

            (KeyCode::Right, KeyModifiers::NONE) => Self::CameraRight,

            _ => Self::None,
        }
    }
}
