use crate::{
    structures::materials::sprite_material::SpriteMaterial,
};

#[derive(Clone)]
pub struct Sprite {
    pub sprite_material: SpriteMaterial,
    pub uv_x: f32,
    pub uv_y: f32,
}
