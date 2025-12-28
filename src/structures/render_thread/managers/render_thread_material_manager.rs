use std::{
    collections::HashMap,
    sync::{RwLock, Arc},
};
use crate::{
    aliases::MaterialID,
    structures::material::Material,
};

pub struct MaterialManager {
    material_storage: RwLock<HashMap<MaterialID, Arc<Material>>>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self { 
            material_storage: RwLock::new(HashMap::new())
        }
    }
    pub fn add_material(&self, material_id: MaterialID, material: Arc<Material>) {
        let mut guard = self.material_storage.write().unwrap();

        guard.insert(material_id, material);
    }
    pub fn get_material(&self, material_id: MaterialID) -> Option<Arc<Material>> {
        let guard = self.material_storage.read().unwrap();

        guard.get(&material_id).cloned()
    }
}
