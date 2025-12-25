
use crate::{
    enums::{
        execute_thread_message_enum::ExecuteThreadMessage,
    },
};

pub struct ExecuteurThreadMessageBus {
    executeur_thread_messages: Vec<ExecuteThreadMessage>
}

impl ExecuteurThreadMessageBus {
    pub fn new() -> Self {
        Self { 
            executeur_thread_messages: Vec::new(),
        }
    }

    pub fn push_message_to_bus(&mut self, message: ExecuteThreadMessage) {
        self.executeur_thread_messages.push(message);
    }

    pub fn drain_message_buffer(&mut self) -> impl Iterator<Item = ExecuteThreadMessage> {
        self.executeur_thread_messages.drain(..)
    }
}
