/* use wgpu::{
    VertexState, 
    FragmentState, 
    ShaderModule, ShaderModuleDescriptor, ShaderSource,
    Device,
    PipelineCompilationOptions,
};
use crate::{
    vertex::Vertex,
};

pub struct DefaultShaderPrograms<'a>{
    vertex_state: VertexState<'a>,
    fragment_state: FragmentState<'a>,
    shader_module: ShaderModule,
}

impl DefaultShaderPrograms {
    pub fn new(device: &Device) -> Self {
        
        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/default_shader.wgsl").into()), 
        };

        let shader_module = device.create_shader_module(shader_module_descriptor);
  
        let vertex_state = VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::get_descriptor()],
            compilation_options: PipelineCompilationOptions::default(), 
        };

        let fragment_state = FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        };
 

        Self { 
            vertex_state: (), 
            fragment_state: (), 
            shader_module: () 
        }
    }
}
 */
