use std::{
    rc::Rc,
};
use crate::{
    aliases::{EntityID},
    structures::{
        render_items::RenderItem,
        managers::{
            material_manager::MaterialManager,
        },
        components::{
            sprite_component::SpriteComponent,
            position_component::PositionComponent,
            size_component::SizeComponent,
        },
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

    pub fn run(
        &mut self,
        position_component: &PositionComponent,
        size_component: &SizeComponent,
        sprite_component: &SpriteComponent,
        _entity_id: EntityID,
    ) {
        let material = self.material_manager.get_material(sprite_component.material_id).unwrap();
        let render_item = RenderItem {
            instance_position: [position_component.position_x, position_component.position_y, position_component.position_z],
            instance_size: [size_component.size_x, size_component.size_y, size_component.size_z],
            material: material
        }; 

        self.render_items_cache.push(render_item);
    }
}

