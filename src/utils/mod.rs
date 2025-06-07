pub mod prelude {

}

pub mod macros;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Renderer(u32, u32),
    Terminal(u16, u16),
}

impl Size {
    pub fn new_renderer(width: u32, height: u32) -> Self {
        Size::Renderer(width, height)
    }

    pub fn new_terminal(cols: u16, rows: u16) -> Self {
        Size::Terminal(cols, rows)
    }

    pub fn to_renderer(&self) -> Size {
        match self {
            Size::Renderer(_, _) => *self,
            Size::Terminal(cols, rows) => Size::Renderer(*cols as u32, *rows as u32 * 2),
        }
    }

    pub fn to_terminal(&self) -> Size {
        match self {
            Size::Renderer(w, h) => Size::Terminal(*w as u16, *h as u16 / 2),
            Size::Terminal(_, _) => *self,
        }
    }
}