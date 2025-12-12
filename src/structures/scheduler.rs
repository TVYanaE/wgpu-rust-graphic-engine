use std::{
    sync::{Arc, RwLock}
};  
use crate::{
    structures::{
        event_buffer_recorder::EventBufferRecorder,
    },
    enums::{
        task_enum::Task,
        events::{
            internal_event_enum::InternalEvent,
            external_event_enum::ExternalEvent,
        }
    },
};

pub struct Scheduler {
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    schedule: Vec<Task>,
    first_wave_tasks: Vec<Task>,
    second_wave_tasks: Vec<Task>,
    third_wave_tasks: Vec<Task>
}

impl Scheduler {
    pub fn new(event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>) -> Self {
        Self { 
            event_buffer_recorder: event_buffer_recorder, 
            schedule: Vec::new(),
            first_wave_tasks: Vec::new(),
            second_wave_tasks: Vec::new(),
            third_wave_tasks: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut internal_event_buffer = Vec::with_capacity(20);
        let mut external_event_buffer = Vec::with_capacity(20);

        let mut read_guard = self.event_buffer_recorder.write().unwrap();

        for internal_event in read_guard.drain_internal_event_buffer() {
            internal_event_buffer.push(internal_event);
        }

        for external_event in read_guard.drain_external_event_buffer() {
            external_event_buffer.push(external_event);
        }

        drop(read_guard);
    }

}
