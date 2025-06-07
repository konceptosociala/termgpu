pub mod input;

use crate::{render::Renderer, ui::UiContext, utils::Size};

use input::KeyboardInput;

pub mod prelude {
    pub use super::{
        Event,
        KeyCode,
        KeyEventKind,
    };
    pub use super::input::*;
}

pub use crossterm::event::{KeyCode, KeyEventKind};

pub enum Event<'a> {
    Input(KeyboardInput),       // 1.
    Resize(Size),               // 2.
    Update,                     // 3.
    Render(&'a mut Renderer),   // 4.
    DrawUi(&'a mut UiContext),  // 5.
}