use crate::{
    enums::{
        external_event_enum::ExternalEvent,
    },
};

pub struct EventRecorder {
    external_event_buffer: Vec<ExternalEvent>
}

impl EventRecorder {
    pub fn new() -> Self {
        Self { external_event_buffer: Vec::new() }
    }

    pub fn collect_external_event(&mut self, window_event: impl Into<ExternalEvent>) {
        self.external_event_buffer.push(window_event.into());
    }

    pub fn drain_external_event_buffer(&mut self) -> impl Iterator<Item = ExternalEvent> {
        self.external_event_buffer.drain(..)
    }
}
