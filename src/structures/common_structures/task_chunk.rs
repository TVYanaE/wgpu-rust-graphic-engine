use std::{
    collections::{HashSet},
};
use crate::{
    enums::{
        task_type_enum::TaskType,
    },
    structures::{
        task::Task,
        task_chunk_time_cost::TaskChunkTimeCost,
    },
};


pub struct TaskChunk {
    tasks: Vec<Task>,
    task_chunk_time_cost: Option<TaskChunkTimeCost>, 
    forbiden_task_list: HashSet<TaskType>
}

impl TaskChunk {
    pub fn new() -> Self {
        Self { 
            tasks: Vec::new(), 
            task_chunk_time_cost: None,
            forbiden_task_list: HashSet::new(),
        }
    }

    pub fn try_insert_task(&mut self, task: Task) -> bool {
        let requirements = task.task_type.get_requirements();

        for requirement in requirements.iter() {
            if self.forbiden_task_list.contains(requirement) {
                return false;
            }
        }

        if let Some(task_chink_time_cost) = self.task_chunk_time_cost {
            if !(task_chink_time_cost.time_cost_type == task.task_time_cost.time_cost_type) {
                return false;
            }
            
            self.tasks.push(task);
            self.forbiden_task_list.extend(requirements); 

            if task.task_time_cost.time_cost > task_chink_time_cost.time_cost {
                self.task_chunk_time_cost.as_mut().unwrap().time_cost = task.task_time_cost.time_cost;
            }

            return true;
        }
        else {
            self.task_chunk_time_cost = Some(TaskChunkTimeCost { 
                time_cost_type: task.task_time_cost.time_cost_type,
                time_cost: task.task_time_cost.time_cost, 
            });

            self.tasks.push(task);
            self.forbiden_task_list.extend(requirements); 

            return true;
        }
    }

    pub fn get_time_cost(&self) -> Option<TaskChunkTimeCost> {
        self.task_chunk_time_cost
    } 

    pub fn drain_chunk(&mut self) -> impl Iterator<Item = Task> {
        self.tasks.drain(..)
    }
}
