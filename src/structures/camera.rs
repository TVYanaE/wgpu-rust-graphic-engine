use glam::{
    Mat4,
    Vec3,
};
use shipyard::{
    Unique
};
use wgpu::{
    BindGroup,
    Buffer,
};

#[repr(C)]
#[derive(Unique, Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformMatrix {
    pub view_projection_matrix: [[f32;4]; 4],
}

impl CameraUniformMatrix {
    pub fn new() -> Self {
        Self { view_projection_matrix: Mat4::IDENTITY.to_cols_array_2d() }
    }
    pub fn from_mat4(view_projection_matrix: &Mat4) -> Self {
        Self { view_projection_matrix: view_projection_matrix.to_cols_array_2d()}
    }
}

#[derive(Unique, Debug, Clone, Copy)]
pub struct Camera {
    pub camera_position: Vec3,
    pub view_target: Vec3,
    pub up: Vec3,
    pub bottom_bound: f32,
    pub top_bound: f32,
    pub far: f32,
    pub near: f32,
    pub aspect: f32,
}

pub struct CameraStorage {
    pub camera_bind_group: BindGroup,
    pub camera_uniform_buffer: Buffer,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Camera { 
            camera_position: Vec3 { x: 0.0, y: 0.0, z: 2.0 },
            view_target: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            bottom_bound: -1.0,
            top_bound: 1.0,
            far: 10.0,
            near: 0.0,
            aspect: width / height,
        }
    } 
}
