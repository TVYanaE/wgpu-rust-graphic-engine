use std::{
    collections::{HashMap},
    time::{Duration},
};

use crate::{
    enums::{
        task_type_enum::TaskType,
    }, 
};


pub struct TimeMenu {
    task_costs: HashMap<TaskType, Duration>,
    
}

impl TimeMenu {
    pub fn new() -> Self {
        let mut task_costs: HashMap<TaskType, Duration> = HashMap::new(); 

        task_costs.insert(TaskType::Init, Duration::from_millis(0));
        task_costs.insert(TaskType::Shutdown, Duration::from_millis(0));
        task_costs.insert(TaskType::Resize, Duration::from_millis(0));
        task_costs.insert(TaskType::LogicCalculation, Duration::from_millis(0));
        task_costs.insert(TaskType::PrepareRenderState, Duration::from_millis(0));
        task_costs.insert(TaskType::DrawRenderState, Duration::from_millis(0));
        task_costs.insert(TaskType::UnknowTask, Duration::from_millis(0)); 

        Self { 
            task_costs: task_costs,
        }
    }

    pub fn get_task_cost(&self, task: TaskType) -> Option<Duration> {
        self.task_costs.get(&task).cloned()
    }

    pub fn get_time_menu(&self) -> HashMap<TaskType, Duration> {
        self.task_costs.clone()
    }

    pub fn set_task_cost(&mut self, task: TaskType, time: Duration) {
        self.task_costs.insert(task, time).unwrap();
    } 
}
