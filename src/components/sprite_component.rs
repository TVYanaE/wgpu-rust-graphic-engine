use crate::{
    aliases::MaterialID,
};

#[derive(Clone, Copy)]
pub struct SpriteComponent {
    pub position_x: f32,
    pub position_y: f32,
    pub material_id: MaterialID,
}
