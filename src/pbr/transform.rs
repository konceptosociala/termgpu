//! Transform module contains transformation components 
//! (structs, traits and GPU structs)

use bytemuck::{Pod, Zeroable};
use serde::{Serialize, Deserialize};
use nalgebra_glm as glm;

use crate::render::{InstanceData, Transformation, TransformationType::{self, *}};

/// A structure representing a transformation in 3D space, including translation, rotation, and scale.
///
/// The transformation is represented by a translation vector, a rotation quaternion, and a uniform scale factor.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    /// The type vector of the transformation.
    pub transformation_type: TransformationType,

    /// The translation vector of the transformation.
    pub translation: glm::Vec3,
    
    /// The rotation of the transformation, represented as a quaternion.
    pub rotation: glm::Quat,

    /// The point relative to which the object will be transformed
    pub pivot: glm::Vec3,
}

impl Transformation for Transform {
    fn from_transform(
        transformation_type: TransformationType,
        translation: nalgebra_glm::Vec3, 
        rotation: nalgebra_glm::Quat, 
        pivot: nalgebra_glm::Vec3,
    ) -> Self {
        Transform::new(transformation_type, translation, rotation, pivot)
    }
}

impl Transform {
    /// Constructs a new `Transform` instance with the given translation, rotation, and scale.
    pub fn new(
        transformation_type: TransformationType,
        translation: glm::Vec3, 
        rotation: glm::Quat, 
        pivot: glm::Vec3,
    ) -> Transform {
        Transform { transformation_type, translation, rotation, pivot }
    }

    /// Returns a `Transform` instance with default properties (identity transformation).
    pub fn identity() -> Transform {
        Transform::default()
    }

    /// Constructs a new `Transform` instance with the given translation and default rotation and scale.
    pub fn new_from_translation(translation: glm::Vec3) -> Transform {
        Transform { translation, ..Default::default() }
    }

    /// Constructs a new `Transform` instance with the given rotation and default translation and scale.
    pub fn new_from_rotation(rotation: glm::Quat) -> Transform {
        Transform { rotation, ..Default::default() }
    }

    /// Returns the local x-axis direction vector in world space.
    pub fn local_x(&self) -> glm::Vec3 {
        let m = TransformUniform::new(self).transform_matrix;
        
        glm::vec3(
            m[(0, 0)],
            m[(0, 1)], 
            m[(0, 2)]
        )
    }
    
    /// Returns the local y-axis direction vector in world space.
    pub fn local_y(&self) -> glm::Vec3 {
        let m = TransformUniform::new(self).transform_matrix;
        
        glm::vec3(
            m[(1, 0)],
            m[(1, 1)], 
            m[(1, 2)]
        )
    }
    
    /// Returns the local z-axis direction vector in world space.
    pub fn local_z(&self) -> glm::Vec3 {
        let m = TransformUniform::new(self).transform_matrix;
        
        glm::vec3(
            m[(2, 0)],
            m[(2, 1)], 
            m[(2, 2)]
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            transformation_type: TransformationType::LookAt,
            translation: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::Quat::identity(),
            pivot: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}

/// A structure representing the transformation matrices used for rendering.
///
/// Contains both the transformation matrix and its inverse.
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct TransformUniform {
    /// The transformation matrix that combines translation, rotation, and scale.
    pub transform_matrix: glm::Mat4,
    
    /// The inverse of the transformation matrix.
    pub inverse_matrix: glm::Mat4,
}

impl Default for TransformUniform {
    fn default() -> Self {
        TransformUniform {
            transform_matrix: glm::Mat4::identity(),
            inverse_matrix: glm::Mat4::identity().try_inverse().unwrap(),
        }
    }
}

impl TransformUniform {
    /// Creates a new `TransformUniform` instance based on the provided `Transform`.
    pub fn new(transform: &Transform) -> TransformUniform {
        let transform_matrix = if transform.transformation_type == LookAt {
            glm::Mat4::identity()
                * glm::translation(&transform.translation)
                * glm::quat_cast(&transform.rotation)
                * glm::translation(&-transform.pivot)
        } else {
            glm::Mat4::identity()
                * glm::quat_cast(&transform.rotation)
                * glm::translation(&transform.translation)
                * glm::translation(&-transform.pivot)
        };

        let inverse_matrix = transform_matrix.try_inverse().unwrap();

        TransformUniform {
            transform_matrix,
            inverse_matrix,
        }
    }
}

impl InstanceData for Transform {
    type UniformData = TransformUniform;

    fn uniform_data(&self) -> Self::UniformData {
        TransformUniform::new(self)
    }
}