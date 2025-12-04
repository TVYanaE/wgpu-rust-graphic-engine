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
    },
    enums::{
        errors::EngineError,
    },
};

#[derive(Default)]
pub struct AppState {
    ecs_state: Option<ECSState>,
    render_state: Option<RenderState>
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

    pub fn redraw_handle(&mut self) -> Result<(), EngineError> {

        self.render_state.as_mut().unwrap().render();
        return Ok(());
    }

    pub fn resize_handle(&mut self, physical_size: PhysicalSize<u32>) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn keyboard_input_handle(
        &mut self, 
        event_loop: &ActiveEventLoop, 
        key_code: KeyCode, 
        key_is_pressed: bool
    ) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn run_fixed_time_tasks(&mut self, timer: &Timer) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn run_real_time_tasks(&mut self, timer: &Timer) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn render_prepare(&mut self) -> Result<(), EngineError> { 
        self.ecs_state.as_mut().unwrap().render_prepared();
        return Ok(());
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.render_state.as_ref().unwrap().window.clone()
    }
}
