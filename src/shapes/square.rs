use crate::{
    buffers_layouts::Vertex
};

pub const SQUARE_VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.0, 0.0], texture_coordinates: [0.0, 1.0]},
    Vertex { position: [1.0, 0.0, 0.0], texture_coordinates: [1.0, 1.0]},
    Vertex { position: [1.0, 1.0, 0.0], texture_coordinates: [1.0, 0.0]},
    Vertex { position: [0.0, 1.0, 0.0], texture_coordinates: [0.0, 0.0]},
];

pub const SQUARE_INDEX: &[u16] = &[0, 1, 2, 0, 2, 3];
