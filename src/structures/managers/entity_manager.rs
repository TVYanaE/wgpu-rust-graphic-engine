use glam::{Vec3};
use crate::{
    structures::{
        managers::{
            entity_id_manager::EntityIDManager,
        },
        components::{
            sprite_component::SpriteComponent,
        },
    }, 
};


pub struct EntityManager {
    entity_id_manager: EntityIDManager,
    is_test_object_exsist: bool,
    is_test_camera_exsist: bool,
}


impl EntityManager {
    pub fn new() -> Self {
        let entity_id_manager = EntityIDManager::new();

        Self {
            entity_id_manager: entity_id_manager,
            is_test_object_exsist: false,
            is_test_camera_exsist: false,
        }
    }

    pub fn create_test_object(&mut self) {
         
    } 
}
