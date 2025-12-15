use crate::{
    structures::{ 
        descriptors::event_descriptor::EventDescriptor,
    },
    enums::{
        events::internal_event_enum::InternalEvent
    },
};

pub struct PhysicsQuartz {
}

impl PhysicsQuartz {
    pub fn new() -> Self {
        Self { }
    }

    pub fn run_tact(&mut self) -> InternalEvent {
        let event_descriptor = EventDescriptor {
            write_components: Vec::new(),
            read_components: Vec::new(),
        };

        InternalEvent::RequestPhysicsCalculation(event_descriptor)
    }
}
