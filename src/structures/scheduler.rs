use winit::{
    event::WindowEvent,
};
use crate::{ 
    enums::{
        task_enum::Task,
        input_event_enum::InputEvent,
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
    pub fn frame_start<IEI>(&mut self, input_events: IEI) 
    where 
        IEI: Iterator<Item = InputEvent>
    {
        for input_event in input_events {
            match input_event {
                InputEvent::WindowEvent(window_event) => {
                    match window_event {
                        WindowEvent::Resized(physical_size) => {
                            self.first_wave.push(Task::Resize(physical_size));
                        },
                        WindowEvent::RedrawRequested => {
                            self.
                            self.first_wave.push(Task::RedrawTask);
                        },
                        _ => { println!("Unhadling Window Event") }
                    }
                },
                _ => { println!("Unhadling Window Event") }
            }
        }

        self.create_schedule();
    }

    fn create_schedule(&mut self) {
          
    }
}
