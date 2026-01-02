use std::{
    collections::{HashMap}
};
use wgpu::{
    Device,
    util::{DeviceExt, BufferInitDescriptor},
};
use crate::{
    modules::{
        shared::{
            render_state::{
                RenderItem
            },
        },
    },
    aliases::MaterialID,
};
use super::{
    render_batch::RenderBatch,
    states::{
        gpu_state::{
            managers::{
                material_manager::MaterialManager,
            },
            buffers_layouts::InstanceVertex,
        },
    },
};
 
pub fn batching(
    device: &Device, 
    render_items: &mut Vec<RenderItem>,
    render_batches_cache: &mut Vec<RenderBatch>,
    material_manager: &MaterialManager,
) {
    if !render_batches_cache.is_empty() {
        render_batches_cache.clear();
    }

    let mut render_groups: HashMap<MaterialID, Vec<RenderItem>> = HashMap::new();  

    for render_item in render_items {
        let key = render_item.material_id;
        render_groups.entry(key).or_default().push(render_item.clone());
    }

    for (material_id, render_items) in render_groups {
        
        let material = material_manager.get_material(material_id).unwrap();
    
        let instances: Vec<InstanceVertex> = render_items
        .iter()
        .map(|render_item| {
            
            InstanceVertex {
                instance_position: render_item.instance_position,
                instance_size: render_item.instance_size,
                uv_offset: material.uv_offset,
                uv_scale: material.uv_scale,
        }
        })
        .collect();
        
        let instance_buffer = device.create_buffer_init(
        &BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        }
        );

        let render_batch = RenderBatch {
            instance_buffer: instance_buffer,
            instance_count: instances.len() as u32,
            render_pipeline: material.render_pipeline.clone(),
            bind_group: material.bind_group.clone(),
        };
        render_batches_cache.push(render_batch); 
        
    }
}


     
