use std::{
    cell::RefCell,
    collections::HashMap,
    sync::Arc,
};
use wgpu::{
    Device,
    BindGroup, BindGroupLayout, BindGroupDescriptor, BindGroupEntry, BindingResource, TextureView, Sampler
};


#[derive(Debug, Hash, PartialEq, Eq)]
pub enum BindGroupName {
    TextureAtlas1BindGroup,
    CameraBindGroup,
}

pub struct BindGroupManager {
    bind_group_storage: RefCell<HashMap<BindGroupName, Arc<BindGroup>>>
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

        self.bind_group_storage.borrow_mut().insert(bind_group_name, Arc::new(bind_group));
    }

    pub fn get_bind_group(&self, bind_group_name: BindGroupName) -> Option<Arc<BindGroup>> {
        self.bind_group_storage.borrow().get(&bind_group_name).cloned()
    }
}
