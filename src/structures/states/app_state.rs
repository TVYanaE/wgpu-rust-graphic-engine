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
        event_buffer::EventBuffer,
    },
    enums::{
        errors::EngineError,
        engine_event_enum::EngineEvent,
    },
};

#[derive(Default)]
pub struct AppState {
    logic_state: Option<LogicState>,
    render_state: Option<RenderState>,
    batcher: Option<Batcher>,
    event_buffer: Option<EventBuffer>,
}

impl AppState {
    pub fn init_logic_state(&mut self) -> Result<(), EngineError> {
        let logic_state = LogicState::new();

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

    pub fn init_event_buffer(&mut self) -> Result<(), EngineError> {
        
        let event_buffer = EventBuffer::new();

        self.event_buffer = Some(event_buffer);

        return Ok(());
    }

    pub fn init_systems(&mut self) -> Result<(), EngineError> {
        
        let material_manager = self.render_state.as_ref().unwrap().material_manager.clone();

        self.logic_state.as_mut().unwrap().init_systems(
            material_manager,
        );
        return Ok(());
    }

    pub fn render_prepare(&mut self) -> Result<(), EngineError> { 
        let render_state = self.render_state.as_ref().unwrap();
        self.logic_state.as_mut().unwrap().render_prepare(
            render_state.window_size.width as f32,
            render_state.window_size.height as f32,
        );
        let render_items = self.logic_state.as_ref().unwrap().get_render_items();
        self.batcher.as_mut().unwrap().batching(render_items, &render_state.device); 

        return Ok(());
    }

    pub fn redraw_handle(&mut self) -> Result<(), EngineError> {

        let render_state = self.render_state.as_mut().unwrap();

        let view_projection_matrix = self.logic_state.as_ref().unwrap().get_view_project_matrix();

        let camera_uniform_matrix = CameraUniformMatrix::from_mat4(view_projection_matrix); 
         
        render_state.queue.write_buffer(&render_state.camera_storage.camera_uniform_buffer, 0, bytemuck::cast_slice(&[camera_uniform_matrix])); 

        let render_batches = self.batcher.as_ref().unwrap().get_render_batches();

        render_state.draw_call(render_batches);

        return Ok(());
    }

    pub fn resize_handle(&mut self, physical_size: PhysicalSize<u32>) -> Result<(), EngineError> {
        self.render_state.as_mut().unwrap().reconfigure_surface(physical_size);
        self.logic_state.as_mut().unwrap().resize_handle(physical_size.width as f32, physical_size.height as f32);
        
        return Ok(());
    }

    pub fn keyboard_input_handle(
        &mut self, 
        key_code: KeyCode, 
        key_is_pressed: bool
    ) -> Result<(), EngineError> {
        
        let event = EngineEvent::from_keyboard_event(key_code, key_is_pressed);      

        self.event_buffer.as_mut().unwrap().register_event(event);

        return Ok(());
    }

    pub fn run_fixed_time_tasks(&mut self, _timer: &Timer) -> Result<(), EngineError> {

        return Ok(());
    }

    pub fn run_real_time_tasks(&mut self, _timer: &Timer) -> Result<(), EngineError> {
        self.logic_state.as_mut().unwrap().run_real_time_tasks(); 
        
        return Ok(());
    } 

    pub fn get_window(&self) -> Arc<Window> {
        self.render_state.as_ref().unwrap().window.clone()
    }
}
