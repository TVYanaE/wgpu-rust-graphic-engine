use crate::{
    modules::{
        render_thread::{
            states::{
                gpu_state::{
                    buffers_layouts::Vertex
                },
            },
        },
    }, 
};

#[allow(dead_code)]
pub struct TriangleShape {
    vertices: [Vertex; 3],
}

#[allow(dead_code)]
impl TriangleShape {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self { 
            vertices: vertices, 
        }
    }
}
