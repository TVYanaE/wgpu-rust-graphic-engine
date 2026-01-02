use wgpu::{
    TextureViewDescriptor, CommandEncoderDescriptor, Operations,
    RenderPassDescriptor, RenderPassColorAttachment,
};
use super::{
    super::{
        states::{
            gpu_state::{
                state::GPUState,
                camera::ViewProjectionUniformMatrixCache,
            },
        },
        render_batch::RenderBatch,
    },
};

pub fn render_phase(
    gpu_state: &GPUState, 
    render_batches_cache: &[RenderBatch],
    view_projection_uniform_matrix_cache: &ViewProjectionUniformMatrixCache,
) {
    let viev_project_uniform_buffer = &gpu_state.camera_storage.camera_uniform_buffer;
    gpu_state.queue.write_buffer(
        viev_project_uniform_buffer, 
        0, 
        bytemuck::cast_slice(&[view_projection_uniform_matrix_cache.view_projection_matrix])
    );
     
    draw_call(gpu_state, render_batches_cache);    
}

fn draw_call(gpu_state: &GPUState, render_batches_cache: &[RenderBatch]) {
        let surface_current_texture = gpu_state.surface.get_current_texture().unwrap();

        let current_texture =  surface_current_texture.texture.clone(); 

        let texture_view_descriptor = TextureViewDescriptor {
            format: Some(gpu_state.surface_texture_format.add_srgb_suffix()),
            ..Default::default()
        };

        let current_texture_view = current_texture.create_view(&texture_view_descriptor);

        let command_encoder_description = CommandEncoderDescriptor::default(); 
        
        let operations = Operations {
            store: wgpu::StoreOp::Store,
            load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
        }; 

        let collor_attachments = RenderPassColorAttachment {
            view: &current_texture_view,
            depth_slice: None,
            resolve_target: None,
            ops: operations, 
        }; 

        let mut command_encoder = gpu_state.device.create_command_encoder(&command_encoder_description); 
        
        let render_pass_description = RenderPassDescriptor {
            label: None,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            color_attachments: &[Some(collor_attachments)],
        };

        let mut render_pass = command_encoder.begin_render_pass(&render_pass_description); 

        render_pass.set_bind_group(0, &gpu_state.camera_storage.camera_bind_group, &[]);

        render_pass.set_viewport(
            0.0,
            0.0,
            gpu_state.surface_configuration.width as f32,
            gpu_state.surface_configuration.height as f32, 
            0.0,
            1.0,
        ); 

        for render_batch in render_batches_cache {
            render_pass.set_pipeline(&render_batch.render_pipeline);
            render_pass.set_bind_group(1, render_batch.bind_group.as_ref(), &[]);
            render_pass.set_vertex_buffer(0, gpu_state.square_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, render_batch.instance_buffer.slice(..));
            render_pass.set_index_buffer(gpu_state.square_index_buffer.slice(..), gpu_state.index_format);
            render_pass.draw_indexed(0..6, 0, 0..render_batch.instance_count);      
        }
        
        drop(render_pass);

        let command_buffer = command_encoder.finish();

        gpu_state.queue.submit([command_buffer]);

        surface_current_texture.present(); 
}
