use winit::{
    event::{
        WindowEvent,
    },
};
use crate::{
    enums::{
        events::{
            winit_event_enum::WinitEvent
        },
    },
};

pub struct WinitEventRecorder {
    winit_event_buffer: Vec<WinitEvent>
}

impl WinitEventRecorder {
    pub fn new() -> Self {
        Self {
            winit_event_buffer: Vec::with_capacity(16) 
        }
    }
    pub fn register_window_winit_event(&mut self, window_event: WindowEvent) {
        self.winit_event_buffer.push(WinitEvent::from(window_event));
    }
    pub fn drain_winit_event_buffer(&mut self) -> impl Iterator<Item = WinitEvent> {
        self.winit_event_buffer.drain(..)
    }
}
