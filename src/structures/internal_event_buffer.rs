use crate::{
    enums::{
        events::internal_event_enum::InternalEvent,
    },
};

pub struct InternalEvenBuffer {
    internal_events: Vec<InternalEvent> 
}

impl InternalEvenBuffer {
    pub fn new() -> Self {
        Self { 
            internal_events: Vec::with_capacity(100) 
        }
    }

    pub fn register_internal_event(&mut self, internal_event: InternalEvent) {
        self.internal_events.push(internal_event);
    }

    pub fn drain_internal_buffer(&mut self) -> impl Iterator <Item = InternalEvent> {
        self.internal_events.drain(..)
    }

    pub fn clear_internal_buffer(&mut self) {
        self.internal_events.clear();
    } 
}
