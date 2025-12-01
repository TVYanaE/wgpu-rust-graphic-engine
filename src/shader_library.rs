use wgpu::{
    ShaderModule, ShaderModuleDescriptor, ShaderSource,
};
use crate::{
    core_state::CoreState, 
};

pub struct ShaderLibrary{
    pub basic_shader_module: ShaderModule,
}

impl ShaderLibrary {
    pub fn new(core_state: &CoreState) -> Self {
        
        let basic_shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Basic shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/basic_shader.wgsl").into()), 
        };

        let basic_shader_module = core_state.device.create_shader_module(basic_shader_module_descriptor); 
     
        Self { 
            basic_shader_module: basic_shader_module
        }
    }
}

/* impl ShaderProgram for DefaultShaderPrograms {
    fn get_vertex_state<'a>(
        &'a self, 
        buffers: &'a [VertexBufferLayout]
    ) -> VertexState<'a> {
        let vertex_state = VertexState {
            module: &self.shader_module,
            entry_point: Some("vs_main"),
            buffers: buffers,
            compilation_options: PipelineCompilationOptions::default(), 
        };
        return vertex_state;
    }

    fn get_fragment_state<'a>(
        &'a self, 
        targets: &'a [Option<ColorTargetState>]
    ) -> FragmentState<'a> {
        let fragment_state = FragmentState {
            module: &self.shader_module,
            entry_point: Some("fs_main"),
            targets: targets,
            compilation_options: PipelineCompilationOptions::default(),
        };
        return fragment_state;
    }
} */
