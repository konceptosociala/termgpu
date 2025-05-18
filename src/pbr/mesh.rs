//! Mesh module provides structures and utilities for handling 3D mesh data.
//! It includes definitions for vertices and meshes, as well as methods for generating 
//! faces for a voxel-based rendering system.
use crate::render::vertex::Vertex;

/// A mesh structure containing vertex data.
#[derive(Debug, Clone, Default)]
pub struct Mesh {
    /// The list of vertices that form the mesh.
    pub vertex_data: Vec<Vertex>,
}