use std::{
    sync::Arc,
};
use winit::{
    window::Window,
    dpi::PhysicalSize,
    keyboard::{KeyCode},
};
use shipyard::{
    UniqueView
};
use crate::{
    structures::{
        states::{
            logic_state::LogicState,
            render_state::RenderState,
        },
        timer::Timer,
        batcher::Batcher,
        camera::CameraUniformMatrix,
    },
    enums::{
        errors::EngineError,
    },
};

#[derive(Default)]
pub struct AppState {
    logic_state: Option<LogicState>,
    render_state: Option<RenderState>,
    batcher: Option<Batcher>,
}

impl AppState {
    pub fn init_logic_state(&mut self) -> Result<(), EngineError> {
        let render_state_ref = self.render_state.as_ref().unwrap();

        let logic_state = LogicState::new(
            render_state_ref.material_manager.clone(), 
            render_state_ref.surface_configuration.width as f32, 
            render_state_ref.surface_configuration.height as f32
        );

        self.logic_state = Some(logic_state);

        return Ok(());
    }

    pub async fn init_render_state(&mut self, window: Arc<Window> ) -> Result<(), EngineError> {
        let render_state = RenderState::new(window).await;

        self.render_state = Some(render_state);

        return Ok(());
    }

    pub fn init_batcher(&mut self) -> Result<(), EngineError> {

        let batcher = Batcher::new();
        
        self.batcher = Some(batcher);

        return Ok(());
    }  

    pub fn test_run(&mut self) {
        self.logic_state.as_mut().unwrap().run_tact();
        self.batcher.as_mut().unwrap().batching(&self.logic_state.as_ref().unwrap().world, &self.render_state.as_ref().unwrap().device);
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.render_state.as_ref().unwrap().window.clone()
    }

    pub fn redraw_handle(&mut self) {
        let render_state_ref = self.render_state.as_mut().unwrap();
        let logic_state_ref = self.logic_state.as_ref().unwrap();

        let matrix = logic_state_ref.world.borrow::<UniqueView<CameraUniformMatrix>>().unwrap();

        render_state_ref.queue.write_buffer(
            &render_state_ref.camera_storage.camera_uniform_buffer, 
            0, 
            bytemuck::bytes_of(&matrix.view_projection_matrix));

        let render_batches = self.batcher.as_mut().unwrap().get_render_batches(); 

        render_state_ref.draw_call(render_batches);
    }
}
