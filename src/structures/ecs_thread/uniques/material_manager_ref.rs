use std::{
    sync::Arc,
};
use shipyard::{
    Unique
};
use crate::{
    structures::{
        managers::material_manager::MaterialManager,
    },
};

#[derive(Unique)]
pub struct MaterialManagerRef {
    pub material_manager: Arc<MaterialManager>
}

impl MaterialManagerRef {
    pub fn new(material_manager: Arc<MaterialManager>) -> Self {
        Self { material_manager }
    }
}
