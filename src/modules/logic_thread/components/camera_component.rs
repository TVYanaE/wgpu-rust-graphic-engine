use glam::{
    Vec3,
};
use shipyard::{
    Component
};

#[derive(Component, Debug, Clone)]
pub struct CameraComponent {
    pub camera_position: Vec3,
    pub view_target: Vec3,
    pub up: Vec3,
    pub bottom_bound: f32,
    pub top_bound: f32,
    pub far: f32,
    pub near: f32,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self { 
            camera_position: Vec3 { x: 0.0, y: 0.0, z: 2.0 },
            view_target: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            bottom_bound: -1.0,
            top_bound: 1.0,
            far: 2.0,
            near: 0.0,
        } 
    }
}
