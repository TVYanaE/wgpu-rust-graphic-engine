use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};
use wgpu::{
    Device,
    BindGroup, BindGroupLayout, BindGroupDescriptor, BindGroupEntry, BindingResource, TextureView, Sampler
};
use crate::{
    enums::bind_group_name_enum::BindGroupName,
};



pub struct BindGroupManager {
    bind_group_storage: RefCell<HashMap<BindGroupName, Rc<BindGroup>>>
}

impl BindGroupManager {
    pub fn new() -> Self {
        Self { bind_group_storage: RefCell::new(HashMap::new()) }
    }

    pub fn create_bind_group(
        &self,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        texture_view: &TextureView, // It using for texture atlas
        sampler: &Sampler,
        label: Option<&str>,
        bind_group_name: BindGroupName,
    ) {
        let bind_group = device.create_bind_group(
            &BindGroupDescriptor {
                layout: bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(texture_view)
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(sampler)
                    },
                ],
                label: label 
            }
        );

        self.bind_group_storage.borrow_mut().insert(bind_group_name, Rc::new(bind_group));
    }

    pub fn get_bind_group(&self, bind_group_name: BindGroupName) -> Option<Rc<BindGroup>> {
        self.bind_group_storage.borrow().get(&bind_group_name).cloned()
    }
}
