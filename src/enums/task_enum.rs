use winit::{
    dpi::PhysicalSize,
};

#[derive(Debug, Clone, Copy)]
pub enum Task {
    // Render pipeline
    PrepareRenderObjects,
    PrepareLogicalRenderBatches,
    Batching,
    Draw,
    // Another pipeline
    Resize(PhysicalSize<u32>),
}


// For DAG
impl Task {
    pub fn dependencies(&self) -> Vec<Task> {
        match self {
            Task::PrepareRenderObjects => { vec![] },
            Task::PrepareLogicalRenderBatches => { vec![Task::PrepareRenderObjects] },
            Task::Batching => { vec![Task::PrepareLogicalRenderBatches] },
            Task::Draw => { vec![Task::Batching] },
            Task::Resize(_) => { vec![] }, 
        }
    }
}
