use glam::{
    Mat4,
    Vec3,
};
use wgpu::{
    BindGroup,
    Buffer,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformMatrix {
    view_projection_matrix: [[f32;4]; 4],
}

impl CameraUniformMatrix {
    pub fn new() -> Self {
        Self { view_projection_matrix: Mat4::IDENTITY.to_cols_array_2d() }
    }
    pub fn from_mat4(view_projection_matrix: &Mat4) -> Self {
        Self { view_projection_matrix: view_projection_matrix.to_cols_array_2d()}
    }
}

pub struct CameraStorage {
    pub camera_bind_group: BindGroup,
    pub camera_uniform_buffer: Buffer,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub camera_position: Vec3,
    pub view_target: Vec3,
    pub up: Vec3,
    pub left_bound: f32,
    pub right_bound: f32,
    pub bottom_bound: f32,
    pub top_bound: f32,
    pub far: f32,
    pub near: f32,
}
