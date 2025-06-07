pub mod camera;
pub mod mesh;
pub mod transform;
pub mod voxel_mesh;

pub mod prelude {
    pub use super::Color;
    // pub use super::camera::*;
    pub use super::mesh::*;
    pub use super::transform::*;
}

pub type Color = nalgebra_glm::Vec3;