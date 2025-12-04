use std::{
    rc::Rc,
};
use wgpu::{
    BindGroup, RenderPipeline, Buffer
};

#[derive(Debug, Clone)]
pub struct RenderBatch {
    pub render_pipeline: Rc<RenderPipeline>,
    pub bind_group: Rc<BindGroup>,
    pub instance_buffer: Buffer,
    pub instance_count: u32,
}
