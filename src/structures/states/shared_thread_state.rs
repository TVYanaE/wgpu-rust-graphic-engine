use std::{
    sync::{Arc},
};
use crate::{
    structures::{
        managers::material_manager::MaterialManager,
    },
};

pub struct SharedThreadState {
    material_manager: Arc<MaterialManager>,
}

impl SharedThreadState {
    pub fn new(material_manager: Arc<MaterialManager>) -> Self {
        Self { material_manager }
    }
}
