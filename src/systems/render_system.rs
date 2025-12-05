use std::{
    rc::Rc,
};
use crate::{
    managers::{
        ecs_manager::ECSManager,
        material_manager::MaterialManager,
    },
    components::{
        sprite_component::SpriteComponent,
    },
    structures::{
        render_items::RenderItem,
    },
};

pub struct RenderSystem {
    material_manager: Rc<MaterialManager>,
    render_items_cache: Vec<RenderItem>, 
}

impl RenderSystem {
    pub fn new(material_manager: Rc<MaterialManager>) -> Self {
        Self { material_manager: material_manager, render_items_cache: Vec::new() }
    }

    pub fn get_render_item_cache(&self) -> &[RenderItem] {
        &self.render_items_cache
    }

    pub fn run(&mut self, ecs_manager: &mut ECSManager) {
        let sprite_storage = ecs_manager.get_storage_mut::<SpriteComponent>();

        if !self.render_items_cache.is_empty() {
            self.render_items_cache.clear();
        }
        
        for (_entity, sprite_component) in sprite_storage.iter() {
            let render_item = RenderItem {
                material: self.material_manager.get_material(sprite_component.material_id).unwrap(),
                instance_size: [sprite_component.size_x, sprite_component.size_y],
                instance_position: [sprite_component.position_x, sprite_component.position_y]
            };  
            
            self.render_items_cache.push(render_item);
        }
    }
}


