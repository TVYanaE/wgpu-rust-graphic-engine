use std::{
    rc::Rc,
};
use wgpu::{
    BindGroup, RenderPipeline
};

#[derive(Debug, Clone)]
pub struct Material {
    pub bind_group: Rc<BindGroup>,
    pub render_pipeline: Rc<RenderPipeline>,
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}
