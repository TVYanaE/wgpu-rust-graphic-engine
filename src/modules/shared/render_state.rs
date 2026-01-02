use glam::{
    Vec3,
};
use shipyard::{
    Unique
};
use crate::{ 
    aliases::MaterialID,
};

#[derive(Debug, Clone, Copy)]
pub struct RenderItem {
    pub instance_position: [f32; 3],
    pub instance_size: [f32; 3],
    pub material_id: MaterialID,
}

#[derive(Debug, Clone, Copy)]
pub struct ActivieCamera {
    pub camera_position: Vec3,
    pub view_target: Vec3,
    pub up: Vec3,
    pub bottom_bound: f32,
    pub top_bound: f32,
    pub far: f32,
    pub near: f32,
}

#[derive(Unique, Default, Clone,)]
pub struct RenderState {
    pub render_item_cache: Option<Vec<RenderItem>>,
    pub activie_camera: Option<ActivieCamera>, 
}
