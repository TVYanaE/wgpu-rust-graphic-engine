
use crate::{
    enums::task_enum::Task
};

pub struct FramePhaseBatch {
    tasks: Vec<Task>
}

impl FramePhaseBatch {
    pub fn new() -> Self {
        Self { 
            tasks: Vec::with_capacity(4)
        }
    }
    
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn drain_batch(&mut self) -> impl Iterator<Item = Task> {
        self.tasks.drain(..)
    }
}
