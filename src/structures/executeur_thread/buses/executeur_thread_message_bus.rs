
use crate::{
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadTaskControllerMessage,
            ExecuteurThreadTimeControllerMessage,
            ExecuteurThreadTimeManagerMessage,
            ExecuteurThreadGlobalExecuteurMessage,
        },
    },
};

pub struct ExecuteurThreadMessageBus {
    executeur_thread_task_controller_messages: Vec<ExecuteurThreadTaskControllerMessage>,
    executeur_thread_time_controller_messages: Vec<ExecuteurThreadTimeControllerMessage>,
    executeur_thread_time_manager_messages: Vec<ExecuteurThreadTimeManagerMessage>,
    executeur_thread_global_executeur_messages: Vec<ExecuteurThreadGlobalExecuteurMessage>,
}

impl ExecuteurThreadMessageBus {
    pub fn new() -> Self {
        Self { 
            executeur_thread_task_controller_messages: Vec::new(),
            executeur_thread_time_controller_messages: Vec::new(),
            executeur_thread_time_manager_messages: Vec::new(),
            executeur_thread_global_executeur_messages: Vec::new(),
        }
    }

    pub fn push_task_controller_message_to_bus(&mut self, message: ExecuteurThreadTaskControllerMessage) {
        match message {
            ExecuteurThreadTaskControllerMessage::ScheduleReady => {
                if !self.executeur_thread_task_controller_messages.contains(&message) {
                    self.executeur_thread_task_controller_messages.push(message);
                }
            }, 
        } 
    }

    pub fn push_time_controller_message_to_bus(&mut self, message: ExecuteurThreadTimeControllerMessage) {
        self.executeur_thread_time_controller_messages.push(message);
    }

    pub fn push_time_manager_message_to_bus(&mut self, message: ExecuteurThreadTimeManagerMessage) {
        self.executeur_thread_time_manager_messages.push(message);
    }

    pub fn push_global_executeur_message_to_bus(&mut self, message: ExecuteurThreadGlobalExecuteurMessage) {
        self.executeur_thread_global_executeur_messages.push(message);
    }

    pub fn drain_task_controller_message_buffer(&mut self) -> impl Iterator<Item = ExecuteurThreadTaskControllerMessage> {
        self.executeur_thread_task_controller_messages.drain(..)
    }

    pub fn drain_time_controller_message_buffer(&mut self) -> impl Iterator<Item = ExecuteurThreadTimeControllerMessage> {
        self.executeur_thread_time_controller_messages.drain(..)
    }

    pub fn drain_time_manager_message_buffer(&mut self) -> impl Iterator<Item = ExecuteurThreadTimeManagerMessage> {
        self.executeur_thread_time_manager_messages.drain(..)
    }

    pub fn drain_global_executeur_message_buffer(&mut self) -> impl Iterator<Item = ExecuteurThreadGlobalExecuteurMessage> {
        self.executeur_thread_global_executeur_messages.drain(..)
    }
}
