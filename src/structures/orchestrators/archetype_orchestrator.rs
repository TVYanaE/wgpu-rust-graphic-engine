
use crate::{
    structures::{
        archetypes::{
            object_archetype::ObjectArchetype,
        },
    }, 
};

pub struct ArcherypeOrchestrator {
    object_archetype: ObjectArchetype
}


impl ArcherypeOrchestrator {
    pub fn new() -> Self {
        Self { 
            object_archetype: ObjectArchetype::new(), 
        }
    } 

    pub fn start_system_polling(&self) {
          
    }
}
