
use flume::{
    Receiver,
}; 
use crate::{
    structures::{
        scheduler::Scheduler
    },
    enums::{
        input_event_enum::InputEvent
    },
};

pub struct ControlThreadState {
    pub input_event_channel_receiver: Receiver<InputEvent>,
    pub scheduler: Scheduler,
    pub input_event_buffer: Vec<InputEvent>,
}

impl ControlThreadState {
    pub fn new(input_event_channel_receiver: Receiver<InputEvent>,) -> Self {
        Self { 
            scheduler: Scheduler::new(),
            input_event_channel_receiver: input_event_channel_receiver,
            input_event_buffer: Vec::with_capacity(32),
        }
    }
    pub fn frame_start(&mut self) {
        self.scheduler.frame_start(self.input_event_buffer.drain(..));   
    }
}
