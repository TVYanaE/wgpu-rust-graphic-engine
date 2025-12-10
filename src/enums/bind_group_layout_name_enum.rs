#[derive(
    Debug, Hash, 
    Eq, PartialEq,
    Clone, Copy,
)]
pub enum BindGroupLayoutName{
    DefaultBindGroupLayout,
    CameraBindGroupLayout,
}
