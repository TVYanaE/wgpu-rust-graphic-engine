use std::{
    collections::{
        HashMap,
    },
    iter::once,
    mem::take,
};
use crate::{
    structures::{
        frame_phase::FramePhase,
    },
    enums::{
        frame_phase_name_enum::FramePhaseName,
        task_enum::Task,
        task_name_enum::TaskName
    },
};


pub struct Schedule {
    frame_phase_map: HashMap<TaskName, FramePhaseName>, 
    input_frame_phase: FramePhase,
    resize_frame_phase: FramePhase,
    logic_frame_phase: FramePhase,
    physics_frame_phase: FramePhase,
    ai_frame_phase: FramePhase,
    gpu_frame_phase: FramePhase, 
    render_frame_phase: FramePhase,
}

impl Schedule {
    pub fn new() -> Self {
        let input_frame_phase = FramePhase::new();
        let resize_frame_phase = FramePhase::new();
        let logic_frame_phase = FramePhase::new();
        let physics_frame_phase = FramePhase::new(); 
        let ai_frame_phase = FramePhase::new();
        let gpu_frame_phase = FramePhase::new(); 
        let render_frame_phase = FramePhase::new();

        let mut frame_phase_map: HashMap<TaskName, FramePhaseName> = HashMap::new();
 
        frame_phase_map.insert(TaskName::RenderFrame, FramePhaseName::RenderProcessing);
        frame_phase_map.insert(TaskName::PhysicsCalculation, FramePhaseName::PhysicsProcessing);
        frame_phase_map.insert(TaskName::CameraReconfigurate, FramePhaseName::ResizeProcessing);

        Self {
            frame_phase_map,
            input_frame_phase, 
            resize_frame_phase, 
            logic_frame_phase, 
            physics_frame_phase, 
            ai_frame_phase, 
            gpu_frame_phase, 
            render_frame_phase 
        }
    }
    pub fn add_task(&mut self, task: Task) {
        let task_descriptor = task.get_task_descriptor().unwrap();

        let phase_name = self.frame_phase_map.get(&task_descriptor.task_name).unwrap();

        match phase_name {
            FramePhaseName::InputProcessing => { self.input_frame_phase.add_task(task); },
            FramePhaseName::ResizeProcessing => { self.resize_frame_phase.add_task(task); },
            FramePhaseName::LogicProcessing => { self.logic_frame_phase.add_task(task); },
            FramePhaseName::PhysicsProcessing => { self.physics_frame_phase.add_task(task); },
            FramePhaseName::AIProcessing => { self.ai_frame_phase.add_task(task); },
            FramePhaseName::GPUDataProcessing => { self.gpu_frame_phase.add_task(task); },
            FramePhaseName::RenderProcessing => { self.render_frame_phase.add_task(task); },
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = FramePhase> {
        once(take(&mut self.input_frame_phase))
            .chain(once(take(&mut self.resize_frame_phase)))
            .chain(once(take(&mut self.logic_frame_phase)))
            .chain(once(take(&mut self.physics_frame_phase)))
            .chain(once(take(&mut self.ai_frame_phase)))
            .chain(once(take(&mut self.gpu_frame_phase)))
            .chain(once(take(&mut self.render_frame_phase)))
    }
}

