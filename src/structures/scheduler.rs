use crate::{ 
    enums::{
        task_enum::Task,
    },
    structures::{
        task_chunk::TaskChunk,
    },
};

pub struct Scheduler { 
    schedule: Vec<TaskChunk>, // Executer must do this from start 
    first_wave: Vec<Task>,
    second_wave: Vec<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {  
            schedule: Vec::new(),
            first_wave: Vec::new(),
            second_wave: Vec::new(),
        }
    }

    // TODO!: use WAVES and time budget
    pub fn create_schedule(&mut self, tasks: Vec<Task>){
        for task in tasks {
            let mut inserted = false;

            for task_chunk in self.schedule.iter_mut() {
                if task_chunk.try_insert_task(task) {
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                let mut new_task_chunk = TaskChunk::new();
                new_task_chunk.try_insert_task(task);
                self.schedule.push(new_task_chunk);
            }
        }
    }
    
    pub fn drain_schedule(&mut self) -> impl Iterator<Item = TaskChunk> {
        self.schedule.drain(..)
    }
}
