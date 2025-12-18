use winit::{
    event::WindowEvent,
    dpi::PhysicalSize,
};
use crate::{
    enums::{
        event_enum::Event,
        winit_event_enum::WinitEvent,
    },
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Task {
    // Init 
    Init,
    // Render pipeline
    PrepareRenderState, 
    DrawRenderState,
    // Another pipeline
    Resize(PhysicalSize<u32>),
}

impl Task {
    pub fn get_requirements(&self) -> Vec<Task> {
        match self {
            Task::Init => { vec![] },
            Task::PrepareRenderState => { vec![] },
            Task::DrawRenderState => { vec![] },
            Task::Resize(_) => { vec![] },
        }
    }

    pub fn events_to_tasks(event_buffer: impl Iterator<Item = Event>) -> Vec<Task> {
        let mut tasks = Vec::new();

        for event in event_buffer {
            match event {
                Event::Shutdown => {},
                Event::WinitEvent(winit_event) => {
                    match winit_event {
                        WinitEvent::WindowEvent(window_event) => {
                            match window_event {
                                WindowEvent::Resized(physical_size) => { tasks.push(Task::Resize(physical_size)); }, 
                                _ => {},
                            }
                        }, 
                    }
                },
            }
        }
        return tasks;
    }
}


