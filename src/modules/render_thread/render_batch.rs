use std::{
    sync::Arc,
};
use wgpu::{
    BindGroup, RenderPipeline, Buffer
};

#[derive(Debug, Clone)]
pub struct RenderBatch {
    pub render_pipeline: Arc<RenderPipeline>,
    pub bind_group: Arc<BindGroup>,
    pub instance_buffer: Buffer,
    pub instance_count: u32,
}
