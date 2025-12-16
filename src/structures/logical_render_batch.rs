use std::{
    sync::Arc,
};
use crate::{
    structures::{
        material::Material,
        buffers_layouts::InstanceVertex, 
    },
};

#[derive(Clone)]
pub struct LogicalRenderBatch {
    pub material: Arc<Material>,
    pub instances: Vec<InstanceVertex>,
}
