use winit::{
    event::WindowEvent,
};
use crate::{ 
    enums::{
        task_enum::Task,
    },
};

pub struct Scheduler { 
    schedule: Vec<Task>,
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
}
