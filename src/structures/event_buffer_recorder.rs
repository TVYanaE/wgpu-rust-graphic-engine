use crate::{
    enums::{
        events::{
            internal_event_enum::InternalEvent,
        },
    }, 
};

pub struct EventBufferRecorder {
    internal_events: Vec<InternalEvent>
}

impl EventBufferRecorder {
    pub fn new() -> Self {
        Self { 
            internal_events: Vec::with_capacity(16)
        }
    }

    pub fn register_internal_event(&mut self, internal_event: InternalEvent) {
        self.internal_events.push(internal_event);
    } 

    pub fn drain_internal_event_buffer(&mut self) -> impl Iterator<Item = InternalEvent> {
        self.internal_events.drain(..)
    }
}
