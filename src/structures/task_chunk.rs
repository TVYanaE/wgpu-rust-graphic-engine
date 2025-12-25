use std::{
    collections::{HashSet},
};
use crate::{
    enums::{
        task_type_enum::TaskType,
    },
    structures::{
        task::Task,
    },
};


pub struct TaskChunk {
    tasks: Vec<Task>,
    forbiden_task_list: HashSet<TaskType>
}

impl TaskChunk {
    pub fn new() -> Self {
        Self { tasks: Vec::new(), forbiden_task_list: HashSet::new() }
    }

    pub fn try_insert_task(&mut self, task: Task) -> bool {
        let requirements = task.task_type.get_requirements();

        for requirement in requirements.iter() {
            if self.forbiden_task_list.contains(requirement) {
                return false;
            }
        }

        self.tasks.push(task);
        self.forbiden_task_list.extend(requirements);
        return true;
    }

    pub fn drain_chunk(&mut self) -> impl Iterator<Item = Task> {
        self.tasks.drain(..)
    }
}
