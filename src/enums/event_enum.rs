use crate::{
    enums::{ 
        winit_event_enum::WinitEvent,
    },
};

pub enum Event {
    WinitEvent(WinitEvent),
    Shutdown,
}
