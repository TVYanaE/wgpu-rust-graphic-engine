use std::mem::size_of;
use bytemuck::{Pod, Zeroable};
use wgpu::{
    VertexBufferLayout, VertexAttribute, BufferAddress, VertexFormat, VertexStepMode
};


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn get_descriptor() -> VertexBufferLayout<'static>{
        const ATTRIBUTES: &[VertexAttribute] = &[ 
            VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x3,
            },
            VertexAttribute {
                offset: size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
                format: VertexFormat::Float32x2,
            },
        ];

        let vertext_buffer_layout = VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: ATTRIBUTES
        };

        return vertext_buffer_layout;
    }
}
