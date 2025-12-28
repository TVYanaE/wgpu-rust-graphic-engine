use shipyard::{Component};
use crate::{
    aliases::MaterialID,
};

#[derive(Component, Clone, Copy)]
pub struct SpriteComponent { 
    pub material_id: MaterialID,
}

impl Default for SpriteComponent {
    fn default() -> Self {
        Self { material_id: 1 }
    }
}
