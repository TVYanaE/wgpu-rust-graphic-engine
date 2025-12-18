use crate::{
    structures::{
        scheduler::Scheduler
    },
    enums::{ 
        event_enum::Event,
        task_enum::Task,
    },
};

pub struct ControlThreadState {
    scheduler: Scheduler,
    task_buffer: Vec<Task>,
}

impl ControlThreadState {
    pub fn new() -> Self {
        Self { 
            scheduler: Scheduler::new(),
            task_buffer: Vec::with_capacity(32),
        }
    }
    pub fn run_logic(&mut self, event_buffer: impl Iterator<Item = Event>) {}
    
    pub fn run_drawing(&mut self) {}
}
