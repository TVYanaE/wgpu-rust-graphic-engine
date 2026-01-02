use std::{
    collections::HashMap,
};
use crate::{
    aliases::MaterialID,
    modules::{
        render_thread::{
            states::{
                gpu_state::{
                    material::Material,
                },
            },
        },
    },
};

pub struct MaterialManager {
    material_storage: HashMap<MaterialID, Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self { 
            material_storage: HashMap::new()
        }
    }
    pub fn add_material(&mut self, material_id: MaterialID, material: Material) {
        self.material_storage.insert(material_id, material);
    }
    pub fn get_material(&self, material_id: MaterialID) -> Option<Material> {

        self.material_storage.get(&material_id).cloned()
    }
}
