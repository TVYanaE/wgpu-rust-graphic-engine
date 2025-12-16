use std::{
    collections::HashMap,
    cell::RefCell,
    sync::Arc,
};
use wgpu::{
    RenderPipeline, RenderPipelineDescriptor, PipelineLayoutDescriptor, PipelineCompilationOptions,
    VertexState, FragmentState,
    Device, SurfaceConfiguration,
    BindGroupLayout,
    ShaderModule,
    PrimitiveState,
    MultisampleState,
};
use crate::{
    enums::render_pipeline_name_enum::RenderPipelineName,
    structures::{
        buffers_layouts::{get_vertex_buffer_layout, get_instance_buffer_layout},
    }, 
};

pub struct RenderPipelineManager {
    render_pipeline_storage: RefCell<HashMap<RenderPipelineName, Arc<RenderPipeline>>>
}

impl RenderPipelineManager {
    pub fn new() -> Self {
        let render_pipeline_storage: RefCell<HashMap<RenderPipelineName, Arc<RenderPipeline>>> = RefCell::new(HashMap::new());  

        Self { render_pipeline_storage:  render_pipeline_storage}
    }
    
    pub fn create_render_pipeline(
        &self,
        render_pipeline_name: RenderPipelineName,
        label: Option<&str>,
        device: &Device,
        surface_configuration: &SurfaceConfiguration,
        bind_groups_layouts: &[&BindGroupLayout],
        shader_module: &ShaderModule,
        primitive_state: PrimitiveState,
        multisample_state: MultisampleState,
    ) {
        let pipe_line_layout_descriptor = PipelineLayoutDescriptor {
            label: label,
            bind_group_layouts: bind_groups_layouts,
            push_constant_ranges: &[],
        };

        let pipe_line_layout = device.create_pipeline_layout(&pipe_line_layout_descriptor);

        let vertex_state = VertexState {
            module: shader_module,
            entry_point: Some("vs_main"),
            buffers: &[
                get_vertex_buffer_layout(),
                get_instance_buffer_layout(),
            ],
            compilation_options: PipelineCompilationOptions::default(), 
        };

        let fragment_state = FragmentState {
            module: shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_configuration.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        };

        let render_pipeline_description = RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipe_line_layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
            cache: None,
        };

        let render_pipeline = Arc::new(device.create_render_pipeline(&render_pipeline_description));
        
        self.render_pipeline_storage.borrow_mut().insert(render_pipeline_name, render_pipeline.clone());
    }
    
    pub fn get_render_pipeline(&self, render_pipeline_name: RenderPipelineName) -> Option<Arc<RenderPipeline>> {
        self.render_pipeline_storage.borrow().get(&render_pipeline_name).cloned()
    }
}
