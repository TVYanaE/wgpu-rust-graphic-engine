use std::{
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
};
use crate::{
    structures::materials::sprite_material::SpriteMaterial
};

pub struct SpriteMaterialManager {
    spripte_material_storage: RefCell<HashMap<String, Rc<SpriteMaterial>>>,
}

impl SpriteMaterialManager {
    pub fn new() -> Self {
        Self { 
            spripte_material_storage: RefCell::new(HashMap::new())
        }
    }
    pub fn add_sprite_material(
        &self,
        sprite_material_name: &str,
        sprite_material: Rc<SpriteMaterial>,
    ) {
        self.spripte_material_storage.borrow_mut().insert(sprite_material_name.to_string(), sprite_material);
    }
    pub fn get_sprite_material(
        &self,
        sprite_material_name: &str
    ) -> Option<Rc<SpriteMaterial>> {
        self.spripte_material_storage.borrow().get(sprite_material_name).cloned()
    } 
}
