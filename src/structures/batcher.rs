use std::{
    collections::HashMap, rc::Rc,
};
use wgpu::{
    Device,
    util::{DeviceExt, BufferInitDescriptor},
};
use crate::{
    structures::{
        render_items::RenderItem,
        render_batch::RenderBatch,
        material::Material,
    },
    buffers_layouts::InstanceVertex,
};



pub struct Batcher {
    render_batch_cache: Vec<RenderBatch>,          
}

impl Batcher {
    pub fn new() -> Self {
        Self { render_batch_cache: Vec::new() }
    }

    pub fn batching(&mut self, render_items: &[RenderItem], device: &Device, ) {
        let mut render_groups: HashMap<*const Material, Vec<RenderItem>> = HashMap::new(); 
       
        if !self.render_batch_cache.is_empty() {
            self.render_batch_cache.clear();
        }

        for render_item in render_items {
            let key = Rc::as_ptr(&render_item.material);
            render_groups.entry(key).or_default().push(render_item.clone());
        }

        for (_key, group) in render_groups {
            let material: Rc<Material> = group[0].material.clone();

            let instances: Vec<InstanceVertex> = group
            .iter()
            .map(|render_item| {
                
                //test 
                /* println!("from batcher uv_offset x = {}", render_item.material.uv_offset[0]);
                println!("from batcher uv_offset y = {}", render_item.material.uv_offset[1]);
                println!("from batcher uv_scale x = {}", render_item.material.uv_scale[0]);
                println!("from batcher uv_scale y = {}", render_item.material.uv_scale[1]); */

                InstanceVertex {
                    instance_position: render_item.instance_position,
                    instance_size: render_item.instance_size,
                    uv_offset: render_item.material.uv_offset,
                    uv_scale: render_item.material.uv_scale,
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

            self.render_batch_cache.push(render_batch); 
        }    
    }

    pub fn get_render_batches(&self) -> &[RenderBatch] {
        &self.render_batch_cache
    }
}
