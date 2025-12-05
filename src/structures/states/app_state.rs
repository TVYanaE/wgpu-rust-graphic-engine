use std::{
    sync::Arc,
};
use winit::{
    window::Window,
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode},
};
use crate::{
    structures::{
        states::{
            ecs_state::ECSState,
            render_state::RenderState,
        },
        timer::Timer,
        batcher::Batcher,
    },
    enums::{
        errors::EngineError,
    },
};

#[derive(Default)]
pub struct AppState {
    ecs_state: Option<ECSState>,
    render_state: Option<RenderState>,
    batcher: Option<Batcher>,
}

impl AppState {
    pub fn init_ecs_state(&mut self) -> Result<(), EngineError> {
        let ecs_state = ECSState::new();

        self.ecs_state = Some(ecs_state);

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

    pub fn init_systems(&mut self) -> Result<(), EngineError> {
        
        let material_manager = self.render_state.as_ref().unwrap().material_manager.clone();

        self.ecs_state.as_mut().unwrap().init_systems(material_manager);
        return Ok(());
    }

    pub fn redraw_handle(&mut self) -> Result<(), EngineError> {

        let render_batches = self.batcher.as_ref().unwrap().get_render_batches();

        self.render_state.as_mut().unwrap().draw_call(render_batches);

        return Ok(());
    }

    pub fn resize_handle(&mut self, _physical_size: PhysicalSize<u32>) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn keyboard_input_handle(
        &mut self, 
        _event_loop: &ActiveEventLoop, 
        _key_code: KeyCode, 
        _key_is_pressed: bool
    ) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn run_fixed_time_tasks(&mut self, _timer: &Timer) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn run_real_time_tasks(&mut self, _timer: &Timer) -> Result<(), EngineError> {
        self.ecs_state.as_mut().unwrap().run_real_time_tasks(); 
        
        return Ok(());
    }

    pub fn render_prepare(&mut self) -> Result<(), EngineError> { 
        self.ecs_state.as_mut().unwrap().render_prepare();
        let render_items = self.ecs_state.as_ref().unwrap().get_render_items();
        self.batcher.as_mut().unwrap().batching(render_items, &self.render_state.as_ref().unwrap().device); 

        return Ok(());
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.render_state.as_ref().unwrap().window.clone()
    }
}
