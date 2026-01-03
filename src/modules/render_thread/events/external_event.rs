use winit::{
    dpi::PhysicalSize,
};



#[derive(Debug)]
pub enum ExternalEvent {
    Resize(PhysicalSize<u32>),
    RedrawRequested,
}
