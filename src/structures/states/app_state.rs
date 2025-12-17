use std::{
    sync::{Arc, RwLock},
};
use winit::{
    window::Window,
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
        batcher::Batcher,
        camera::CameraUniformMatrix,
        event_recorder::EventRecorder,
        scheduler::Scheduler,
    },
    enums::{
        errors::EngineError,
        external_event_enum::ExternalEvent,
    },
};

#[derive(Default)]
pub struct AppState {
    logic_state: Option<LogicState>,
    render_state: Option<RenderState>,
    batcher: Option<Batcher>,
    event_recorder: Option<EventRecorder>,
    scheduler: Option<Scheduler>,
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

    

    pub fn init_batcher(&mut self) -> Result<(), EngineError> {

        let batcher = Batcher::new();
        
        self.batcher = Some(batcher);

        return Ok(());
    }  

    pub fn init_event_recorder(&mut self) -> Result<(), EngineError> {
        let event_recorder = EventRecorder::new();

        self.event_recorder = Some(event_recorder);

        return Ok(());
    }

    pub fn init_scheduler(&mut self) -> Result<(), EngineError> {
        let scheduler = Scheduler::new(); 

        self.scheduler = Some(scheduler);

        return Ok(());
    }

    pub fn external_event_handling(&mut self, window_event: impl Into<ExternalEvent>) -> Result<(), EngineError> {
        self.event_recorder.as_mut().unwrap().collect_external_event(window_event);

        return Ok(());
    }

    pub fn run_tact(&mut self) {
        let external_event_buffer = self.event_recorder.as_mut().unwrap().drain_external_event_buffer();
        self.scheduler.as_ref().unwrap().run_tact(external_event_buffer);

        self.logic_state.as_mut().unwrap().run_tact();
        self.batcher.as_mut().unwrap().batching(&self.logic_state.as_ref().unwrap().world, &self.render_state.as_ref().unwrap().device);
        self.get_window().request_redraw();
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
