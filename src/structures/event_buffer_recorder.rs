use crate::{
    enums::{
        events::{
            external_event_enum::ExternalEvent,
            internal_event_enum::InternalEvent,
        },
    },
    structures::{
        internal_event_buffer::InternalEvenBuffer,
        external_event_buffer::ExternalEventBuffer,
    },
};

pub struct EventBufferRecorder {
    internal_event_buffer: InternalEvenBuffer,
    external_event_buffer: ExternalEventBuffer
}

impl EventBufferRecorder {
    pub fn new() -> Self {
        Self { 
            internal_event_buffer: InternalEvenBuffer::new(),
            external_event_buffer: ExternalEventBuffer::new(),
        }
    }

    pub fn register_external_event(&mut self, external_event: ExternalEvent) {
        self.external_event_buffer.register_external_event(external_event); 
    }

    pub fn register_internal_event(&mut self, internal_event: InternalEvent) {
        self.internal_event_buffer.register_internal_event(internal_event);
    }
    
    pub fn drain_external_event_buffer(&mut self) -> impl Iterator<Item = ExternalEvent> {
        self.external_event_buffer.drain_external_event_buffer()
    }

    pub fn drain_internal_event_buffer(&mut self) -> impl Iterator<Item = InternalEvent> {
        self.internal_event_buffer.drain_internal_buffer()
    }
}
