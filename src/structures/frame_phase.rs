
use crate::{
    structures::{
        frame_phase_batch::FramePhaseBatch
    }, 
    enums::{
        task_enum::Task,
    },
};

pub struct FramePhase {
    frame_phase_batches: Vec<FramePhaseBatch>
}

impl FramePhase {
    pub fn new() -> Self {
        Self { 
            frame_phase_batches: Vec::with_capacity(4) 
        }
    }

    pub fn add_task(&mut self, task: Task) {
        let task_descriptor = task.get_task_descriptor().unwrap();
        let write_components = &task_descriptor.write_components;
        
        if self.frame_phase_batches.is_empty() {
            let frame_phase_batch = FramePhaseBatch::new();

            self.frame_phase_batches.push(frame_phase_batch);
        }

        for frame_phase_batch in self.frame_phase_batches.iter_mut() {
            if frame_phase_batch.conflict_with_write_components(write_components) {
                continue;
            }
            else {
                frame_phase_batch.add_task(task);
                return;
            }
        }
        
        let mut frame_phase_batch = FramePhaseBatch::new();

        frame_phase_batch.add_task(task);

        self.frame_phase_batches.push(frame_phase_batch);

    }

    pub fn drain_frame_phase(&mut self) -> impl Iterator<Item = FramePhaseBatch> {
        self.frame_phase_batches.drain(..)
    }
}

impl Default for FramePhase {
    fn default() -> Self {
        Self::new()
    }
}
