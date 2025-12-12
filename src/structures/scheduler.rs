use std::{
    sync::{Arc, RwLock}, 
    collections::{
        HashMap
    },
}; 
use chrono::{
    Duration
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
    time_menu: HashMap<Task, Duration>,
    render_time_budget: Duration,
    logic_time_budget: Duration,
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    schedule: Vec<Task>,
    first_wave_tasks: Vec<Task>,
    second_wave_tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new(event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>) -> Self {
        let mut time_menu = HashMap::new(); 

        time_menu.insert(Task::RenderFrame, Duration::milliseconds(8));

        Self {
            time_menu: time_menu, 
            render_time_budget: Duration::milliseconds(8),
            logic_time_budget: Duration::milliseconds(8),
            event_buffer_recorder: event_buffer_recorder, 
            schedule: Vec::with_capacity(128),
            first_wave_tasks: Vec::with_capacity(32),
            second_wave_tasks: Vec::with_capacity(32),
        }
    }

    pub fn events_buffer_handling(&mut self) {
        let mut internal_event_buffer = Vec::with_capacity(20);
        let mut external_event_buffer = Vec::with_capacity(20);

        let mut guard = self.event_buffer_recorder.write().unwrap();

        for internal_event in guard.drain_internal_event_buffer() {
            internal_event_buffer.push(internal_event);
        }

        for external_event in guard.drain_external_event_buffer() {
            external_event_buffer.push(external_event);
        }

        drop(guard);
        
        
        
    }

    pub fn create_schedule(&mut self) {
        
    }

}
