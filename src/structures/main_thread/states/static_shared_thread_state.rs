use std::{
    sync::{Arc},
};
use crate::{
    structures::{
        managers::material_manager::MaterialManager,
    },
};

pub struct StaticSharedThreadState {
    material_manager: Arc<MaterialManager>,
}

impl StaticSharedThreadState {
    pub fn new(material_manager: Arc<MaterialManager>) -> Self {
        Self { material_manager }
    }
}
