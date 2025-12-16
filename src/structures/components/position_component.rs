use shipyard::{Component};

#[derive(Component, Debug, Clone, Copy)]
pub struct PositionComponent {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
}


impl Default for PositionComponent {
    fn default() -> Self {
        Self { position_x: 0.0, position_y: 0.0, position_z: 0.0 }
    }
}
