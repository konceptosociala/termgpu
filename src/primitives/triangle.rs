use crate::{prelude::BufferId, render::{vertex::Vertex, Drawable, Renderer}};
use nalgebra_glm as glm;
pub struct Triangle {
    pub vertex_data: [Vertex; 3],
    pub vertex_buffer: Option<BufferId>,
}

impl Drawable for Triangle {
    fn update(&mut self, renderer: &mut Renderer) {
        if self.vertex_buffer.is_none() {
            self.vertex_buffer = Some(
                renderer.create_vertex_buffer(self.vertex_data.len())
            );
        }

        let Some(id) = self.vertex_buffer else { unreachable!() };
        
        renderer
            .update_vertex_buffer(id, &self.vertex_data)
            .expect("Cannot call update() on Triangle");
    }
    

    fn vertex_buffer(&self) -> BufferId {
        self.vertex_buffer
            .expect("Triangle is not set up with update()")
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertex_data: [
                Vertex {
                    position: glm::vec3(0.0, 0.5, 0.0),
                    color: glm::vec3(1.0, 0.0, 0.0),
                    normal: glm::vec3(0.0, 0.0, 1.0),
                },
                Vertex {
                    position: glm::vec3(-0.5, -0.5, 0.0),
                    color: glm::vec3(0.0, 1.0, 0.0),
                    normal: glm::vec3(0.0, 0.0, 1.0),
                },
                Vertex {
                    position: glm::vec3(0.5, -0.5, 0.0),
                    color: glm::vec3(0.0, 0.0, 1.0),
                    normal: glm::vec3(0.0, 0.0, 1.0),
                },
            ],
            vertex_buffer: None,
        }
    }
}