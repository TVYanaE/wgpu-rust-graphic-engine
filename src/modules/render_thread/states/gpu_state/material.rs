use std::{
    sync::Arc,
};
use wgpu::{
    BindGroup, RenderPipeline
};

#[derive(Debug, Clone)]
pub struct Material {
    pub bind_group: Arc<BindGroup>,
    pub render_pipeline: Arc<RenderPipeline>,
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}
