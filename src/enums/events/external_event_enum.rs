
use winit::{
    dpi::{PhysicalSize},
    event::{WindowEvent},
}; 
use crate::{
    enums::{
        events::{
            winit_event_enum::WinitEvent,
        },
    },
};

#[derive(Debug, Clone, Copy)]
pub enum ExternalEvent {
    ResizedRequest(PhysicalSize<u32>),
    Unhandling,
}


impl From<WinitEvent> for ExternalEvent {
    fn from(value: WinitEvent) -> Self {
        match value {
            WinitEvent::WindowEvent(window_event) => {
                match window_event {
                    WindowEvent::Resized(physical_size) => { Self::ResizedRequest(physical_size) },
                    _ => { Self::Unhandling }
                }
            }
        }
    }
}
