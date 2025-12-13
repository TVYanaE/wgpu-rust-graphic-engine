use std::{
    sync::{Arc, RwLock},
};
use winit::{
    event::WindowEvent,
};
use crate::{
    structures::{
        event_buffer_recorder::EventBufferRecorder
    },
    enums::{
        events::{
            winit_event_enum::WinitEvent,
            external_event_enum::ExternalEvent,
        },
    },
};

pub struct ExternalEventQuarts {
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    input_buffer_event: Vec<WinitEvent>,
}

impl ExternalEventQuarts {
    pub fn new(event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>) -> Self {
        Self { 
            event_buffer_recorder: event_buffer_recorder,
            input_buffer_event: Vec::new() 
        }
    }
    pub fn register_winit_window_event(&mut self, window_event: WindowEvent) {
        self.input_buffer_event.push(WinitEvent::from(window_event)); 
    }
    pub fn run(&mut self) {
        let mut external_event_buffer = Vec::new();

        for winit_event in self.input_buffer_event.drain(..) {
            let external_event = ExternalEvent::from(winit_event);

            external_event_buffer.push(external_event);
        }

        let mut guard = self.event_buffer_recorder.write().unwrap();

        for external_event in external_event_buffer {
            guard.register_external_event(external_event);
        }
    }
}
