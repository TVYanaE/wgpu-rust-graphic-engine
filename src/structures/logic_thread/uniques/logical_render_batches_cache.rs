use shipyard::{
    Unique,
};
use crate::{
    structures::logical_render_batch::LogicalRenderBatch
};

#[derive(Clone, Unique)]
pub struct LogicalRenderBatchesCache {
    pub logical_render_batches: Vec<LogicalRenderBatch>
}

impl LogicalRenderBatchesCache {
    pub fn new() -> Self {
        Self { logical_render_batches: Vec::new() }
    }
}
