use std::{
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
};
use wgpu::{
    Device,
    Sampler, SamplerDescriptor, AddressMode, FilterMode,
};
use crate::{
    enums::sampler_name_enum::SamplerName,
};


pub struct SamplerManager {
    sampler_storage: RefCell<HashMap<SamplerName, Rc<Sampler>>>,
}

impl SamplerManager {
    pub fn new(device: &Device) -> Self {
        let sampler_storage= RefCell::new(HashMap::new());
        
        let default_sampler_descriptor = SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge, 
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        };

        let default_sampler = device.create_sampler(&default_sampler_descriptor);
        
        sampler_storage.borrow_mut().insert(SamplerName::DefaultSampler, Rc::new(default_sampler));

        Self { sampler_storage: sampler_storage }
    }
    pub fn get_sampler(&self, sampler_name: SamplerName) -> Option<Rc<Sampler>> {
        self.sampler_storage.borrow().get(&sampler_name).cloned()
    }
}
