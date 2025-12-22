use crate::{
    enums::{ 
        winit_event_enum::WinitEvent,
    },
};

#[derive(Debug, Clone)]
pub enum Event {
    WinitEvent(WinitEvent), 
}
