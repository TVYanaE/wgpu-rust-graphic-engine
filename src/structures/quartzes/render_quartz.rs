use std::{
    sync::{Arc, RwLock},
};
use crate::{
    structures::{
        event_buffer_recorder::EventBufferRecorder,
    },
    enums::{
        events::{
            internal_event_enum::InternalEvent,
        }, 
    },
};

pub struct RenderQuartz {
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>
}

impl RenderQuartz {
    pub fn new(event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>) -> Self {
        Self { event_buffer_recorder: event_buffer_recorder }
    }

    pub fn run(&mut self) {
        let mut guard = self.event_buffer_recorder.write().unwrap();

        guard.register_internal_event(InternalEvent::RequestRender);
    }
}
