use crate::structures::buffers_layouts::Vertex;

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
