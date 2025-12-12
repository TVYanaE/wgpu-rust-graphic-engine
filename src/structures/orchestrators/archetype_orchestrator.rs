
use crate::{
    structures::{
        archetypes::{
            object_archetype::ObjectArchetype,
        },
    },
    enums::{
        events::internal_event_enum::InternalEvent,
    },
};

pub struct ArcherypeOrchestrator {
    inernal_events_buffer: Vec<InternalEvent>,
    object_archetype: ObjectArchetype
}


impl ArcherypeOrchestrator {
    pub fn new() -> Self {
        Self { 
            inernal_events_buffer: Vec::new(),
            object_archetype: ObjectArchetype::new(), 
        }
    }

    pub fn drain_internal_events_buffer(&mut self) -> impl Iterator<Item = InternalEvent> {
        self.inernal_events_buffer.drain(..)
    }

    pub fn start_system_polling(&self) {
          
    }
}
