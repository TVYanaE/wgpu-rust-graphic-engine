use crate::{
    managers::{
        ecs_manager::ECSManager,
    },
    components::sprite_component::SpriteComponent,
};


pub struct EntityFactory {
    is_object_exsist: bool,
}


impl EntityFactory {
    pub fn new() -> Self {
        Self {
            is_object_exsist: false
        }
    }

    pub fn create_test_object(&mut self, ecs_manager_ref: &mut ECSManager) {
        if self.is_object_exsist {
            return;
        }

        let test_object_entity = ecs_manager_ref.create_entity();

        let test_object_sprite_component = SpriteComponent {
            size_x: 0.5,
            size_y: 0.5,
            position_x: -0.5,
            position_y: -0.5,
            material_id: 0,
        };

        ecs_manager_ref.add_component_to_entity(test_object_entity, test_object_sprite_component);

        self.is_object_exsist = true;
    }
}
