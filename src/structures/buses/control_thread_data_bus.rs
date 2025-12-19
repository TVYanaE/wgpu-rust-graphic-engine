use crate::{
    enums::{
        event_enum::Event
    },
    structures::{
        task::Task,
    },
};

pub struct ControlThreadDataBus {
    event_data_buffer: Vec<Event>,
    task_data_buffer: Vec<Task>,
}

impl ControlThreadDataBus {
    pub fn new() -> Self {
        Self { 
            event_data_buffer: Vec::with_capacity(8),
            task_data_buffer: Vec::with_capacity(8),
        }
    }

    pub fn push_event_to_bus(&mut self, event: Event) -> usize {
        self.event_data_buffer.push(event);
        self.event_data_buffer.len() - 1
    }

    pub fn drain_event_data_buffer(&mut self) -> impl Iterator<Item = Event> {
        self.event_data_buffer.drain(..)
    }

    pub fn push_task_to_bus(&mut self, task: Task) -> usize {
        self.task_data_buffer.push(task);
        self.task_data_buffer.len() - 1
    }

    pub fn drain_task_data_buffer(&mut self) -> impl Iterator<Item = Task> {
        self.task_data_buffer.drain(..)
    }
} 


