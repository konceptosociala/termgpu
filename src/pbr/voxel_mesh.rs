use nalgebra_glm as glm;

use crate::render::vertex::Vertex;
use super::{mesh::Mesh, Color};

impl Mesh {
    /// Adds a top face to the mesh at the specified position with the given color.
    pub fn add_top_face(&mut self, x: usize, y: usize, z: usize, color: Color) {
        let normal = glm::vec3(0.0, 1.0, 0.0);

        self.vertex_data.extend(&[
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0), normal, color },
        ]);
    }

    /// Adds a bottom face to the mesh at the specified position with the given color.
    pub fn add_bottom_face(&mut self, x: usize, y: usize, z: usize, color: Color) {
        let normal = glm::vec3(0.0, -1.0, 0.0);

        self.vertex_data.extend(&[
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32 + 1.0), normal, color },
        ]);
    }

    /// Adds a front face to the mesh at the specified position with the given color.
    pub fn add_front_face(&mut self, x: usize, y: usize, z: usize, color: Color) {
        let normal = glm::vec3(0.0, 0.0, 1.0);

        self.vertex_data.extend(&[
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32), normal, color },
        ]);
    }

    /// Adds a back face to the mesh at the specified position with the given color.
    pub fn add_back_face(&mut self, x: usize, y: usize, z: usize, color: Color) {
        let normal = glm::vec3(0.0, 0.0, -1.0);

        self.vertex_data.extend(&[
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32 + 1.0), normal, color },
        ]);
    }

    /// Adds a left face to the mesh at the specified position with the given color.
    pub fn add_left_face(&mut self, x: usize, y: usize, z: usize, color: Color) {
        let normal = glm::vec3(-1.0, 0.0, 0.0);

        self.vertex_data.extend(&[
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32, y as f32 + 1.0, z as f32 + 1.0), normal, color },
        ]);
    }

    /// Adds a right face to the mesh at the specified position with the given color.
    pub fn add_right_face(&mut self, x: usize, y: usize, z: usize, color: Color) {
        let normal = glm::vec3(1.0, 0.0, 0.0);

        self.vertex_data.extend(&[
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32, z as f32 + 1.0), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32), normal, color },
            Vertex { position: glm::vec3(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0), normal, color },
        ]);
    }
}