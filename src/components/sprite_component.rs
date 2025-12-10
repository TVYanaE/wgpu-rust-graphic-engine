use crate::{
    aliases::MaterialID,
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct SpriteComponent { 
    pub material_id: MaterialID,
}


impl Default for SpriteComponent {
    fn default() -> Self {
        Self { material_id: 0 }
    }
}
