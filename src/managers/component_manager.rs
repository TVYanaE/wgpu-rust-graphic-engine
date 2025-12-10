use crate::{
    aliases::{EntityID, MaterialID,}
};


pub struct ComponentManager {
    active_entities: Vec<EntityID>,
    // Render archetype fields
    position_x: Vec<f32>,
    position_y: Vec<f32>,
    position_z: Vec<f32>,
    size_x: Vec<f32>,
    size_y: Vec<f32>,
    size_z: Vec<f32>,
    material_id: Vec<MaterialID>
}

impl ComponentManager {
    pub fn new() -> Self {
        Self { 
            active_entities: Vec::new(), 
            position_x: Vec::new(), 
            position_y: Vec::new(), 
            position_z: Vec::new(), 
            size_x: Vec::new(), 
            size_y: Vec::new(), 
            size_z: Vec::new(), 
            material_id: Vec::new(),
        }
    }

    pub fn add_entity_to_render_archetype(&mut self)
}
