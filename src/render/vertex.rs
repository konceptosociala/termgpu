use bytemuck::{Pod, Zeroable};
use nalgebra_glm as glm;

use crate::pbr::Color;

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct Vertex {
    /// Position of the vertex in 3D space.
    pub position: glm::Vec3,
    /// Normal vector for the vertex, used in lighting calculations.
    pub normal: glm::Vec3,
    /// Color of the vertex.
    pub color: Color,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x3,
    ];

    pub(crate) fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}