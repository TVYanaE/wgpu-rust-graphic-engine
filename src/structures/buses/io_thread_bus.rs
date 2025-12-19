use crate::{
    enums::{
        event_enum::Event,
    },
};

pub struct IOThreadBus {
    event_buffer: Vec<Event>
}

impl IOThreadBus {
    pub fn new() -> Self {
        Self { event_buffer: Vec::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.event_buffer.push(event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Event> {
        self.event_buffer.drain(..)
    }
}


