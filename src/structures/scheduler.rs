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
        frame_phase::FramePhase,
        frame_phase_batch::FramePhaseBatch,
    },
    enums::{
        task_enum::Task,
        task_name_enum::TaskName,
        events::{
            internal_event_enum::InternalEvent,
            external_event_enum::ExternalEvent,
        },
        frame_phase_name_enum::FramePhaseName,
    },
};

pub struct Scheduler {
    time_menu: HashMap<TaskName, Duration>,
    wave_map: HashMap<TaskName, WaveNumber>,
    render_time_budget: Duration,
    logic_time_budget: Duration,
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    schedule: Vec<FramePhase>,
    first_wave_tasks: Vec<Task>,
    second_wave_tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new(event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>) -> Self {
        let mut time_menu = HashMap::new(); 

        time_menu.insert(TaskName::RenderFrame, Duration::milliseconds(8));

        let mut wave_map = HashMap::new();

        wave_map.insert(TaskName::RenderFrame, 1 as WaveNumber);

        let mut schedule = Vec::new();

        let input_frame_phase = FramePhase::new(FramePhaseName::InputProcessing); 
        let logic_frame_phase = FramePhase::new(FramePhaseName::LogicProcessing);
        let physics_frame_phase = FramePhase::new(FramePhaseName::PhysicsProcessing); 
        let ai_frame_phase = FramePhase::new(FramePhaseName::AIProcessing);
        let gpu_frame_phase = FramePhase::new(FramePhaseName::GPUDataProcessing); 
        let render_frame_phase = FramePhase::new(FramePhaseName::RenderProcessing);

        schedule.push(input_frame_phase);
        schedule.push(logic_frame_phase);
        schedule.push(physics_frame_phase);
        schedule.push(ai_frame_phase);
        schedule.push(gpu_frame_phase);
        schedule.push(render_frame_phase);

        Self {
            time_menu: time_menu,
            wave_map: wave_map,
            render_time_budget: Duration::milliseconds(8),
            logic_time_budget: Duration::milliseconds(8),
            event_buffer_recorder: event_buffer_recorder, 
            schedule: schedule,
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
        
        for internal_event in internal_event_buffer {
            let task = Task::from(internal_event);
            
            let task_name = task.get_task_name();
            
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

        for 
    }

    pub fn create_schedule(&mut self) {
        
    }

}
