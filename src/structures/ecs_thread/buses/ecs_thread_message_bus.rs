
use crate::{
    enums::{
        ecs_thread_message_enums::{
            ECSThreadWorldManagerMessage,
        },
    },
};

pub struct ECSThreadMessageBus {
    ecs_thread_world_manager_messages: Vec<ECSThreadWorldManagerMessage>,
}

impl ECSThreadMessageBus {
    pub fn new() -> Self {
        Self {
            ecs_thread_world_manager_messages: Vec::new(),
        }
    }
    
    pub fn push_world_manager_message_to_bus(&mut self, message: ECSThreadWorldManagerMessage) {
        self.ecs_thread_world_manager_messages.push(message);
    }

    pub fn drain_world_manager_message_buffer(&mut self) -> impl Iterator<Item = ECSThreadWorldManagerMessage> {
        self.ecs_thread_world_manager_messages.drain(..)
    }
}
