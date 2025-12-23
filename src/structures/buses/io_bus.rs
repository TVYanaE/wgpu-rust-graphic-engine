use crate::{
    enums::{
        io_event_enum::IOEvent,
    },
};

pub struct IOBus {
    io_event_buffer: Vec<IOEvent>
}

impl IOBus {
    pub fn new() -> Self {
        Self { io_event_buffer: Vec::new() }
    }

    pub fn push(&mut self, io_event: IOEvent) {
        self.io_event_buffer.push(io_event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = IOEvent> {
        self.io_event_buffer.drain(..)
    }
}


