use std::{
    sync::Arc,
};
use winit::{
    window::Window,
    dpi::PhysicalSize,
    keyboard::{KeyCode},
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
        let logic_state = LogicState::new(self.render_state.as_ref().unwrap().material_manager.clone());

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
        let render_batches = self.batcher.as_mut().unwrap().get_render_batches(); 

        self.render_state.as_mut().unwrap().draw_call(render_batches);
    }
}
