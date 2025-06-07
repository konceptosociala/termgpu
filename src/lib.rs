pub mod app;
pub mod event;
pub mod pbr;
pub mod primitives;
pub mod render;
pub mod terminal;
pub mod ui;
pub mod utils;

pub mod prelude {
    pub use crate::{fatal, include_spirv, include_wgsl};
    pub use crate::app::prelude::*;
    pub use crate::event::prelude::*;
    pub use crate::render::prelude::*;
    pub use crate::pbr::prelude::*;
    pub use crate::primitives::prelude::*;
    pub use crate::terminal::prelude::*;
    pub use crate::ui::prelude::*;
    // pub use crate::utils::prelude::*;
    pub use nalgebra_glm as glm;

}