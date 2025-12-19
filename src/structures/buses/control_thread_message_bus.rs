use std::{
    collections::{
        VecDeque,
    },
};
use crate::{ 
    structures::{
        task_chunk::TaskChunk,
        messages::{
            event_message::EventMessage,
            task_message::TaskMessage,
        },
    },
    enums::{
    },
};

pub struct ControlThreadMessagesBus {
    event_messages_queue: VecDeque<EventMessage>,
    task_messages_queue: VecDeque<TaskMessage>, 
    task_chunk_messages_queue: VecDeque<TaskChunk>,
}

impl ControlThreadMessagesBus {
    pub fn new() -> Self {
        Self { 
            event_messages_queue: VecDeque::with_capacity(8), 
            task_messages_queue: VecDeque::with_capacity(8),
            task_chunk_messages_queue: VecDeque::with_capacity(8),
        }
    }
    
    pub fn push_event_message_to_bus(&mut self, event_message: EventMessage) {
        self.event_messages_queue.push_back(event_message);
    }

    pub fn drain_event_message_queue(&mut self) -> impl Iterator<Item = EventMessage> {
        self.event_messages_queue.drain(..)
    }

    pub fn push_task_message_to_bus(&mut self, task_message: TaskMessage) {
        self.task_messages_queue.push_back(task_message);
    }
    
    pub fn drain_task_message_queue(&mut self) -> impl Iterator<Item = TaskMessage> {
        self.task_messages_queue.drain(..)
    }
}
