use crate::{
    enums::{
        events::{external_event_enum::ExternalEvent},
    },
};

pub struct ExternalEventBuffer {
    external_events: Vec<ExternalEvent>,
}

impl ExternalEventBuffer {
    pub fn new() -> Self {
        Self { 
            external_events: Vec::with_capacity(100),  
        }
    }

    pub fn register_external_event(&mut self, external_event: ExternalEvent) {
        self.external_events.push(external_event);
    }

    pub fn drain_external_event_buffer(&mut self) -> impl Iterator<Item = ExternalEvent> {
        self.external_events.drain(..)
    }

    pub fn clear_external_buffer(&mut self) {
        self.external_events.clear();
    }
}
