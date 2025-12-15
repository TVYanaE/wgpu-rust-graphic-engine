use std::{
    sync::{Arc, RwLock},
};
use crate::{
    structures::{
        quartzes::{
            physics_quartz::PhysicsQuartz,
            render_quartz::RenderQuartz,
            external_event_quartz::ExternalEventQuarts,
        },
        event_buffer_recorder::EventBufferRecorder,
        winit_event_recorder::WinitEventRecorder,
        winit_data_buffer::WinitDataBuffer,
    },
};

pub struct GlobalQuartz {
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    physics_quartz: PhysicsQuartz,
    render_quartz: RenderQuartz,
    external_event_quartz: ExternalEventQuarts,
}

impl GlobalQuartz {
    pub fn new(
        event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
        winit_event_recorder: Arc<RwLock<WinitEventRecorder>>,
        winit_data_buffer: Arc<RwLock<WinitDataBuffer>>,
    ) -> Self {
        let physics_quartz = PhysicsQuartz::new();
        let render_quartz = RenderQuartz::new();
        let external_event_quartz = ExternalEventQuarts::new(
            event_buffer_recorder.clone(), 
            winit_event_recorder, 
            winit_data_buffer
        );

        Self { event_buffer_recorder, physics_quartz, render_quartz, external_event_quartz }
    }

    pub fn run_tact(&mut self) {
        self.external_event_quartz.run_tact();

        let render_internal_event = self.render_quartz.run_tact();
        let physics_calc_internal_event = self.physics_quartz.run_tact();

        let mut event_buffer_recorder_guard = self.event_buffer_recorder.write().unwrap();

        event_buffer_recorder_guard.register_internal_event(render_internal_event);
        event_buffer_recorder_guard.register_internal_event(physics_calc_internal_event);
    }
}
