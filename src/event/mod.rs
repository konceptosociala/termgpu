pub mod input;

use crate::{render::Renderer, ui::UiContext};

use input::KeyboardInput;

pub enum Event<'a> {
    Resize(u16, u16),
    Input(KeyboardInput),
    Update,
    Render(&'a mut Renderer),
    DrawUi(&'a mut UiContext),
}