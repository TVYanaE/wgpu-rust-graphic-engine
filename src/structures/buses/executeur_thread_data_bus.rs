use std::{
    collections::{VecDeque},
};

use crate::{
    structures::{
        task::Task,
    },
};

pub struct ExecuteurThreadDataBus {
    schedule: VecDeque<Task>
}

impl ExecuteurThreadDataBus {
    pub fn new() -> Self {

        Self { 
            schedule: VecDeque::new(),
        }
    }

    pub fn add_task_to_bus(&mut self, task: Task) {
        self.schedule.push_back(task);
    }

    pub fn add_tasks_to_bus(&mut self, tasks: impl Iterator<Item = Task>) {
        self.schedule.extend(tasks);
    }

    pub fn get_latest_task(&mut self) -> Option<Task> {
        self.schedule.pop_front()
    }

    pub fn get_oldest_task(&mut self) -> Option<Task> {
        self.schedule.pop_back()
    }

    pub fn drain_task_buffer(&mut self) -> impl Iterator<Item = Task> {
        self.schedule.drain(..)
    }
}
