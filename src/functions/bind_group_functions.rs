use wgpu::{
    Device,
    BindGroup, BindGroupLayout, TextureView, Sampler, BindGroupDescriptor, BindGroupEntry, BindingResource
};

pub fn create_bind_group_for_sprite_material(
    device: &Device,
    bind_group_layout: &BindGroupLayout,
    texture_view: &TextureView,
    sampler: &Sampler,
    label: Option<&str>,
) -> BindGroup {
    device.create_bind_group(
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
    )  
}
