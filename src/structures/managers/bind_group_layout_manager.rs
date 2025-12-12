use std::{
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
};
use wgpu::{
    BindGroupLayout, BindGroupLayoutEntry, ShaderStages, BindingType, TextureSampleType, TextureViewDimension,
    SamplerBindingType, BindGroupLayoutDescriptor,
    Device,
};
use crate::{
    enums::{
        bind_group_layout_name_enum::BindGroupLayoutName,
    }, 
};

const DEFAULT_BIND_GROUP_LAYOUT_DESCRIPTOR_ENTRIES: &[BindGroupLayoutEntry] = &[
    BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::FRAGMENT,
        count: None,
        ty: BindingType::Texture { 
            sample_type: TextureSampleType::Float { filterable: true }, 
            view_dimension: TextureViewDimension::D2, 
            multisampled: false, 
        },
    },
    BindGroupLayoutEntry {
        binding: 1,
        visibility: ShaderStages::FRAGMENT,
        count: None,
        ty: BindingType::Sampler(SamplerBindingType::Filtering)
    },
];

const CAMERA_BIND_GROUP_LAYOUT_DESCRIPTOR_ENTRIES: &[BindGroupLayoutEntry] = &[
    BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        count: None,
        ty: BindingType::Buffer { 
            ty: wgpu::BufferBindingType::Uniform, 
            has_dynamic_offset: false, 
            min_binding_size: None, 
        },
    },
];

pub struct BindGroupLayoutManager {
    bind_group_layout_storage: RefCell<HashMap<BindGroupLayoutName, Rc<BindGroupLayout>>>,
}
        
impl BindGroupLayoutManager {
    pub fn new(device: &Device) -> Self {
        let bind_group_layout_storage= RefCell::new(HashMap::new());
     
        let default_bind_group_layout_descriptor = BindGroupLayoutDescriptor {
            label: Some("default_bind_group_layout_descriptor"),
            entries: DEFAULT_BIND_GROUP_LAYOUT_DESCRIPTOR_ENTRIES,
        };   

        let default_bind_group_layout = device.create_bind_group_layout(&default_bind_group_layout_descriptor);

        bind_group_layout_storage.borrow_mut().insert(
            BindGroupLayoutName::DefaultBindGroupLayout, 
            Rc::new(default_bind_group_layout)
        );

        let camera_bind_group_layout_descriptor = BindGroupLayoutDescriptor {
            label: Some("Camera bind group layout descriptor"),
            entries: CAMERA_BIND_GROUP_LAYOUT_DESCRIPTOR_ENTRIES,
        };

        let camera_bind_group_layout = device.create_bind_group_layout(&camera_bind_group_layout_descriptor);

        bind_group_layout_storage.borrow_mut().insert(
            BindGroupLayoutName::CameraBindGroupLayout, 
            Rc::new(camera_bind_group_layout),
        );

        Self { bind_group_layout_storage: bind_group_layout_storage }    
    }
    pub fn get_bind_group_layout(
        &self, 
        bind_group_layout_name: &BindGroupLayoutName,
    ) -> Option<Rc<BindGroupLayout>> {
        self.bind_group_layout_storage.borrow().get(bind_group_layout_name).cloned()     
    }
}
