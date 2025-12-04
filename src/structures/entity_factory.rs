use crate::{
    managers::{
        ecs_manager::ECSManager,
    },
    components::sprite_component::SpriteComponent,
};


pub struct EntityFactory {
    is_triangle_exsist: bool,
}


impl EntityFactory {
    pub fn new() -> Self {
        Self {
            is_triangle_exsist: false
        }
    }

    pub fn create_triagle_tree(&self, ecs_manager_ref: &mut ECSManager) {
        if self.is_triangle_exsist {
            return;
        }

        let triangle_entity = ecs_manager_ref.create_entity();

        let triangle_sprite_component = SpriteComponent {
            sprite_material_name: "default_sprite_material".to_string()
        };

        ecs_manager_ref.add_component_to_entity(triangle_entity, triangle_sprite_component);        
    }
}
