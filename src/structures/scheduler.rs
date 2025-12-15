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
    aliases::{WaveNumber},
    structures::{
        event_buffer_recorder::EventBufferRecorder,
        schedule::Schedule,
        frame_phase::FramePhase,
    },
    enums::{
        task_enum::Task,
        task_name_enum::TaskName,
    },
};

pub struct Scheduler {
    time_menu: HashMap<TaskName, Duration>,
    wave_map: HashMap<TaskName, WaveNumber>,
    time_budget: Duration,
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    schedule: Schedule,
    first_wave_tasks: Vec<Task>,
    second_wave_tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new(event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>) -> Self {
        let mut time_menu = HashMap::new(); 

        time_menu.insert(TaskName::RenderFrame, Duration::milliseconds(8));
        time_menu.insert(TaskName::CameraReconfigurate, Duration::milliseconds(3));
        time_menu.insert(TaskName::PhysicsCalculation, Duration::milliseconds(3));

        let mut wave_map = HashMap::new();

        wave_map.insert(TaskName::RenderFrame, 1 as WaveNumber);
        wave_map.insert(TaskName::CameraReconfigurate, 1 as WaveNumber);
        wave_map.insert(TaskName::PhysicsCalculation, 1 as WaveNumber);

        let schedule = Schedule::new();

        Self {
            time_menu: time_menu,
            wave_map: wave_map,
            time_budget: Duration::milliseconds(16),
            event_buffer_recorder: event_buffer_recorder, 
            schedule: schedule,
            first_wave_tasks: Vec::with_capacity(32),
            second_wave_tasks: Vec::with_capacity(32),
        }
    }

    pub fn events_buffer_handling(&mut self) {
        let mut internal_event_buffer = Vec::with_capacity(20);

        let mut guard = self.event_buffer_recorder.write().unwrap();

        for internal_event in guard.drain_internal_event_buffer() {
            internal_event_buffer.push(internal_event);
        } 

        drop(guard);
        
        for internal_event in internal_event_buffer {
            let task = Task::from(internal_event);
            
            if let Some(task_name) = task.get_task_name() {
                if let Some(wave_number) = self.wave_map.get(&task_name) {
                    match wave_number {
                        1 => { self.first_wave_tasks.push(task); },
                        2 => { self.second_wave_tasks.push(task); },
                        _ => { panic!("Panic in scheduler events_buffer_handling"); },
                    }
                }
                else {
                    panic!("Panic in scheduler events_buffer_handling");
                }
            }
            else {
                continue;
            } 
        } 
    }

    pub fn fill_schedule(&mut self) {
        for first_wave_task in self.first_wave_tasks.drain(..) {
            let task_name = first_wave_task.get_task_name().unwrap();
            let time_cost = self.time_menu.get(&task_name).unwrap().clone();
                       
            self.schedule.add_task(first_wave_task);

            self.time_budget -= time_cost; 
        }

        if self.time_budget <= Duration::zero() {
            return;
        } 
        
        while (self.time_budget > Duration::zero()) && (!self.second_wave_tasks.is_empty()) {
            let second_wave_task = self.second_wave_tasks.pop().unwrap();
            let task_name = second_wave_task.get_task_name().unwrap();
            let time_cost = self.time_menu.get(&task_name).unwrap().clone();

            self.schedule.add_task(second_wave_task);

            self.time_budget -= time_cost;
        }
    }
    
    pub fn drain_schedule(&mut self) -> impl Iterator<Item = FramePhase> {
        self.time_budget = Duration::milliseconds(16);
        self.schedule.drain() 
    }
}
