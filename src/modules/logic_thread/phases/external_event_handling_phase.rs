use std::{
    collections::{VecDeque},
};
use super::super::{
    events::{
        external_event::{
            ExternalEvent,
        },
    },
    managers::{
        world_manager::WorldManager
    },
}; 

pub fn external_event_handling_phase(
    external_event_queue: &mut VecDeque<ExternalEvent>,
    world_manager: &WorldManager,
) {
    while let Some(external_event) = external_event_queue.pop_front() {
        match external_event {
            ExternalEvent::KeyEvent(key_event) => {
                world_manager.key_event_handling(key_event);
            } 
        }
    }
}


