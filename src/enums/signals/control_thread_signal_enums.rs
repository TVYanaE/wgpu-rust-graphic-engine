use winit::{
    event::WindowEvent,
};
use crate::{
    enums::{
        event_enum::Event, 
        winit_event_enum::WinitEvent,
    },
};

pub enum ControlThreadInputSignal {
    Event(Event) 
}

impl From<WindowEvent> for ControlThreadInputSignal {
    fn from(value: WindowEvent) -> Self {
        Self::Event(Event::WinitEvent(WinitEvent::WindowEvent(value)))
    }
}
