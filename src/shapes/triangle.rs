use crate::vertex::Vertex;


pub struct TriangleShape {
    vertices: [Vertex; 3],
}

impl TriangleShape {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self { 
            vertices: vertices, 
        }
    }
}
