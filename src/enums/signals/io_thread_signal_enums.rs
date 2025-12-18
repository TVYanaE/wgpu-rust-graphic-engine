use winit::{
    event::WindowEvent,
};
use crate::{
    enums::{
        winit_event_enum::WinitEvent
    },
};

pub enum IOThreadInputSignal {
    WinitEvent(WinitEvent),
    SendEventBuffer,
    Shutdown,
    Destroy,
}

impl Into<IOThreadInputSignal> for WindowEvent {
    fn into(self) -> IOThreadInputSignal {
        IOThreadInputSignal::WinitEvent(WinitEvent::WindowEvent(self))
    }
}
