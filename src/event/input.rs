use crossterm::event::{KeyCode, KeyEventKind};

pub struct KeyboardInput {
    pub code: KeyCode,
    pub kind: KeyEventKind,
}