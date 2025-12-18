
use crate::{
    structures::{
        task_chunk::TaskChunk,
    }, 
    enums::task_enum::Task,
};

pub struct GlobalExecuter {
 
}

impl GlobalExecuter {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn execute_schedule(&self, schedule: impl Iterator<Item = TaskChunk>) {
        for mut task_chunk in schedule {
            for task in task_chunk.drain_chunk() {
                match task {
                    Task::Init => {},
                    Task::PrepareRenderState => {},
                    Task::DrawRenderState => {},
                    Task::Resize(physical_size) => {},
                } 
            }
        } 
    }
}
