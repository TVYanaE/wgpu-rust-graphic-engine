
use winit::{
    dpi::PhysicalSize
};
use crate::{
    structures::{
        task_chunk::TaskChunk,
    },
};

pub struct DynamicSharedThreadState {
    physical_size: Option<PhysicalSize<u32>>,
    schedule: Vec<TaskChunk>,
}

impl DynamicSharedThreadState {
    pub fn new() -> Self {
        Self {
            physical_size: None,
            schedule: Vec::new(),
        }
    }

    pub fn set_physical_size(&mut self, physical_size: PhysicalSize<u32>) {
        self.physical_size = Some(physical_size);
    }

    pub fn get_physical_size(&self) -> Option<PhysicalSize<u32>> {
        self.physical_size
    }

    pub fn set_schedule(&mut self, schedule: impl Iterator<Item = TaskChunk>) {
        self.schedule.extend(schedule);
    }

    pub fn drain_schedule(&mut self) -> impl Iterator<Item = TaskChunk> {
        self.schedule.drain(..)
    }
}
