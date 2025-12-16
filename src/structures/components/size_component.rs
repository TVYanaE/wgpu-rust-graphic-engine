use shipyard::{Component};

#[derive(Component, Debug, Clone, Copy)]
pub struct SizeComponent {
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
}

impl Default for SizeComponent {
    fn default() -> Self {
        Self { size_x: 1.0, size_y: 1.0, size_z: 0.0 }
    }
}
