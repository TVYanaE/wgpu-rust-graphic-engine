use std::{
    collections::{
        VecDeque,
    },
};
use crate::{ 
    structures::{
        task::Task,  
    },
};

pub struct ControlThreadMessagesBus { 
    task_queue: VecDeque<Task>, 
}

impl ControlThreadMessagesBus {
    pub fn new() -> Self {
        Self { 
            task_queue: VecDeque::with_capacity(8),
        }
    } 

    pub fn push_task_to_bus(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }

    pub fn push_tasks_to_bus(&mut self, tasks: &[Task]) {
        self.task_queue.extend(tasks);
    }
    
    pub fn drain_task_queue(&mut self) -> impl Iterator<Item = Task> {
        self.task_queue.drain(..)
    }
}
