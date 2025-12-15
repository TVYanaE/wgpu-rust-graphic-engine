use crate::{
    structures::{
        descriptors::event_descriptor::EventDescriptor,
    },
    enums::{
        events::{
            internal_event_enum::InternalEvent,
        }, 
        component_name_enum::ComponentName,
    },
};

pub struct RenderQuartz {
}

impl RenderQuartz {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn run_tact(&mut self) -> InternalEvent {
        let event_descriptor = EventDescriptor {
            read_components: vec![ComponentName::Position, ComponentName::Size, ComponentName::Sprite],
            write_components: vec![],
        };

        InternalEvent::RequestRender(event_descriptor)
    }
}
