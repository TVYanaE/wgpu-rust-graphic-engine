#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Transform2D {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
}
