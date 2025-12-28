use std::{
    collections::VecDeque,
};
use crate::{
    enums::{
        io_event_enum::IOEvent,
    },  
    structures::{
        task::Task,
    },
};

pub struct ControlThreadDataBus {
    task_queue: VecDeque<Task>,
    io_event_queue: VecDeque<IOEvent>
}


impl ControlThreadDataBus {
    pub fn new() -> Self {
        Self { 
            task_queue: VecDeque::new(),
            io_event_queue: VecDeque::new() 
        }
    }

    pub fn push_io_events(&mut self, io_events: impl Iterator<Item = IOEvent>) {
        self.io_event_queue.extend(io_events);
    }

    pub fn push_io_event(&mut self, io_event: IOEvent) {
        self.io_event_queue.push_back(io_event);
    }

    pub fn drain_io_event_queue(&mut self) -> impl Iterator<Item = IOEvent> {
        self.io_event_queue.drain(..)
    }

    pub fn get_latest_event(&mut self) -> Option<IOEvent> {
        self.io_event_queue.pop_front()
    }

    pub fn get_oldest_event(&mut self) -> Option<IOEvent> {
        self.io_event_queue.pop_back()
    }

    pub fn push_tasks(&mut self, tasks: impl Iterator<Item = Task>) {
        self.task_queue.extend(tasks);
    }

    pub fn push_task(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }

    pub fn drain_task_queue(&mut self) -> impl Iterator<Item = Task> {
        self.task_queue.drain(..)
    }

    pub fn get_latest_task(&mut self) -> Option<Task> {
        self.task_queue.pop_front()
    }

    pub fn get_oldest_task(&mut self) -> Option<Task> {
        self.task_queue.pop_back()
    }
}
