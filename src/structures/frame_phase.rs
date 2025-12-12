
use crate::{
    structures::{
        frame_phase_batch::FramePhaseBatch
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

    pub fn add_frame_phase_batch(&mut self, frame_phase_batch: FramePhaseBatch) {
        self.frame_phase_batches.push(frame_phase_batch);
    }

    pub fn drain_frame_phase(&mut self) -> impl Iterator<Item = FramePhaseBatch> {
        self.frame_phase_batches.drain(..)
    }
}
