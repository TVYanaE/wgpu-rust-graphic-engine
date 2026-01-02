use glam::{
    Vec3, vec3
};
use shipyard::{Component};

#[derive(Component, Debug, Clone, Copy)]
pub struct PositionComponent {
    pub position: Vec3,
}


impl Default for PositionComponent {
    fn default() -> Self {
        Self { position: vec3(0.0, 0.0, 0.0) }
    }
}
