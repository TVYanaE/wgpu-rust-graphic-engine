use glam::{
    Mat4,
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
pub struct ViewProjectionUniformMatrixCache {
    pub view_projection_matrix: [[f32;4]; 4],
}

impl Default for ViewProjectionUniformMatrixCache {
    fn default() -> Self {
        Self { view_projection_matrix: Mat4::IDENTITY.to_cols_array_2d() }
    }
}

pub struct CameraStorage {
    pub camera_bind_group: BindGroup,
    pub camera_uniform_buffer: Buffer,
}


