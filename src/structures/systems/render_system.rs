use std::{
    rc::Rc,
};
use crate::{ 
    structures::{
        render_items::RenderItem,
        managers::{
            material_manager::MaterialManager,
        },
        components::{
            sprite_component::SpriteComponent,
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

    pub fn run(&mut self) { 
    }
}

