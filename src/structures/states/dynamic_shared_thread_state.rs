
use winit::{
    dpi::PhysicalSize
};

pub struct DynamicSharedThreadState {
    physical_size: Option<PhysicalSize<u32>>,
}

impl DynamicSharedThreadState {
    pub fn new() -> Self {
        Self {
            physical_size: None,
        }
    }

    pub fn set_physical_size(&mut self, physical_size: PhysicalSize<u32>) {
        self.physical_size = Some(physical_size);
    }

    pub fn get_physical_size(&self) -> Option<PhysicalSize<u32>> {
        self.physical_size
    }
}
