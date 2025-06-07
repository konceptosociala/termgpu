//! Camera module contains camera struct, its GPU representation
//! and auxiliary constants

use bytemuck::{Pod, Zeroable};
use nalgebra_glm as glm;
use serde::{Deserialize, Serialize};
use crate::render::{hal::Padding, TransformationType};
use super::transform::Transform;

/// A matrix to convert OpenGL coordinate system to WGPU coordinate system.
pub const OPENGL_TO_WGPU_MATRIX: glm::Mat4 = glm::Mat4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

/// Represents a camera in the scene, holding information about its type and projection parameters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Camera {
    aspect: f32,
    fovy: f32,
    near: f32,
    far: f32,
}

impl Camera {
    /// Creates a new camera with the specified type and aspect ratio.
    pub fn new(aspect: f32) -> Camera {
        Camera {
            aspect,
            fovy: 45.0,
            near: 0.1,
            far: 100.0,
        }
    }

    /// Builds the view-projection matrix for the camera based on its transform.
    pub fn build_view_projection(&self, transform: &Transform) -> glm::Mat4 {
        let rotation_matrix = glm::quat_cast(&transform.rotation);
        let translation_matrix = glm::translation(&transform.translation);

        let view = match transform.transformation_type {
            TransformationType::FirstPerson => rotation_matrix * translation_matrix,
            TransformationType::LookAt => translation_matrix * rotation_matrix,
        };

        let projection = glm::perspective(self.aspect, self.fovy, self.near, self.far);

        OPENGL_TO_WGPU_MATRIX * projection * view
    }

    /// Sets the aspect ratio of the camera's view.
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

/// Uniform data structure for the camera, used for passing camera information to the GPU.
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct CameraUniform {
    position: glm::Vec3,
    _padding: Padding,
    view_projection: glm::Mat4,
}

impl Default for CameraUniform {
    fn default() -> Self {
        CameraUniform {
            position: glm::Vec3::identity(),
            view_projection: glm::Mat4::identity(),
            _padding: Padding::default(),
        }
    }
}

impl CameraUniform {
    /// Creates a new `CameraUniform` from a given camera and transform.
    pub fn new(camera: &Camera, transform: &Transform) -> CameraUniform {
        CameraUniform {
            position: transform.translation,
            view_projection: camera.build_view_projection(transform),
            _padding: Padding::default(),
        }
    }
}