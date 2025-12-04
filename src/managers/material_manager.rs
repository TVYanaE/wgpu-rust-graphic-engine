use std::{
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
};
use crate::{
    aliases::MaterialID,
    structures::material::Material,
};

pub struct MaterialManager {
    material_storage: RefCell<HashMap<MaterialID, Rc<Material>>>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self { 
            material_storage: RefCell::new(HashMap::new())
        }
    }
    pub fn add_material(&self, material_id: MaterialID, material: Rc<Material>) {
        self.material_storage.borrow_mut().insert(material_id, material);
    }
    pub fn get_material(&self, material_id: MaterialID) -> Option<Rc<Material>> {
        self.material_storage.borrow().get(&material_id).cloned()
    }
}
