use wgpu::{
    VertexState, FragmentState,
};

pub trait ShaderProgram {
    fn get_vertex_state<'a>(&'a self) -> VertexState<'a>;
    fn get_fragment_state<'a>(&'a self) -> FragmentState<'a>;
}
