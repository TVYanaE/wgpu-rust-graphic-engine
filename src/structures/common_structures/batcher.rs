
use wgpu::{
    Device,
    util::{DeviceExt, BufferInitDescriptor},
};
use shipyard::{
    World,
    UniqueViewMut
};
use crate::{
    structures::{
        render_batch::RenderBatch,
        uniques::logical_render_batches_cache::LogicalRenderBatchesCache,
    }, 
};

pub struct Batcher {
    render_batches_cache: Vec<RenderBatch>
}

impl Batcher {
    pub fn new() -> Self {
        Self { render_batches_cache: Vec::new() }
    }
    pub fn get_render_batches(&self) -> &[RenderBatch] {
        &self.render_batches_cache
    }
    
    pub fn batching(&mut self, world: &World, device: &Device) {

        if !self.render_batches_cache.is_empty() {
            self.render_batches_cache.clear();
        }

        let mut logical_render_batches_cache = world
            .borrow::<UniqueViewMut<LogicalRenderBatchesCache>>()
            .unwrap();

        for logical_render_batch in logical_render_batches_cache.logical_render_batches.drain(..) {
            let instance_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&logical_render_batch.instances),
                usage: wgpu::BufferUsages::VERTEX,
            }
            );

            let render_batch = RenderBatch {
                instance_buffer: instance_buffer,
                instance_count: logical_render_batch.instances.len() as u32,
                render_pipeline: logical_render_batch.material.render_pipeline.clone(),
                bind_group: logical_render_batch.material.bind_group.clone(),
            };
            self.render_batches_cache.push(render_batch);
        }
         
    }
}

     
