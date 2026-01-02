use std::mem::size_of;
use bytemuck::{Pod, Zeroable};
use wgpu::{
    VertexBufferLayout, VertexAttribute, BufferAddress, VertexFormat, VertexStepMode
};


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3], 
    pub texture_coordinates: [f32; 2]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InstanceVertex {
    pub instance_position: [f32; 3],
    pub instance_size: [f32; 3],
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2]
}

const VERTEX_ATTRIBUTES: &[VertexAttribute] = &[ 
    VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: VertexFormat::Float32x3,
    },
    VertexAttribute {
        offset: size_of::<[f32; 3]>() as BufferAddress,
        shader_location: 1,
        format: VertexFormat::Float32x2
    },    
];

const INSTANCE_ATTRIBUTES: &[VertexAttribute] = &[
    VertexAttribute {
        offset: 0,
        shader_location: 2,
        format: VertexFormat::Float32x3
    },
    VertexAttribute {
        offset: size_of::<[f32; 3]>() as BufferAddress,
        shader_location: 3,
        format: VertexFormat::Float32x3,
    },
    VertexAttribute {
        offset: size_of::<[f32; 6]>() as BufferAddress,
        shader_location: 4,
        format: VertexFormat::Float32x2,
    },
    VertexAttribute {
        offset: size_of::<[f32; 8]>() as BufferAddress,
        shader_location: 5,
        format: VertexFormat::Float32x2
    },
];

pub fn get_vertex_buffer_layout() -> VertexBufferLayout<'static>{
    VertexBufferLayout {
        array_stride: size_of::<Vertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: VERTEX_ATTRIBUTES
    }
}

pub fn get_instance_buffer_layout() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
        array_stride: size_of::<InstanceVertex>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: INSTANCE_ATTRIBUTES
    }
}
