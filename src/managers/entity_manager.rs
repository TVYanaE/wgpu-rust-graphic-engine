use glam::{Vec3};
use crate::{
    managers::{
        entity_id_manager::EntityIDManager,
    },
    components::{
        sprite_component::SpriteComponent,
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
        if self.is_test_object_exsist {
            return;
        }

        let test_object_entity_1 = self.entity_id_manager.create_entity();

        let test_object_sprite_component_1 = SpriteComponent {
            size_x: 1.0,
            size_y: 1.0,
            size_z: 0.0,
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            material_id: 0,
        };

        let test_object_entity_2 = entity_manager.create_entity();

        let test_object_sprite_component_2 = SpriteComponent {
            size_x: 1.0,
            size_y: 1.0,
            size_z: 0.0,
            position_x: 1.0,
            position_y: 1.0,
            position_z: 0.0,
            material_id: 1,
        };

        ecs_manager_ref.add_component_to_entity(test_object_entity_1, test_object_sprite_component_1);
        ecs_manager_ref.add_component_to_entity(test_object_entity_2, test_object_sprite_component_2);

        self.is_test_object_exsist = true;
    } 
}
